use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{DeriveInput, Ident, parse_macro_input};

#[proc_macro_derive(CRUD)]
pub fn derive_crud(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let name_str = name.to_string().to_lowercase();
    let api_name = Ident::new(&format!("{}Api", name), Span::call_site());

    let create_fn = Ident::new(&format!("create_{}", name_str), Span::call_site());
    let create_path = format!("/{}/create", name_str);

    let read_fn = Ident::new(&format!("read_{}", name_str), Span::call_site());
    let read_path = format!("/{}/read/{{id}}", name_str);

    let builder_name = Ident::new(&format!("{}UpdateBuilder", name), Span::call_site());
    let update_fn = Ident::new(&format!("update_{}", name_str), Span::call_site());
    let update_path = format!("/{}/update/{{id}}", name_str);

    let delete_fn = Ident::new(&format!("delete_{}", name_str), Span::call_site());
    let delete_path = format!("/{}/delete/{{id}}", name_str);

    let fields = match &input.data {
        syn::Data::Struct(syn::DataStruct { fields, .. }) => fields,
        _ => panic!("Read derive macro only works on structs"),
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
        impl #name {
            pub fn create(&self) -> Result<(), String> {
                // Implementierung der Create-Operation
                todo!();
            }

            pub fn read(id: String) -> Result<Self, String> {
                let mut result = Self {
                    #(#field_names: Default::default(),)*
                };
                result.id = id;

                todo!();
            }

            pub fn delete(id_to_delete: String) -> Result<(), String> {
                todo!();
            }

            pub fn api() -> #api_name {
                #api_name
            }
        }

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
                todo!();
            }
        }

        pub struct #api_name;

        #[OpenApi]
        impl #api_name {
            #[oai(path = #create_path, method = "post")]
            async fn #create_fn(&self, data: Json<#name>) -> Json<#name> {
                let result = #name::create(&data.0);
                todo!()
                // Json(#name {
                //     action: "create".to_string(),
                //     data: data.0,
                // })
            }

            #[oai(path = #read_path, method = "get")]
            async fn #read_fn(&self, Path(id): Path<String>) -> Json<#name> {
                let result = #name::read(id);
                todo!();
            }

            #[oai(path = #update_path, method = "put")]
            async fn #update_fn(&self, Path(id): Path<String>, data: Json<#name>) -> Json<#name> {
                todo!()
            }

            #[oai(path = #delete_path, method = "delete")]
            async fn #delete_fn(&self, Path(id): Path<String>) -> Json<#name> {
                let result = #name::delete(id);
                todo!();
            }
        }
    };

    TokenStream::from(expanded)
}
