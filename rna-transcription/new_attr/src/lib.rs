use syn::{self, DeriveInput, Expr, Lit}; // to parse into an AST
use quote::quote; // to turn back into a tokenStream
use proc_macro::TokenStream; 

#[proc_macro_attribute]
pub fn with_attrs(attrs: TokenStream,input: TokenStream) -> TokenStream {
    

    let expr = syn::parse::<Expr>(attrs).unwrap(); 

    let mut strand: String = String::new();

    match expr {
        Expr::Lit(s) => match s.lit {
            Lit::Str(z) => strand = z.value(), // get the DNA/RNA sequence 
            _ => {}
        },
        _ => {}
    }

    let name = &syn::parse_macro_input!(input as DeriveInput).ident; // get the name of the struct..

    let ret = quote! {
        #[derive(Debug, PartialEq)]
        pub struct #name(String);
        impl #name {
            pub fn new(dna: &str) -> Result<#name, usize> {
                let res = dna.chars().enumerate().try_for_each(|(i, v)| {
                    if let false = #strand.contains(v) {
                        return Err(i);
                    }
                    Ok(())
                });
                if let Err(s) = res {
                    Err(s)
                } else {
                    Ok(Self(dna.to_string()))
                }
            }
        }
    };

    ret.into()
}