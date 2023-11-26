extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Fields};
use syn::spanned::Spanned;

#[proc_macro_derive(StateMutGen)]
pub fn state_mut_gen_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let methods = if let syn::Data::Struct(data) = &input.data {
        match &data.fields {
            Fields::Named(fields) => fields.named.iter().map(|f| {
                let field_name = &f.ident;
                let field_type = &f.ty;

                let method_name = syn::Ident::new(&format!("smgen_{}_set", field_name.as_ref().unwrap()), field_name.span());
                quote! {
                    fn #method_name(arg: #field_type) -> crate::game::state::mutation::StateMutator {
                        // Insert boilerplate code here
                    }
                }
            }).collect(),

            _ => Vec::new()
        }
    } else {
        Vec::new()
    };

    let expanded = quote! {
        impl #name {
            #(#methods)*
        }
    };

    dbg!(expanded.to_string());


    TokenStream::from(expanded)
}

