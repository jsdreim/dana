use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    Result,
    Token,
};


pub struct ImplScale {
    unit_type: syn::Type,
    unit_base: syn::Ident,
    idents: Vec<syn::Ident>,
}

impl ToTokens for ImplScale {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let utype = &self.unit_type;
        let ubase = &self.unit_base;

        for ident in &self.idents {
            let id_str = ident.to_string();
            let id_trait = syn::Ident::new(&format!("Si{id_str}"), ident.span());
            let id_const = syn::Ident::new(&id_str.to_uppercase(), ident.span());
            let unit_var = syn::Ident::new(&format!("{id_str}{ubase}"), ubase.span());

            tokens.extend(quote! {
                impl ::dimensional::units::si::#id_trait for #utype {
                    const #id_const: Self = Self::#unit_var;
                }
            });
        }
    }
}


pub struct MacroScale {
    impls: Vec<ImplScale>,
}

impl Parse for MacroScale {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut impls = Vec::new();

        while input.parse::<Token![for]>().is_ok() {
            let unit_type = input.parse()?;
            input.parse::<Token![impl]>()?;

            let inner;
            parenthesized!(inner in input);

            let mut idents = Vec::new();

            while let Ok(ident) = inner.parse() {
                idents.push(ident);

                if inner.parse::<Token![,]>().is_err() {
                    break;
                }
            }

            let unit_base = input.parse()?;

            impls.push(ImplScale { unit_type, unit_base, idents });

            if input.parse::<Token![;]>().is_err() {
                break;
            }
        }

        Ok(Self { impls })
    }
}

impl ToTokens for MacroScale {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        for impl_scale in &self.impls {
            impl_scale.to_tokens(tokens);
        }
    }
}
