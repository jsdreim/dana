#![allow(dead_code)]

use proc_macro::TokenStream;
use quote::ToTokens;


struct CallDebug {
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

    pub fn print(&self, output: impl ToTokens) {
        let text_out = format!("{}", output.to_token_stream());

        #[cfg(procmacro2_semver_exempt)]
        eprintln!(
            "[{}:{}:{}] {} = {}",
            self.span.source_file().path().display(),
            self.line,
            self.column,
            self.text,
            text_out,
        );

        #[cfg(not(procmacro2_semver_exempt))]
        eprintln!(
            "{} = {}",
            self.text,
            text_out,
        );
    }
}
