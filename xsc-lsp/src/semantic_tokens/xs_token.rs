use xsc_core::parsing::span::Span;

#[derive(Debug)]
pub struct XsToken {
    pub span: Span,
    pub type_: u32,
    pub modifiers: u32,
}

impl XsToken {
    pub fn from(span: &Span, type_: u32, modifiers: u32) -> XsToken {
        Self { span: span.clone(), type_, modifiers }
    }
}