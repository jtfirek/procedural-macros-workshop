use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use syn::{parse_macro_input, DeriveInput};
use quote::quote;

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    // parse the input into a token tree
    let input_tree = parse_macro_input!(input as DeriveInput);

    // extract the identifier of the input struct
    let name = input_tree.ident;

    // creating name + "Builder" 
    let name_builder = Ident::new(&format!("{}Builder", name), Span::call_site());

    let result = quote!{
        
        pub struct #name_builder {
            executable: Option<String>,
            args: Option<Vec<String>>,
            env: Option<Vec<String>>,
            current_dir: Option<String>,
        }

        impl #name {
            pub fn builder() -> #name_builder {
                #name_builder {
                    executable: None,
                    args: None,
                    env: None,
                    current_dir: None,
                }
            }
        }
    };

    return TokenStream::from(result)
}
