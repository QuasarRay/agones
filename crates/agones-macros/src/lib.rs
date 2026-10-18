use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Fields, FnArg, ItemFn, LitStr, ReturnType, Type, parse_macro_input};

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


/// Generates paired Bolero and Kani checks from one pure typed invariant.
///
/// The annotated function must:
/// - be a non-async, non-generic free function,
/// - accept exactly one owned input argument,
/// - return `bool`.
///
/// Bolero explores the structured input using `TypeGenerator`; Kani uses a
/// symbolic value of the same type. Keeping the invariant body single-source
/// prevents the fuzz/property test and proof harness from drifting apart.
#[proc_macro_attribute]
pub fn agones_invariant(attr: TokenStream, item: TokenStream) -> TokenStream {
    if !attr.is_empty() {
        return syn::Error::new(
            proc_macro2::Span::call_site(),
            "agones_invariant does not accept arguments",
        )
        .into_compile_error()
        .into();
    }

    let function = parse_macro_input!(item as ItemFn);
    match expand_invariant(function) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.into_compile_error().into(),
    }
}

fn expand_invariant(function: ItemFn) -> syn::Result<proc_macro2::TokenStream> {
    if function.sig.asyncness.is_some() {
        return Err(syn::Error::new_spanned(
            &function.sig,
            "agones_invariant requires a synchronous function",
        ));
    }
    if !function.sig.generics.params.is_empty() {
        return Err(syn::Error::new_spanned(
            &function.sig.generics,
            "agones_invariant does not support generic functions",
        ));
    }
    if function.sig.inputs.len() != 1 {
        return Err(syn::Error::new_spanned(
            &function.sig.inputs,
            "agones_invariant requires exactly one input",
        ));
    }

    let input_type = match function.sig.inputs.first().expect("length checked") {
        FnArg::Typed(argument) => argument.ty.as_ref().clone(),
        FnArg::Receiver(receiver) => {
            return Err(syn::Error::new_spanned(
                receiver,
                "agones_invariant can only be used on free functions",
            ));
        }
    };

    let returns_bool = match &function.sig.output {
        ReturnType::Type(_, ty) => matches!(
            ty.as_ref(),
            Type::Path(path)
                if path.qself.is_none()
                    && path.path.segments.len() == 1
                    && path.path.segments[0].ident == "bool"
        ),
        ReturnType::Default => false,
    };
    if !returns_bool {
        return Err(syn::Error::new_spanned(
            &function.sig.output,
            "agones_invariant requires a bool return type",
        ));
    }

    let name = &function.sig.ident;
    let bolero_name = format_ident!("{name}_bolero");
    let kani_name = format_ident!("{name}_kani");

    Ok(quote! {
        #function

        #[cfg(test)]
        #[test]
        fn #bolero_name() {
            bolero::check!()
                .with_type::<#input_type>()
                .cloned()
                .for_each(|input| {
                    assert!(#name(input));
                });
        }

        #[cfg(kani)]
        #[kani::proof]
        fn #kani_name() {
            let input: #input_type = kani::any();
            assert!(#name(input));
        }
    })
}


/// Generates static API identity metadata for a stored Agones resource.
///
/// Usage:
/// ```ignore
/// #[derive(agones_macros::AgonesApiResource)]
/// #[agones_resource(
///     group = "agones.dev",
///     version = "v1",
///     kind = "GameServer",
///     plural = "gameservers"
/// )]
/// struct GameServer { /* ... */ }
/// ```
#[proc_macro_derive(AgonesApiResource, attributes(agones_resource))]
pub fn derive_agones_api_resource(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match expand_api_resource(input) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.into_compile_error().into(),
    }
}

fn expand_api_resource(input: DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let mut group = None::<String>;
    let mut version = None::<String>;
    let mut kind = None::<String>;
    let mut plural = None::<String>;
    let mut core_path = None::<syn::Path>;
    let mut attributes = 0usize;

    for attr in input
        .attrs
        .iter()
        .filter(|attr| attr.path().is_ident("agones_resource"))
    {
        attributes += 1;
        if attributes > 1 {
            return Err(syn::Error::new_spanned(
                attr,
                "AgonesApiResource accepts exactly one #[agones_resource(...)] attribute",
            ));
        }

        attr.parse_nested_meta(|meta| {
            let value: LitStr = meta.value()?.parse()?;
            if meta.path.is_ident("core_path") {
                let path = value.parse::<syn::Path>()
                    .map_err(|_| meta.error("core_path must be a valid Rust path"))?;
                if core_path.replace(path).is_some() {
                    return Err(meta.error("duplicate core_path"));
                }
                return Ok(());
            }
            let value = value.value();

            if meta.path.is_ident("group") {
                if group.replace(value).is_some() {
                    return Err(meta.error("duplicate group"));
                }
            } else if meta.path.is_ident("version") {
                if version.replace(value).is_some() {
                    return Err(meta.error("duplicate version"));
                }
            } else if meta.path.is_ident("kind") {
                if kind.replace(value).is_some() {
                    return Err(meta.error("duplicate kind"));
                }
            } else if meta.path.is_ident("plural") {
                if plural.replace(value).is_some() {
                    return Err(meta.error("duplicate plural"));
                }
            } else {
                return Err(meta.error("expected group, version, kind, plural, or core_path"));
            }
            Ok(())
        })?;
    }

    if attributes == 0 {
        return Err(syn::Error::new_spanned(
            &name,
            "AgonesApiResource requires #[agones_resource(...)]",
        ));
    }

    let group = group.ok_or_else(|| syn::Error::new_spanned(&name, "missing group"))?;
    let version = version.ok_or_else(|| syn::Error::new_spanned(&name, "missing version"))?;
    let kind = kind.ok_or_else(|| syn::Error::new_spanned(&name, "missing kind"))?;
    let plural = plural.ok_or_else(|| syn::Error::new_spanned(&name, "missing plural"))?;
    let core_path = match core_path {
        Some(path) => quote!(#path),
        None => match proc_macro_crate::crate_name("agones-core") {
            Ok(proc_macro_crate::FoundCrate::Itself) => quote!(::agones_core),
            Ok(proc_macro_crate::FoundCrate::Name(name)) => {
                let name = syn::Ident::new_raw(&name, proc_macro2::Span::call_site());
                quote!(::#name)
            }
            Err(_) => return Err(syn::Error::new_spanned(
                &name,
                "AgonesApiResource requires an agones-core dependency or core_path",
            )),
        },
    };
    let api_version = format!("{group}/{version}");

    let group = LitStr::new(&group, proc_macro2::Span::call_site());
    let version = LitStr::new(&version, proc_macro2::Span::call_site());
    let api_version = LitStr::new(&api_version, proc_macro2::Span::call_site());
    let kind = LitStr::new(&kind, proc_macro2::Span::call_site());
    let plural = LitStr::new(&plural, proc_macro2::Span::call_site());

    Ok(quote! {
        impl #impl_generics #core_path::AgonesApiResource for #name #ty_generics #where_clause {
            const GROUP: &'static str = #group;
            const VERSION: &'static str = #version;
            const API_VERSION: &'static str = #api_version;
            const KIND: &'static str = #kind;
            const PLURAL: &'static str = #plural;
        }
    })
}
