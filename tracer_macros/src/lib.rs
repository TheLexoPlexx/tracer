use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{DeriveInput, Ident, parse_macro_input};

#[proc_macro_derive(Create)]
pub fn derive_create(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let struct_name = Ident::new(&format!("JsonCreate{name}"), Span::call_site());

    let expanded = quote! {

        #[derive(Debug, Serialize, Deserialize)]
        pub struct #struct_name {
            action: String,
            data: #name,
        }

        impl #name {
            pub fn create(&self) -> Result<(), String> {
                // Implementierung der Create-Operation
                todo!();
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(Read)]
pub fn derive_read(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let jsonread_struct = Ident::new(&format!("JsonRead{name}"), Span::call_site());

    let fields = match &input.data {
        syn::Data::Struct(syn::DataStruct { fields, .. }) => fields,
        _ => panic!("Read derive macro only works on structs"),
    };

    let field_names: Vec<_> = fields.iter().map(|f| f.ident.as_ref().unwrap()).collect();

    let expanded = quote! {
        #[derive(Debug, Serialize, Deserialize)]
        pub struct #jsonread_struct {
            action: String,
            id: String,
        }

        impl #name {
            pub fn read(id: String) -> Result<Self, String> {
                let mut result = Self {
                    #(#field_names: Default::default(),)*
                };
                result.id = id;

                todo!();
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(Update)]
pub fn derive_update(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let builder_name = Ident::new(&format!("{}UpdateBuilder", name), Span::call_site());

    let fields = match &input.data {
        syn::Data::Struct(syn::DataStruct { fields, .. }) => fields,
        _ => panic!("Update derive macro only works on structs"),
    };

    let field_names: Vec<_> = fields.iter().map(|f| f.ident.as_ref().unwrap()).collect();

    let field_types: Vec<_> = fields.iter().map(|f| &f.ty).collect();

    let builder_fields = quote! {
        #(
            pub #field_names: Option<#field_types>,
        )*
    };

    let builder_methods = quote! {
        #(
            pub fn #field_names(mut self, value: #field_types) -> Self {
                self.#field_names = Some(value);
                self
            }
        )*
    };

    let expanded = quote! {
        pub struct #builder_name {
            #builder_fields
        }

        impl #name {
            pub fn update(&self) -> #builder_name {
                #builder_name {
                    #(
                        #field_names: Some(self.#field_names.clone()),
                    )*
                }
            }
        }

        impl #builder_name {
            #builder_methods

            pub async fn save(self) -> Result<(), String> {
                // Hier kommt die eigentliche Update-Logik
                // Nur die Some-Felder werden aktualisiert
                todo!()
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(Delete)]
pub fn derive_delete(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let jsonread_struct = Ident::new(&format!("JsonDelete{name}"), Span::call_site());

    let expanded = quote! {
        #[derive(Debug, Serialize, Deserialize)]
        pub struct #jsonread_struct {
            action: String,
            id: String,
        }

        impl #name {
            pub fn delete(&self) -> Result<(), String> {
                // Implementierung der Delete-Operation
                todo!();
            }
        }
    };

    TokenStream::from(expanded)
}
