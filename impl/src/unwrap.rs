use crate::utils::{AttrParams, DeriveType, State};
use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{DeriveInput, Fields, Result};

pub fn expand(input: &DeriveInput, trait_name: &'static str) -> Result<TokenStream> {
    let state = State::with_attr_params(
        input,
        trait_name,
        quote! {},
        "unwrap".into(),
        AttrParams {
            enum_: vec!["ignore"],
            variant: vec!["ignore"],
            struct_: vec!["ignore"],
            field: vec!["ignore"],
        },
    )?;
    assert!(
        state.derive_type == DeriveType::Enum,
        "Unwrap can only be derived for enums",
    );

    let enum_name = &input.ident;
    let (imp_generics, type_generics, where_clause) = input.generics.split_for_impl();

    let mut funcs = vec![];
    for variant_state in state.enabled_variant_data().variant_states {
        let variant = variant_state.variant.unwrap();
        let fn_name = format_ident!(
            "unwrap_{}",
            variant.ident.to_string().to_case(Case::Snake),
            span = variant.ident.span(),
        );
        let variant_ident = &variant.ident;

        let (data_pattern, ret_value, ret_type) = match variant.fields {
            Fields::Named(_) => panic!("cannot unwrap anonymous records"),
            Fields::Unnamed(ref fields) => {
                let data_pattern =
                    (0..fields.unnamed.len()).fold(vec![], |mut a, n| {
                        a.push(format_ident!("field_{n}"));
                        a
                    });
                let ret_type = &fields.unnamed;
                (
                    quote! { (#(#data_pattern),*) },
                    quote! { (#(#data_pattern),*) },
                    quote! { (#ret_type) },
                )
            }
            Fields::Unit => (quote! {}, quote! { () }, quote! { () }),
        };

        let other_arms = state.variant_states.iter().map(|variant| {
            variant.variant.unwrap()
        }).filter(|variant| {
            &variant.ident != variant_ident
        }).map(|variant| {
            let data_pattern = match variant.fields {
                Fields::Named(_) => quote! { {..} },
                Fields::Unnamed(_) => quote! { (..) },
                Fields::Unit => quote! {},
            };
            let variant_ident = &variant.ident;
            quote! { #enum_name :: #variant_ident #data_pattern =>
                      panic!(concat!("called `", stringify!(#enum_name), "::", stringify!(#fn_name),
                                     "()` on a `", stringify!(#variant_ident), "` value"))
            }
        });

        let variant_name = stringify!(variant_ident);
        let func = quote! {
            #[track_caller]
            #[doc = "Unwraps this value to the `"]
            #[doc = #variant_name]
            #[doc = "` variant\n\nPanics if this value is of any other type"]
            pub fn #fn_name(self) -> #ret_type {
                match self {
                    #enum_name ::#variant_ident #data_pattern => #ret_value,
                    #(#other_arms),*
                }
            }
        };
        funcs.push(func);
    }

    let imp = quote! {
        #[automatically_derived]
        impl #imp_generics #enum_name #type_generics #where_clause {
            #(#funcs)*
        }
    };

    Ok(imp)
}
