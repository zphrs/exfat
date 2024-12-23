mod fields;
extern crate proc_macro;

use fields::get_bounding_code;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

#[proc_macro_derive(DiskLayout, attributes(min, max, size))]
pub fn derive_disk_layout(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    impl_derive_disk_layout(&input)
}

fn impl_derive_disk_layout(ast: &syn::DeriveInput) -> TokenStream {
    let DeriveInput { data, ident, .. } = ast;
    let Data::Struct(s) = data.clone() else {
        panic!("expected struct")
    };
    let bounding_code = get_bounding_code(s.fields);
    let q = quote! {
        impl #ident {
            pub fn verify_bounds(&self) -> Result<(), BoundError> {
                #bounding_code
                Ok(())
            }
        }
    };
    q.into()
}
