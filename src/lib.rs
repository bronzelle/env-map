use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(EnvMap)]
pub fn env_map_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let main_struct = ast.ident;
    let const_value = Ident::new(
        &format!("{}", &main_struct).to_uppercase(),
        main_struct.span(),
    );
    let builder = format_ident!("{}Builder", &main_struct);
    let fields = if let Data::Struct(data_struct) = &ast.data {
        match &data_struct.fields {
            Fields::Named(fields) => &fields.named,
            Fields::Unnamed(fields) => &fields.unnamed,
            Fields::Unit => panic!("Unit structs are not supported"),
        }
    } else {
        panic!("ReplicateStruct is only implemented for structs");
    };
    let fields_clone = fields.clone();
    let fields_iter = fields_clone.iter().filter_map(|f| f.ident.as_ref());
    let fields_into_iter = fields.iter();
    let gen = quote! {
        use serde::Deserialize;
        use std::{env, sync::OnceLock};

        #[derive(Deserialize, Clone)]
        struct #builder {
            #(#fields_into_iter),*
        }

        impl Default for #builder {
            fn default() -> Self {
                let dotenv = dotenvy::dotenv();
                match envy::from_env::<#builder>() {
                    Ok(builder) => builder,
                    Err(_) => panic!("missing env vars"),
                }
            }
        }

        impl #builder {
            pub fn build(self) -> #main_struct {
                #main_struct {
                    #(#fields_iter: self.#fields_iter.clone(),)*
                }
            }
        }

        impl Default for #main_struct {
            fn default() -> Self {
                let builder = #builder::default();
                builder.build()
            }
        }

        impl #main_struct {
            pub fn get_config() -> OnceLock<#main_struct> {
                // #const_value.get_or_init(#main_struct::default)
                #const_value
            }
        }

        const #const_value: OnceLock<#main_struct> = OnceLock::new();
    };
    gen.into()
}
