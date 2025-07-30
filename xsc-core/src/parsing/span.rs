use chumsky::span::SimpleSpan;

pub type Span = SimpleSpan<usize>;

pub type Spanned<T> = (T, Span);
