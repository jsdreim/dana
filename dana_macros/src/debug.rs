use proc_macro::TokenStream;
use quote::ToTokens;


#[allow(dead_code)]
pub struct CallDebug {
    span: proc_macro2::Span,
    text: String,
    line: usize,
    column: usize,
}

impl CallDebug {
    pub fn new(macro_name: &str, stream: &TokenStream) -> Self {
        let span = proc_macro2::Span::mixed_site();
        let text = span.source_text()
            .unwrap_or_else(|| format!("{macro_name}!({stream})"));

        let mut text = text.replacen("!", "!\x1B[37;2m", 1);
        text.insert_str(0, "\x1B[94m");
        text.push_str("\x1B[m");

        let proc_macro2::LineColumn { line, column } = span.start();

        Self { span, text, line, column }
    }

    pub fn auto() -> Option<Self> {
        let span = proc_macro2::Span::mixed_site();
        let text = span.source_text()?;

        let proc_macro2::LineColumn { line, column } = span.start();

        Some(Self { span, text, line, column })
    }

    #[allow(unexpected_cfgs)]
    pub fn print(&self, output: impl ToTokens) {
        use std::io::Write;

        let text_out = format!("{}", output.to_token_stream());
        let pipe_out = &mut std::io::stderr().lock();

        #[cfg(procmacro2_semver_exempt)]
        write!(
            pipe_out,
            "[{}:{}:{}] ",
            self.span.source_file().path().display(),
            self.line,
            self.column,
        ).unwrap();

        writeln!(
            pipe_out,
            "{} = {}",
            self.text,
            text_out,
        ).unwrap();
    }
}


macro_rules! macro_dbg {
    { as macro $name:literal for $stream:ident $(;)? if $db:ident; $($t:tt)* } => {{
        let debug = $crate::debug::CallDebug::new($name, &$stream);
        let value = { $($t)* };
        if value.$db { debug.print(&value); }
        value
    }};
    { as macro $name:literal for $stream:ident; $($t:tt)* } => {{
        let debug = $crate::debug::CallDebug::new($name, &$stream);
        let value = { $($t)* };
        debug.print(&value);
        value
    }};
    { $($t:tt)* } => { $($t)* };
}


macro_rules! wrap_dbg {
    ($inner:ident $(::  $method:ident)? as $vis:vis $wrap:ident) => {
        wrap_dbg!($inner $(:: $method)? as $vis     $wrap { debug: if ? });
    };
    ($inner:ident $(::  $method:ident)? as $vis:vis $wrap:ident {
        $debug:ident: if $sigil:tt$(,)?
    }) => {
        #[allow(dead_code)]
        $vis struct $wrap {
            pub $debug: bool,
            pub inner: $inner,
        }

        impl ::syn::parse::Parse for $wrap {
            fn parse(input: ::syn::parse::ParseStream) -> ::syn::Result<Self> {
                Ok(Self {
                    $debug: input.parse::<::syn::Token![$sigil]>().is_ok(),
                    inner: input.parse()?,
                })
            }
        }

        impl ::quote::ToTokens for $wrap {
            fn to_tokens(&self, tokens: &mut ::proc_macro2::TokenStream) {
                self.inner$(.$method())?.to_tokens(tokens);
            }
        }
    };
}
