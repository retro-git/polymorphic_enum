use proc_macro::TokenStream;
use syn::{FnArg, Token, punctuated::Punctuated};

/*
    Automatically generate a struct for each variant of an enum. Convert the enum such that each variant holds an instance of its corresponding struct.
    Implement a given trait on the newly generated enum. For each trait method, match on the enum variant and call the method on the underlying struct.
    Implement From and Into for each enum variant/struct pair.
    Generate a declarative macro that is named the lowercase of the enum name. It is the same as vec!, but automatically calls .into() on each element.
*/
#[proc_macro]
pub fn polymorphic_enum(input: TokenStream) -> TokenStream {
    // This functional-style macro expects a trait definition, followed by an enum definition. Parse them.
    let input = syn::parse_macro_input!(input as syn::File);

    let (trait_item, trait_attrs) = match &input.items[0] {
        syn::Item::Trait(trait_item) => (trait_item, &trait_item.attrs),
        _ => panic!("Expected a trait definition as the first item."),
    };

    // The second item is the enum definition. Parse it along with any attributes it may have.
    let (enum_item, enum_attrs) = match &input.items[1] {
        syn::Item::Enum(enum_item) => (enum_item, &enum_item.attrs),
        _ => panic!("Expected an enum definition as the second item."),
    };

    // Map each enum variant to a struct with the same name, containing the variant's fields. If the variant's fields are named, the struct's fields are named the same. If the variant's fields are unnamed, the struct's fields unnamed.
    let structs = enum_item.variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        // let variant_attrs = &variant.attrs;
        let fields = match &variant.fields {
            syn::Fields::Named(fields) => {
                let fields = fields.named.iter().map(|field| {
                    let field_name = &field.ident;
                    let field_type = &field.ty;
                    quote::quote! {
                        #field_name: #field_type
                    }
                });
                quote::quote! {
                    #(#fields),*
                }
            },
            syn::Fields::Unnamed(fields) => {
                let fields = fields.unnamed.iter().enumerate().map(|(_, field)| {
                    let field_type = &field.ty;
                    quote::quote! {
                        #field_type
                    }
                });
                quote::quote! {
                    #(#fields),*
                }
            },
            syn::Fields::Unit => quote::quote! {},
        };
        match &variant.fields {
            syn::Fields::Named(_) => quote::quote! {
                #(#enum_attrs)*
                struct #variant_name {
                    #fields
                }
            },
            syn::Fields::Unnamed(_) => quote::quote! {
                #(#enum_attrs)*
                struct #variant_name (
                    #fields
                );
            },
            syn::Fields::Unit => quote::quote! {
                #(#enum_attrs)*
                struct #variant_name;
            },
        }
    });

    // Map each enum variant to a variant with a single unnamed field, the struct with the same name.
    let variants = enum_item.variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        quote::quote! {
            #variant_name(#variant_name)
        }
    });

    // Get the identifier only of each enum variant.
    let variant_idents = enum_item.variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        quote::quote! {
            #variant_name
        }
    });

    let enum_name = &enum_item.ident;

    // Implement the trait for the enum. For each trait method, match on the enum variant and call the method on the underlying struct.
    let trait_methods = trait_item.items.iter().map(|item| {
        let variant_idents = variant_idents.clone();
        match item {
            syn::TraitItem::Fn(method) => {
                let method_name = &method.sig.ident;
                let method_inputs = &method.sig.inputs;
                let method_attrs = &method.attrs;

                // Remove the self parameter from the method inputs.
                let method_inputs_self_and_types_removed = method_inputs.clone().into_iter().filter_map(|input| {
                    match input {
                        syn::FnArg::Receiver(_) => None,
                        syn::FnArg::Typed(pat_type) => Some(syn::FnArg::Typed(pat_type.clone())),
                    }
                }).collect::<Punctuated<FnArg, Token![,]>>();
                // Get the identifier only of each method input, removing the type.
                let method_inputs_self_and_types_removed: Punctuated<syn::Ident, Token![,]> = method_inputs_self_and_types_removed.clone().into_iter().map(|input| {
                    match input {
                        syn::FnArg::Typed(pat_type) => {
                            match *pat_type.pat {
                                syn::Pat::Ident(pat_ident) => pat_ident.ident,
                                _ => panic!("Expected a Pat::Ident."),
                            }
                        },
                        _ => panic!("Expected a FnArg::Typed."),
                    }
                }).collect::<Punctuated<syn::Ident, Token![,]>>();

                let method_output = &method.sig.output;
                let method_body = match method_output {
                    syn::ReturnType::Default => quote::quote! {
                        match self {
                            #(#enum_name::#variant_idents(inner) => inner.#method_name(#method_inputs_self_and_types_removed),)*
                        }
                    },
                    syn::ReturnType::Type(_, _) => quote::quote! {
                        match self {
                            #(#enum_name::#variant_idents(inner) => inner.#method_name(#method_inputs_self_and_types_removed),)*
                        }
                    },
                };
                quote::quote! {
                    #(#method_attrs)*
                    fn #method_name(#method_inputs) #method_output {
                        #method_body
                    }
                }
            },
            _ => panic!("Expected a trait method."),
        }
    });

    // Implement From and Into for each enum variant/struct pair.
    let from_impls = enum_item.variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        let variant_name_lower = syn::Ident::new(&variant_name.to_string().to_lowercase(), variant_name.span());
        let struct_name = &variant.ident;
        quote::quote! {
            impl From<#struct_name> for #enum_name {
                fn from(#variant_name_lower: #struct_name) -> Self {
                    #enum_name::#variant_name(#variant_name_lower)
                }
            }
        }
    });

    let into_impls = enum_item.variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        // Make variant_name all lowercase.
        let variant_name_lower = syn::Ident::new(&variant_name.to_string().to_lowercase(), variant_name.span());
        let struct_name = &variant.ident;
        quote::quote! {
            impl Into<#struct_name> for #enum_name {
                fn into(self) -> #struct_name {
                    match self {
                        #enum_name::#variant_name(#variant_name_lower) => #variant_name_lower,
                        _ => panic!("Tried to convert a {} into a {} but the enum variant was not {}", stringify!(#enum_name), stringify!(#struct_name), stringify!(#variant_name)),
                    }
                }
            }
        }
    });

    // Generate a declarative macro that is named the lowercase of the enum name. It is the same as vec!, but automatically calls .into() on each element.
    let declarative_macro_name = syn::Ident::new(&enum_name.to_string().to_lowercase(), enum_name.span());
    let declarative_macro = quote::quote! {
        macro_rules! #declarative_macro_name {
            ($($x:expr),*) => (vec![$($x.into()),*]);
        }
    };

    let trait_name = &trait_item.ident;

    // Generate the output.
    let output = quote::quote! {
        #(#trait_attrs)*
        #trait_item

        #(#structs)*

        #(#enum_attrs)*
        enum #enum_name {
            #(#variants),*
        }

        impl #trait_name for #enum_name {
            #(#trait_methods)*
        }

        #(#from_impls)*
        #(#into_impls)*

        #declarative_macro
    };

    output.into()
}