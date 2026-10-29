use ropey::Rope;
use tower_lsp::lsp_types::Position;
use xsc_core::parsing::span::Span;

pub fn pos_to_char(src: &Rope, pos: &Position) -> usize {
    let line = src.line(pos.line as usize);
    src.line_to_char(pos.line as usize) + line.utf16_cu_to_char(pos.character as usize)
}

pub fn pos_to_byte(src: &Rope, pos: &Position) -> usize {
    let line = src.line(pos.line as usize);
    src.line_to_byte(pos.line as usize) + line.char_to_byte(line.utf16_cu_to_char(pos.character as usize))
}

pub fn byte_to_pos(src: &Rope, index: usize) -> Position {
    let line = src.byte_to_line(index);
    let line_start = src.line_to_byte(line);
    let byte_col = index - line_start;

    let rope = src.line(line);
    let utf16_col = rope.slice(..rope.byte_to_char(byte_col)).len_utf16_cu();

    Position {
        line: line as u32,
        character: utf16_col as u32,
    }
}

pub fn pos_from_span(src: &Rope, span: &Span) -> (Position, Position) {
    (byte_to_pos(src, span.start), byte_to_pos(src, span.end))
}

pub fn span_from_pos(src: &Rope, start: &Position, end: &Position) -> Span {
    let start = pos_to_byte(src, start);
    let end = pos_to_byte(src, end);
    
    Span::new(start, end)
}