use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use std::num::ParseIntError;
use syn::{DeriveInput, parse_macro_input};

pub(super) fn aoc_day_struct(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the input tokens into a syntax tree
    let input: DeriveInput = parse_macro_input!(input);

    // Hand the output tokens back to the compiler
    _aoc_day_struct(input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

fn _aoc_day_struct(input: DeriveInput) -> Result<TokenStream, syn::Error> {
    let name: Ident = input.ident;
    let name_str: String = name.to_string();

    // Name must end with two digits.
    let day_number: Result<usize, ParseIntError> = {
        // Compute the byte position just before the two last characters.
        let split_position: usize = name_str.char_indices().nth_back(1).unwrap().0;
        name_str[split_position..].parse()
    };

    if let Ok(day_nbr) = day_number {
        Ok(quote! {
            use crate::solution::Aoc25Day;
            impl Aoc25Day for #name {
                fn get_day_number(&self) -> usize {
                    #day_nbr
                }
            }
        })
    } else {
        // Raise a compile error if unable to extract a number from the identifier
        Err(syn::Error::new(
            Span::call_site(),
            format!("Unable to determine the day number from '{name}'"),
        ))?
    }
}
