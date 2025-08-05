use chumsky::prelude::*;

use crate::parsing::ast::AstNode;
use crate::parsing::lexer::Token;
use crate::parsing::parser::parser_input::ParserInput;
use crate::parsing::span::{Span, Spanned};

pub fn label_def_or_goto_or_dbg<'tokens>() -> impl Parser<
    'tokens,
    ParserInput<'tokens>,
    Spanned<AstNode>,
    extra::Err<Rich<'tokens, Token, Span>>,
> + Clone {
    one_of([Token::Label, Token::Goto, Token::Dbg])
        .then(
            select! { Token::Identifier(id) => id }
                .map_with(|id, info| (id, info.span()))
        )
        .then_ignore(just(Token::SColon))
        .map_with(|(tok, name), info| (match tok {
            Token::Label => AstNode::LabelDef(name),
            Token::Goto  => AstNode::Goto(name),
            _            => AstNode::Debug(name),
        }, info.span()))
}
