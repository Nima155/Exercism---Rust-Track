extern crate proc_macro;

use proc_macro::TokenStream; // allows us to read and manipulate rust code from our code
use quote::quote; // allows us to turn syn data strctures back into rust code
use syn; // allows us to turn rust code from a string into a data strcuture.. like a tree.

#[proc_macro_derive(Attrs)]
pub fn with_attrs_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    let name = &ast.ident;

    let gen = quote! {
        impl Attrs for #name {
            fn with_attrs(self, attrs: &[(&str, &str)]) -> Self {
                Self {
                    attrs: 
                        attrs
                            .iter()
                            .map(|(one, two)| (one.to_string(), two.to_string()))
                            .collect::<HashMap<String, String>>(),
                    
                    ..self
                }
            } 
        }
    };
    gen.into()

}
