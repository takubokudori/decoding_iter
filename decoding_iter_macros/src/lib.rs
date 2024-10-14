use std::{path::Path, str::FromStr};
use syn::punctuated::Punctuated;
use syn::{
    parse_macro_input, Expr, ExprLit, ExprTuple, ItemConst, Lit, LitInt, LitStr,
};

/// Validates the code points in an array at compile-time.
///
/// ```raw
/// compile_time_check_code_point!(
/// let INDEX_JIS0208: &[u16] = &[0x0001, 0x0002, 0x0003];
/// )
/// ```
#[proc_macro]
pub fn validate_code_points(
    ast: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let e_const = parse_macro_input!(ast as ItemConst);
    let arr_ident = &e_const.ident;
    let expr = e_const.expr.as_ref();
    match expr {
        Expr::Reference(expr) => match expr.expr.as_ref() {
            Expr::Array(expr) => {
                for (i, elem) in expr.elems.iter().enumerate() {
                    match elem {
                        Expr::Lit(lit) => match &lit.lit {
                            Lit::Int(token) => {
                                let code_point: u16 =
                                    token.base10_parse().unwrap();
                                if !is_valid_code_point(code_point) {
                                    panic!("code point {arr_ident}[{i}] ({code_point:#06x}) is invalid.");
                                }
                            }
                            _ => panic!("Invalid code point literal"),
                        },
                        _ => panic!("Invalid array element"),
                    }
                }
            }
            _ => panic!("Invalid expr"),
        },
        _ => panic!("Invalid expr"),
    }

    let func = create_fn_get_from_index(arr_ident);
    let q = quote::quote! { #e_const #func };
    q.into()
}

fn to_ident(s: impl AsRef<str>) -> syn::Ident {
    syn::Ident::new(s.as_ref(), proc_macro2::Span::call_site())
}

fn is_valid_code_point(code_point: impl Into<u32>) -> bool {
    core::char::from_u32(code_point.into()).is_some()
}

fn create_fn_get_from_index(
    arr_ident: &syn::Ident,
) -> proc_macro2::TokenStream {
    let arr_name_lower = arr_ident.to_string().to_ascii_lowercase();
    let fn_get_char_from = format!("get_char_from_{arr_name_lower}");
    let fn_get_char_from = to_ident(fn_get_char_from);
    let fn_get_code_point_from =
        format!("get_code_point_from_{arr_name_lower}");
    let fn_get_code_point_from = to_ident(fn_get_code_point_from);
    quote::quote! {
    #[inline]
    fn #fn_get_code_point_from(pointer: u32) -> Option<u32> {
        let code_point = *#arr_ident.get(pointer as usize)?;
        if code_point == 0 {
            None
        } else {
            Some(code_point as u32)
        }
    }

    #[inline]
    fn #fn_get_char_from(pointer: u32) -> Option<char> {
        // SAFETY: The code points in #arr_name are validated at compile-time.
        #fn_get_code_point_from(pointer).map(|code_point| unsafe {core::char::from_u32_unchecked(code_point)} )
    }
        }
}

/// Creates a code point array and functions from an index file.
/// index-jis0208.txt => `const INDEX_JIS0208: &[u8] = &[ ... ];`
#[proc_macro]
pub fn array_from_index_file(
    ast: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let path = parse_macro_input!(ast as LitStr).value();
    let arr_name = path.rsplit_once('/').unwrap().1;
    let arr_name = arr_name.rsplit_once('.').unwrap().0;
    let arr_name = arr_name.replace('-', "_").to_ascii_uppercase();
    let path = Path::new(&path);
    index_file_to_array(path, arr_name)
}

/// Creates a code point array and functions from an index file.
/// index-jis0208.txt => `const INDEX_JIS0208: &[u8] = &[ ... ];`
#[proc_macro]
pub fn array_from_index_file_u32(
    ast: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let path = parse_macro_input!(ast as LitStr).value();
    let arr_name = path.rsplit_once('/').unwrap().1;
    let arr_name = arr_name.rsplit_once('.').unwrap().0;
    let arr_name = arr_name.replace('-', "_").to_ascii_uppercase();
    let path = Path::new(&path);
    index_file_to_array_u32(path, arr_name)
}

#[proc_macro]
pub fn array_from_index_ranges_file(
    ast: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let path = parse_macro_input!(ast as LitStr).value();
    let arr_name = path.rsplit_once('/').unwrap().1;
    let arr_name = arr_name.rsplit_once('.').unwrap().0;
    let arr_name = arr_name.replace('-', "_").to_ascii_uppercase();
    let path = Path::new(&path);
    index_file_to_array_ranges(path, arr_name)
}

