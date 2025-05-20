use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{DeriveInput, Ident, parse_macro_input};

#[proc_macro_derive(Create)]
pub fn derive_create(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let name_str = name.to_string().to_lowercase();
    let create_fn = Ident::new(&format!("create_{}", name_str), Span::call_site());
    let create_api = Ident::new(&format!("CreateApi{}", name), Span::call_site());
    let path = format!("/{}/create", name_str);

    let expanded = quote! {
        impl #name {
            pub fn create(&self) -> Result<(), String> {
                // Implementierung der Create-Operation
                todo!();
            }

            pub fn create_api() -> #create_api {
                #create_api
            }
        }

        pub struct #create_api;

        #[OpenApi]
        impl #create_api {
            #[oai(path = #path, method = "post")]
            async fn #create_fn(&self, data: Json<#name>) -> Json<#name> {
                let result = #name::create(&data.0);
                todo!()
                // Json(#name {
                //     action: "create".to_string(),
                //     data: data.0,
                // })
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(Read)]
pub fn derive_read(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let name_str = name.to_string().to_lowercase();
    let read_fn = Ident::new(&format!("read_{}", name_str), Span::call_site());
    let read_api = Ident::new(&format!("ReadApi{}", name), Span::call_site());
    let path = format!("/{}/read/{{id}}", name_str);

    let fields = match &input.data {
        syn::Data::Struct(syn::DataStruct { fields, .. }) => fields,
        _ => panic!("Read derive macro only works on structs"),
    };

    let field_names: Vec<_> = fields.iter().map(|f| f.ident.as_ref().unwrap()).collect();

    let expanded = quote! {

        impl #name {
            pub fn read(id: String) -> Result<Self, String> {
                let mut result = Self {
                    #(#field_names: Default::default(),)*
                };
                result.id = id;

                todo!();
            }

            pub fn read_api() -> #read_api {
                #read_api
            }
        }

        pub struct #read_api;

        #[OpenApi]
        impl #read_api {
            #[oai(path = #path, method = "get")]
            async fn #read_fn(&self, Path(id): Path<String>) -> Json<#name> {
                let result = #name::read(id);
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
    let name_str = name.to_string().to_lowercase();
    let builder_name = Ident::new(&format!("{}UpdateBuilder", name), Span::call_site());
    let update_fn = Ident::new(&format!("update_{}", name_str), Span::call_site());
    let update_api = Ident::new(&format!("UpdateApi{}", name), Span::call_site());
    let path = format!("/{}/update/{{id}}", name_str);

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

            pub fn update_api() -> #update_api {
                #update_api
            }
        }

        impl #builder_name {
            #builder_methods

            pub async fn save(self) -> Result<(), String> {
                // Hier kommt die eigentliche Update-Logik
                // Nur die Some-Felder werden aktualisiert
                todo!();
            }
        }

        pub struct #update_api;

        #[OpenApi]
        impl #update_api {
            #[oai(path = #path, method = "put")]
            async fn #update_fn(&self, Path(id): Path<String>, data: Json<#name>) -> Json<#name> {
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
    let name_str = name.to_string().to_lowercase();
    let delete_fn = Ident::new(&format!("delete_{name_str}"), Span::call_site());
    let delete_api = Ident::new(&format!("DeleteApi{name}"), Span::call_site());
    let path = format!("/{name_str}/delete/{{id}}");

    let expanded = quote! {
        impl #name {
            pub fn delete(id_to_delete: String) -> Result<(), String> {
                // Implementierung der Delete-Operation, sollte jetzt id_to_delete verwenden
                todo!();
            }

            pub fn delete_api() -> #delete_api {
                #delete_api
            }
        }

        pub struct #delete_api;

        #[OpenApi]
        impl #delete_api {
            #[oai(path = #path, method = "delete")]
            async fn #delete_fn(&self, Path(id): Path<String>) -> Json<#name> {
                let result = #name::delete(id);
                todo!();
            }
        }
    };

    TokenStream::from(expanded)
}

// #[proc_macro_derive(Api)]
// pub fn derive_api(input: TokenStream) -> TokenStream {
//     let input = parse_macro_input!(input as DeriveInput);
//     let name = &input.ident;
//     let name_str = name.to_string().to_lowercase();
//     let api = Ident::new(&format!("{}Api", name), Span::call_site());

//     let expanded = quote! {
//         pub struct #api;

//         #[OpenApi]
//         impl #api {
//             #[oai(path = #path, method = "get")]
//             async fn #read_fn(&self) -> Json<#name> {
//                 todo!()
//             }
//         }
//     };

//     TokenStream::from(expanded)
// }
