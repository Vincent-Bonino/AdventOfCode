use proc_macro2::{Ident, Span};
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{Expr, ExprLit, Fields, FieldsUnnamed, Lit, Meta, MetaNameValue, Variant};

use super::{char_range::CharRange, tile_chars::TileChars};
use crate::utils::extract_type_ident;

static CHAR_ATTRIBUTE_NAME: &str = "chr";
static RANGE_ATTRIBUTE_NAME: &str = "range";

static VALID_ATTRIBUTES: &[&str] = &["tile", CHAR_ATTRIBUTE_NAME, RANGE_ATTRIBUTE_NAME];

#[derive(Clone, Copy, Debug, Default)]
pub struct AocTileEnumVariant {
    // TryFrom<chr>
    pub tile_chars: Option<TileChars>,
    has_value: bool,
}

impl AocTileEnumVariant {
    pub fn from_meta_name_value(meta_name_value: &MetaNameValue) -> Result<Self, syn::Error> {
        let mut result: Self = Self::default();
        result.handle_meta_name_value(meta_name_value)?;
        Ok(result)
    }

    pub fn get_char(&self) -> Option<char> {
        if let Some(TileChars::Unique(chr)) = self.tile_chars {
            Some(chr)
        } else {
            None
        }
    }

    pub fn get_match_function(&self) -> Option<Ident> {
        if let Some(TileChars::Range(range)) = self.tile_chars {
            Some(range.get_match_function())
        } else {
            None
        }
    }

    pub fn is_char(&self) -> bool {
        matches!(self.tile_chars, Some(TileChars::Unique(_)))
    }

    pub fn is_range(&self) -> bool {
        matches!(self.tile_chars, Some(TileChars::Range(_)))
    }

    // Logic

    pub fn can_match_char(&self) -> bool {
        self.tile_chars.is_some()
    }

    /// Merge with another AocTileEnumVariants.
    fn merge(&mut self, other: AocTileEnumVariant) -> Result<(), ()> {
        if let Some(tile_chars) = other.tile_chars {
            self.set_tile_chars(tile_chars)?;
        }
        self.has_value |= other.has_value;

        Ok(())
    }

    #[rustfmt::skip]
    pub fn verify(&self) -> Result<(), String> {
        if !self.can_match_char() {
            return Err("Variant must be associated to a char or a range of char.".to_string())
        }

        if let Some(TileChars::Range(_)) = self.tile_chars && !self.has_value {
            // Re-checking for good measures
            return Err(format!(
                "Variant with the {RANGE_ATTRIBUTE_NAME} attribute must have a `char` field."
            ));
        }

        // All is good
        Ok(())
    }

    // Setters

    pub fn set_tile_chars(&mut self, tile_chars: TileChars) -> Result<(), ()> {
        match self.tile_chars {
            Some(_) => Err(()),
            None => {
                self.tile_chars = Some(tile_chars);
                Ok(())
            }
        }
    }

    // Parsing utils

    fn handle_meta_name_value(&mut self, meta_name_value: &MetaNameValue) -> Result<(), syn::Error> {
        // Handle cases where the attribute defines an already define property.
        if meta_name_value.path.is_ident(CHAR_ATTRIBUTE_NAME) {
            let value: char = Self::get_char_from_mnv_value(&meta_name_value.value)?;

            self.set_tile_chars(TileChars::Unique(value)).map_err(|_err| {
                syn::Error::new(
                    meta_name_value.value.span(),
                    format!(
                        "Value for {} is already set to '{:?}'",
                        CHAR_ATTRIBUTE_NAME, self.tile_chars
                    ),
                )
            })?
        } else if meta_name_value.path.is_ident(RANGE_ATTRIBUTE_NAME) {
            let value: String = Self::get_string_from_mnv_value(&meta_name_value.value)?;
            let range: CharRange =
                CharRange::try_from(value).map_err(|err| syn::Error::new(meta_name_value.value.span(), err))?;

            self.set_tile_chars(TileChars::Range(range)).map_err(|_err| {
                syn::Error::new(
                    meta_name_value.value.span(),
                    format!(
                        "Value for {} is already set to '{:?}'",
                        RANGE_ATTRIBUTE_NAME, self.tile_chars
                    ),
                )
            })?
        } else {
            // Other macros could use other attributes, do not consider it an error.
            // Handle cases where the attribute is not defined by the macro.
            // return Err(syn::Error::new(
            //     meta_name_value.path.span(),
            //     format!(
            //         "Unknown simple attribute '{}'",
            //         meta_name_value.path.to_token_stream(),
            //     ),
            // ));
        }
        Ok(())
    }