fn index_file_to_array(
    path: impl AsRef<Path>,
    arr_name: impl AsRef<str>,
) -> proc_macro::TokenStream {
    let arr_name = arr_name.as_ref();
    let arr_ident = to_ident(arr_name);
    let path = path.as_ref();
    let f = std::fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("Failed to read {}: {e}", path.display()));

    let cp_indexes: &mut [u16] = &mut [0; 65535];
    let mut max_idx = 0;
    for (i, l) in f.lines().enumerate() {
        if l.is_empty() || l.starts_with('#') {
            // skip empty lines and comment lines.
            continue;
        }
        let mut it = l.splitn(3, '\x09');
        let idx = it.next().unwrap().trim();
        let idx = usize::from_str(idx).unwrap();
        if idx > max_idx {
            max_idx = idx;
        }
        let code_point = it.next().unwrap().strip_prefix("0x").unwrap();
        let code_point = u16::from_str_radix(code_point, 16).unwrap();
        assert_ne!(code_point, 0, "code_point in an index file must not be 0.");
        if !is_valid_code_point(code_point) {
            panic!("code_point {code_point:#06x} at line {i} is invalid.");
        }
        cp_indexes[idx] = code_point;
    }
    let lits: Vec<LitInt> = cp_indexes
        .iter()
        .copied()
        .take(max_idx + 1)
        .map(|cp| {
            LitInt::new(&format!("{cp:#06x}"), proc_macro2::Span::call_site())
        })
        .collect();
    let func = create_fn_get_from_index(&arr_ident);
    let func = quote::quote! { #func };

    let arr_index = quote::quote! {
        const #arr_ident: &[u16] = &[
            #(#lits),*
        ];
    };
    // buf
    let q = quote::quote! {
        #arr_index

        #func
    };
    q.into()
}

fn index_file_to_array_u32(
    path: impl AsRef<Path>,
    arr_name: impl AsRef<str>,
) -> proc_macro::TokenStream {
    let arr_name = arr_name.as_ref();
    let arr_ident = to_ident(arr_name);
    let path = path.as_ref();
    let f = std::fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("Failed to read {}: {e}", path.display()));

    let cp_indexes: &mut [u32] = &mut [0; 65535];
    let mut max_idx = 0;
    for (i, l) in f.lines().enumerate() {
        if l.is_empty() || l.starts_with('#') {
            // skip empty lines and comment lines.
            continue;
        }
        let mut it = l.splitn(3, '\x09');
        let idx = it.next().unwrap().trim();
        let idx = usize::from_str(idx).unwrap();
        if idx > max_idx {
            max_idx = idx;
        }
        let code_point = it.next().unwrap().strip_prefix("0x").unwrap();
        let code_point = u32::from_str_radix(code_point, 16).unwrap();
        assert_ne!(code_point, 0, "code_point in an index file must not be 0.");
        if !is_valid_code_point(code_point) {
            panic!("code_point {code_point:#06x} at line {i} is invalid.");
        }
        cp_indexes[idx] = code_point;
    }
    let lits: Vec<LitInt> = cp_indexes
        .iter()
        .copied()
        .take(max_idx + 1)
        .map(|cp| {
            LitInt::new(&format!("{cp:#06x}"), proc_macro2::Span::call_site())
        })
        .collect();
    let func = create_fn_get_from_index(&arr_ident);
    let func = quote::quote! { #func };

    let arr_index = quote::quote! {
        const #arr_ident: &[u32] = &[
            #(#lits),*
        ];
    };
    // buf
    let q = quote::quote! {
        #arr_index

        #func
    };
    q.into()
}

fn index_file_to_array_ranges(
    path: impl AsRef<Path>,
    arr_name: impl AsRef<str>,
) -> proc_macro::TokenStream {
    let arr_name = arr_name.as_ref();
    let arr_ident = to_ident(arr_name);
    let path = path.as_ref();
    let f = std::fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("Failed to read {}: {e}", path.display()));

    let mut cp_indexes: Vec<(usize, u32)> = Vec::with_capacity(1024);
    let mut before_idx = 0;

    for (i, l) in f.lines().enumerate() {
        if l.is_empty() || l.starts_with('#') {
            // skip empty lines and comment lines.
            continue;
        }
        let mut it = l.splitn(3, '\x09');
        let idx = it.next().unwrap().trim();
        let idx = usize::from_str(idx).unwrap();
        assert!(idx >= before_idx, "{idx} > {before_idx} not satisfied");
        before_idx = idx;
        let code_point = it.next().unwrap().strip_prefix("0x").unwrap();
        let code_point = u32::from_str_radix(code_point, 16).unwrap();
        assert_ne!(code_point, 0, "code_point in an index file must not be 0.");
        if !is_valid_code_point(code_point) {
            panic!("code_point {code_point:#06x} at line {i} is invalid.");
        }
        cp_indexes.push((idx, code_point));
    }
    let mut lits: Vec<ExprTuple> = Vec::with_capacity(65535);
    for (idx, cp) in cp_indexes.iter() {
        if *cp == 0 {
            break;
        }
        let mut elems = Punctuated::new();
        let lit: Lit =
            LitInt::new(&format!("{idx:#06x}"), proc_macro2::Span::call_site())
                .into();
        let l = ExprLit { attrs: vec![], lit };
        elems.push(Expr::Lit(l));
        let lit: Lit =
            LitInt::new(&format!("{cp:#06x}"), proc_macro2::Span::call_site())
                .into();
        let l = ExprLit { attrs: vec![], lit };
        elems.push(Expr::Lit(l));
        lits.push(ExprTuple {
            attrs: vec![],
            paren_token: Default::default(),
            elems,
        });
    }

    let q = quote::quote! {
        const #arr_ident: &[(u32,u32)] = &[
            #(#lits),*
        ];
    };
    q.into()
}
