use ropey::Rope;
use tower_lsp::lsp_types::Position;
use xsc_core::parsing::span::Span;

pub fn pos_from_span(src: &Rope, span: &Span) -> (Position, Position) {
    let start_line = src.char_to_line(span.start);
    let start_col = span.start - src.line_to_char(start_line);

    let end_line = src.char_to_line(span.end);
    let end_col = span.end - src.line_to_char(end_line);

    (Position {
        line: start_line as u32,
        character: start_col as u32,
    },
    Position {
        line: end_line as u32,
        character: end_col as u32,
    })
}

pub fn span_from_pos(src: &Rope, start: &Position, end: &Position) -> Span {
    let start = src.line_to_char(start.line as usize) + start.character as usize;
    let end = src.line_to_char(end.line as usize) + end.character as usize;
    
    Span::new(start, end)
}