// //INPUT EXAMPLE:
// polymorphic_enum!(trait Move {
//     fn execute(&mut self);
//     fn valid_for_state(&self, state: u8) -> bool;
// }

// enum Moves {
//     Attack { card_id: u32, attack_power: u32, name: String },
//     Defend,
// })

// //OUTPUT EXAMPLE:
// struct Attack {
//     card_id: u32,
//     attack_power: u32,
//     name: String,
// }

// struct Defend;

// enum Moves {
//     Attack(Attack),
//     Defend(Defend),
// }

// impl Move for Moves {
//     fn execute(&self) {
//         match self {
//             Moves::Attack(inner) => inner.execute(),
//             Moves::Defend(inner) => inner.execute(),
//         }
//     }

//     fn valid_for_state(&self, state: u8) -> bool {
//         match self {
//             Moves::Attack(inner) => inner.valid_for_state(state),
//             Moves::Defend(inner) => inner.valid_for_state(state),
//         }
//     }
// }

use proc_macro::TokenStream;
use syn::{FnArg, Token, punctuated::Punctuated};

#[proc_macro]
pub fn polymorphic_enum(input: TokenStream) -> TokenStream {
    // This functional-style macro expects a trait definition, followed by an enum definition. Parse them.
    let input = syn::parse_macro_input!(input as syn::File);
    let trait_item = match &input.items[0] {
        syn::Item::Trait(trait_item) => trait_item,
        _ => panic!("Expected a trait definition as the first item."),
    };

    // The second item is the enum definition.
    let enum_item = match &input.items[1] {
        syn::Item::Enum(enum_item) => enum_item,
        _ => panic!("Expected an enum definition as the second item."),
    };

    // Map each enum variant to a struct with the same name, containing the variant's fields. If the variant's fields are named, the struct's fields are named the same. If the variant's fields are unnamed, the struct's fields unnamed.
    let structs = enum_item.variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        let mut named = false;
        let fields = match &variant.fields {
            syn::Fields::Named(fields) => {
                named = true;
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
        match named {
            true => quote::quote! {
                struct #variant_name {
                    #fields
                }
            },
            false => quote::quote! {
                struct #variant_name (
                    #fields
                );
            }
        }
    });

    // Map each enum variant to a variant with a single unnamed field, the struct with the same name.
    let variants = enum_item.variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        quote::quote! {
            #variant_name(#variant_name)
        }
    });

    // Get the identifier of each enum variant only.
    let variant_idents = enum_item.variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        quote::quote! {
            #variant_name
        }
    });

    dbg!(&enum_item.ident);

    let enum_name = &enum_item.ident;

    // Implement the trait for the enum. For each trait method, match on the enum variant and call the method on the underlying struct.
    let trait_methods = trait_item.items.iter().map(|item| {
        let variant_idents = variant_idents.clone();
        match item {
            syn::TraitItem::Fn(method) => {
                let method_name = &method.sig.ident;
                let method_inputs = &method.sig.inputs;
                let method_inputs_self_removed = method_inputs.clone().into_iter().filter_map(|input| {
                    match input {
                        syn::FnArg::Receiver(_) => None,
                        syn::FnArg::Typed(pat_type) => Some(syn::FnArg::Typed(pat_type.clone())),
                    }
                }).collect::<Punctuated<FnArg, Token![,]>>();
                // Turn method_inputs_self_removed into Punctuated<
                let method_inputs_self_removed: Punctuated<syn::Ident, Token![,]> = method_inputs_self_removed.clone().into_iter().map(|input| {
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
                            #(#enum_name::#variant_idents(inner) => inner.#method_name(#method_inputs_self_removed),)*
                        }
                    },
                    syn::ReturnType::Type(_, _) => quote::quote! {
                        match self {
                            #(#enum_name::#variant_idents(inner) => inner.#method_name(#method_inputs_self_removed),)*
                        }
                    },
                };
                quote::quote! {
                    fn #method_name(#method_inputs) #method_output {
                        #method_body
                    }
                }
            },
            _ => panic!("Expected a trait method."),
        }
    });

    let trait_name = &trait_item.ident;

    let output = quote::quote! {
        #trait_item
        #(#structs)*

        enum #enum_name {
            #(#variants),*
        }

        impl #trait_name for #enum_name {
            #(#trait_methods)*
        }
    };

    output.into()
}