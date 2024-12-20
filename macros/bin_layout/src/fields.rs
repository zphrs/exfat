use proc_macro2::TokenStream;
use quote::{quote, quote_spanned, ToTokens};
use syn::{Expr, Fields, Ident, Meta, Type};

struct FieldMetadata {
    ident: Ident,
    min: Option<Expr>,
    max: Option<Expr>,
}

impl FieldMetadata {
    fn new(ident: Ident, min: Option<Expr>, max: Option<Expr>) -> Self {
        Self { ident, min, max }
    }
    fn serialize_stream(&self) -> TokenStream {
        let Self {
            ident, min, max, ..
        } = &self;
        let str_ident = ident.to_string();
        let min = min.clone().map_or(quote! {}, |min| {
            quote! {
                let min = #min;
                if self.#ident < min {
                    return Err(BoundError::too_small(
                        #str_ident.clone(),
                        self.#ident,
                        min,
                    ));
                }
            }
        });
        let max = max.clone().map_or(quote! {}, |max| {
            quote! {
                let max = #max;
                if self.#ident > max {
                    return Err(BoundError::too_big(
                        #str_ident.clone(),
                        self.#ident,
                        max,
                    ));
                }
            }
        });
        quote! {
            {
            #min
            #max
            }
        }
    }
    fn first_num(value: &Expr) -> usize {
        let self_tokens = value.to_token_stream();
        // get first number in token stream
        let self_num = self_tokens.into_iter().next().unwrap();
        self_num.to_string().parse().unwrap()
    }
}

pub fn get_bounding_code(fields: Fields) -> TokenStream {
    let mut field_metadata = Vec::new();
    for field in fields {
        let attrs = field.attrs;
        let ident = field.ident.expect("Every field must have an identity.");
        let [min, max] = ["min", "max"].map(|constraint| {
            let attr = attrs.iter().find_map(|attr| {
                let Meta::List(ref meta_list) = attr.meta else {
                    return None;
                };
                if meta_list.path.is_ident(constraint) {
                    return Some(meta_list);
                }
                None
            });

            attr.map(|a| {
                let expr: Expr = a.parse_args().unwrap();
                expr
            })
        });
        field_metadata.push(FieldMetadata::new(ident, min, max));
    }
    let streams = field_metadata.iter().map(FieldMetadata::serialize_stream);

    quote! {
        #(#streams)*
    }
}