    #[expect(unused)]
    fn get_bool_from_mnv_value(expr: &Expr) -> Result<bool, syn::Error> {
        match expr {
            // Only bool literals are accepted
            Expr::Lit(ExprLit {
                lit: Lit::Bool(bool_value),
                ..
            }) => Ok(bool_value.value()),

            // Every other case is an error
            _ => Err(syn::Error::new(
                expr.span(),
                format!("Value must be a bool literal, not '{}'", expr.to_token_stream()),
            )),
        }
    }

    fn get_char_from_mnv_value(expr: &Expr) -> Result<char, syn::Error> {
        match expr {
            // Only char literals are accepted
            Expr::Lit(ExprLit {
                lit: Lit::Char(char_value),
                ..
            }) => Ok(char_value.value()),

            // Every other case is an error
            _ => Err(syn::Error::new(
                expr.span(),
                format!("Value must be a char literal, not '{}'", expr.to_token_stream()),
            )),
        }
    }

    fn get_string_from_mnv_value(expr: &Expr) -> Result<String, syn::Error> {
        match expr {
            // Only string literals are accepted
            Expr::Lit(ExprLit {
                lit: Lit::Str(string_value),
                ..
            }) => Ok(string_value.value()),

            // Every other case is an error
            _ => Err(syn::Error::new(
                expr.span(),
                format!("Value must be a string literal, not '{}'", expr.to_token_stream()),
            )),
        }
    }
}

impl Parse for AocTileEnumVariant {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut result: Self = Self::default();

        let parsed = Punctuated::<MetaNameValue, syn::Token![,]>::parse_terminated(input)?;
        for key_value in parsed {
            result.handle_meta_name_value(&key_value)?;
        }

        Ok(result)
    }
}

/// Parse an enum variant's attributes.
pub fn _parse_variant_attributes(variant: &Variant) -> Result<AocTileEnumVariant, syn::Error> {
    let mut result: AocTileEnumVariant = AocTileEnumVariant::default();

    // Parse fields
    if let Fields::Unnamed(FieldsUnnamed { unnamed, .. }) = &variant.fields {
        let fields_types: Vec<Option<&Ident>> = unnamed.iter().map(|f| extract_type_ident(&f.ty).ok()).collect();

        let mut has_invalid_fields: bool = false;
        let mut span: Span = variant.span();

        for field_type in &fields_types {
            if let Some(field_type) = field_type
                && field_type != &"char"
            {
                span = field_type.span();
                break;
            }
        }

        if let Some(Some(field_type)) = fields_types.first()
            && field_type != &"char"
        {
            has_invalid_fields = true;
        }
        if fields_types.len() != 1 {
            has_invalid_fields = true;
        }

        if has_invalid_fields {
            return Err(syn::Error::new(
                span,
                format!(
                    "Variants with the {} attribute must have a `char` field and no other.",
                    RANGE_ATTRIBUTE_NAME,
                ),
            ));
        }

        result.has_value = true;
    } else {
        result.has_value = false;
    }

    // Parse attributes
    for attr in &variant.attrs {
        match &attr.meta {
            // Attribute looks like `list(arg1, arg2)`, look for known key/value pairs
            Meta::List(list) => {
                result.merge(attr.parse_args()?).map_err(|_err| {
                    syn::Error::new(
                        list.span(),
                        format!("Invalid instruction, must be one of {VALID_ATTRIBUTES:?}"),
                    )
                })?;
            }

            // Attribute looks like `name = value`
            Meta::NameValue(name_value) => {
                result
                    .merge(AocTileEnumVariant::from_meta_name_value(name_value)?)
                    .map_err(|_err| {
                        syn::Error::new(
                            name_value.span(),
                            format!("Invalid instruction, must be one of {VALID_ATTRIBUTES:?}"),
                        )
                    })?;
            }

            // Attribute looks like `#[path]`, nothing to do with it
            Meta::Path(_path) => {
                // However, other macros could use their own attributes.
                // This is not an error, just ignore.
            }
        }
    }

    Ok(result)
}
