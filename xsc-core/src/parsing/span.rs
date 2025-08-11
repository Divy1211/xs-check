use chumsky::span::SimpleSpan;

pub type Span = SimpleSpan<usize>;

pub type Spanned<T> = (T, Span);

pub fn contains(span1: &Span, span2: &Span) -> bool {
    span1.start <= span2.start && span2.end <= span1.end
}