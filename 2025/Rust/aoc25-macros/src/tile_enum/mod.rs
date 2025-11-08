pub mod char_range;
mod tile_chars;
pub mod variant;

use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{Data, DeriveInput, parse_macro_input};

use variant::{_parse_variant_attributes, AocTileEnumVariant};

pub(super) fn aoc_tile_enum(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the input tokens into a syntax tree
    let input: DeriveInput = parse_macro_input!(input);

    // Hand the output tokens back to the compiler
    _aoc_tile_enum(input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

fn _aoc_tile_enum(input: DeriveInput) -> Result<TokenStream, syn::Error> {
    // Only work on enum
    if let Data::Enum(data_enum) = input.data {
        let name: Ident = input.ident;
        let mut variants: Vec<(&Ident, AocTileEnumVariant)> = Vec::with_capacity(data_enum.variants.len());

        for variant in &data_enum.variants {
            variants.push((&variant.ident, _parse_variant_attributes(variant)?));
        }

        // -- Test for inconsistencies --

        let mut errors: Vec<String> = Vec::new();

        // Verify parsed variants
        for (var_name, var_var) in variants.iter() {
            if let Err(error_message) = var_var.verify() {
                errors.push(format!("{var_name}: {error_message}"))
            }
        }

        // [TryFrom<char>] No char must map to more than one variant
        if errors.is_empty() {
            for i in 0..variants.len() {
                let (l_ident, l_var): &(&Ident, AocTileEnumVariant) = variants.get(i).unwrap();
                for j in (i + 1)..variants.len() {
                    let (r_ident, r_var): &(&Ident, AocTileEnumVariant) = variants.get(j).unwrap();

                    if l_var.tile_chars.unwrap().has_overlap_with(&r_var.tile_chars.unwrap()) {
                        errors.push(format!("Variants {l_ident} and {r_ident} overlap."))
                    }
                }
            }
        }

        // Finally
        if !errors.is_empty() {
            return Err(syn::Error::new(Span::call_site(), errors.join(" ")));
        }

        // -- Generate code --

        let _all_var_idents: Vec<&Ident> = variants.iter().map(|&(ident, _)| ident).collect();

        // Build vec for variants that can be built from a single char
        let var_ident_from_char: Vec<&Ident> = variants
            .iter()
            .filter_map(|&(ident, var)| if var.is_char() { Some(ident) } else { None })
            .collect();
        let var_char_from_char: Vec<char> = variants.iter().filter_map(|&(_, var)| var.get_char()).collect();

        // Build vec for variants that can be built from a range of char
        let var_ident_from_range: Vec<&Ident> = variants
            .iter()
            .filter_map(|&(ident, var)| if var.is_range() { Some(ident) } else { None })
            .collect();
        let var_func_from_range: Vec<Ident> = variants
            .iter()
            .filter_map(|&(_, var)| var.get_match_function())
            .collect();

        // Display impl block
        let display_impl: TokenStream = quote! {
            impl Display for #name {
                fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                    match self {
                        #( Self::#var_ident_from_char => f.write_char(#var_char_from_char), )*
                        #( Self::#var_ident_from_range(value) => f.write_char(*value), )*
                    }
                }
            }
        };

        // TryFrom<char> impl block
        let try_from_char_impl: TokenStream = quote! {
            impl TryFrom<char> for #name {
                type Error = String;
                fn try_from(value: char) -> Result<Self, Self::Error> {
                    match value {
                        #( #var_char_from_char => Ok(Self::#var_ident_from_char), )*
                        #( value if value.#var_func_from_range() => Ok(Self::#var_ident_from_range(value)), )*
                        _ => Err(format!("Unable to transform '{value}' to {}", stringify!(#name))),
                    }
                }
            }
        };

        // Gather pieces together
        Ok(quote! {
            use ::std::fmt::{Display, Formatter, Write};

            #display_impl
            #try_from_char_impl
        })
    } else {
        // Raise a compile error if applied on something that is not an enum
        Err(syn::Error::new(Span::call_site(), "Derive macro only works on enums"))
    }
}
