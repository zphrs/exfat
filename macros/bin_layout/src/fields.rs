use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{Expr, Fields, Ident, Meta, Type};

struct FieldMetadata {
    ident: Ident,
    size: usize,
    offset: usize,
}

impl FieldMetadata {
    fn new(ident: Ident, size: &Expr, offset: &Expr) -> Self {
        Self {
            ident,
            size: Self::first_num(size),
            offset: Self::first_num(offset),
        }
    }
    fn serialize_stream(&self) -> TokenStream {
        let Self {
            ident,
            size,
            offset,
            ..
        } = &self;
        let end = offset + size;
        quote! {
            self.#ident = input[#offset;#end]
        }
    }
    fn first_num(value: &Expr) -> usize {
        let self_tokens = value.to_token_stream();
        // get first number in token stream
        let self_num = self_tokens.into_iter().next().unwrap();
        self_num.to_string().parse().unwrap()
    }
}

impl PartialEq for FieldMetadata {
    fn eq(&self, other: &Self) -> bool {
        self.offset == other.offset
    }
}

impl Eq for FieldMetadata {}

/// Orders fields based on offset
impl PartialOrd for FieldMetadata {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.offset.partial_cmp(&other.offset)
    }
}

impl Ord for FieldMetadata {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.offset.cmp(&other.offset)
    }
}

pub fn get_serde(fields: Fields) -> (TokenStream, TokenStream) {
    let mut field_metadata = Vec::new();
    for field in fields {
        let attrs = field.attrs;
        let ty = field.ty;
        let ident = field.ident.expect("Every field must have an identity.");
        let Type::Array(arr_type) = ty else {
            panic!("All fields should be arrays");
        };
        let Type::Path(v_type) = *arr_type.elem else {
            panic!("should be path type");
        };
        println!("{:?}", v_type.path.get_ident());
        if !v_type.path.is_ident("u8") {
            panic!("only supports u8 arrays");
        }
        let offset_attr = attrs.iter().find(|attr| {
            let Meta::NameValue(ref nv) = attr.meta else {
                return false;
            };
            nv.path.is_ident("offset")
        });
        let Some(offset_attr) = offset_attr else {
            panic!("Every field requires a specified offset.");
        };
        let Meta::NameValue(ref nv) = offset_attr.meta else {
            unreachable!();
        };
        let offset: &Expr = &nv.value;
        let size = arr_type.len;
        field_metadata.push(FieldMetadata::new(ident, &size, offset));
    }
    field_metadata.sort();
    (
        quote! {
            ()*
        },
        quote! {},
    )
}
