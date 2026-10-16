use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, LitStr, parse_macro_input};

/// Generates string-preserving Agones enum plumbing.
///
/// Known variants use `#[agones(value = "...")]`; exactly one tuple variant
/// uses `#[agones(unknown)]` and stores unknown source values verbatim. This
/// mirrors Go's named-string types: validation can reject unknown values later
/// without losing them during parsing or round-tripping.
#[proc_macro_derive(AgonesStringEnum, attributes(agones))]
pub fn derive_agones_string_enum(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match expand(input) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.into_compile_error().into(),
    }
}

fn expand(input: DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let name = input.ident;
    let Data::Enum(data) = input.data else {
        return Err(syn::Error::new_spanned(name, "AgonesStringEnum requires an enum"));
    };

    let mut known = Vec::new();
    let mut unknown = None;

    for variant in data.variants {
        let ident = variant.ident;
        let mut value = None;
        let mut is_unknown = false;

        for attr in variant.attrs.iter().filter(|a| a.path().is_ident("agones")) {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("value") {
                    let lit: LitStr = meta.value()?.parse()?;
                    value = Some(lit.value());
                    Ok(())
                } else if meta.path.is_ident("unknown") {
                    is_unknown = true;
                    Ok(())
                } else {
                    Err(meta.error("expected `value = \"...\"` or `unknown`"))
                }
            })?;
        }

        if is_unknown {
            if unknown.is_some() {
                return Err(syn::Error::new_spanned(ident, "only one unknown variant is allowed"));
            }
            match variant.fields {
                Fields::Unnamed(ref fields) if fields.unnamed.len() == 1 => {}
                _ => {
                    return Err(syn::Error::new_spanned(
                        ident,
                        "the unknown variant must contain exactly one String field",
                    ));
                }
            }
            unknown = Some(ident);
        } else {
            if !matches!(variant.fields, Fields::Unit) {
                return Err(syn::Error::new_spanned(ident, "known variants must be unit variants"));
            }
            let value = value.unwrap_or_else(|| ident.to_string());
            known.push((ident, value));
        }
    }

    let unknown = unknown.ok_or_else(|| {
        syn::Error::new_spanned(&name, "an #[agones(unknown)] variant is required")
    })?;

    let known_idents: Vec<_> = known.iter().map(|(ident, _)| ident).collect();
    let known_values: Vec<_> = known
        .iter()
        .map(|(_, value)| LitStr::new(value, proc_macro2::Span::call_site()))
        .collect();

    Ok(quote! {
        impl #name {
            pub const fn known_values() -> &'static [&'static str] {
                &[#(#known_values),*]
            }

            pub fn as_str(&self) -> &str {
                match self {
                    #(Self::#known_idents => #known_values,)*
                    Self::#unknown(value) => value.as_str(),
                }
            }
        }

        impl ::core::fmt::Display for #name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.write_str(self.as_str())
            }
        }

        impl ::core::convert::AsRef<str> for #name {
            fn as_ref(&self) -> &str {
                self.as_str()
            }
        }

        impl ::core::convert::From<&str> for #name {
            fn from(value: &str) -> Self {
                Self::from(value.to_owned())
            }
        }

        impl ::core::convert::From<String> for #name {
            fn from(value: String) -> Self {
                match value.as_str() {
                    #(#known_values => Self::#known_idents,)*
                    _ => Self::#unknown(value),
                }
            }
        }

        impl ::serde::Serialize for #name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                serializer.serialize_str(self.as_str())
            }
        }

        impl<'de> ::serde::Deserialize<'de> for #name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                let value = <String as ::serde::Deserialize>::deserialize(deserializer)?;
                Ok(Self::from(value))
            }
        }
    })
}
