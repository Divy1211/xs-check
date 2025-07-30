use chumsky::prelude::*;

use crate::parsing::ast::{ASTreeNode, Expr, Identifier};
use crate::parsing::lexer::Token;
use crate::parsing::parser::expression::expression;
use crate::parsing::parser::parser_input::ParserInput;
use crate::parsing::parser::statement::body::body;
use crate::parsing::parser::statement::var_assign::var_assign;
use crate::parsing::span::{Span, Spanned};

pub fn for_<'tokens>(
    statement: impl Parser<
        'tokens,
        ParserInput<'tokens>,
        Spanned<ASTreeNode>,
        extra::Err<Rich<'tokens, Token, Span>>,
    > + Clone
) -> impl Parser<
    'tokens,
    ParserInput<'tokens>,
    Spanned<ASTreeNode>,
    extra::Err<Rich<'tokens, Token, Span>>,
> + Clone {
    // XS for loop syntax is absolutely ugly:
    // for(i = 0; < 10) {}
    // is equivalent to the following in C++
    // for(int i = 0; i < 10; ++i) {}

    just(Token::For)
        .ignore_then(
            var_assign()
                .then(one_of([Token::Le, Token::Lt, Token::Ge, Token::Gt,]))
                .then(expression())
                .delimited_by(just(Token::LParen), just(Token::RParen))
        )
        .then(body(statement.clone()))
        .map_with(|
            (((var_operand1, op), operand2), body),
            info
        | {
            let (var, _span) = var_operand1.clone();
            let (op1, op1_span) = match var {
                ASTreeNode::VarAssign { name, .. } => name,
                _ => (Identifier::new("ForUnreachable"), SimpleSpan::new(0, 0)),
            };

            let spanned_expr = (Expr::Identifier(op1), op1_span);
            let (_op2, op2_span) = operand2.clone();
            let condition = (match op {
                Token::Le => Expr::Le(Box::new(spanned_expr), Box::new(operand2)),
                Token::Lt => Expr::Lt(Box::new(spanned_expr), Box::new(operand2)),
                Token::Ge => Expr::Ge(Box::new(spanned_expr), Box::new(operand2)),
                _         => Expr::Gt(Box::new(spanned_expr), Box::new(operand2)),
            }, SimpleSpan::new(op1_span.start, op2_span.end));

            (ASTreeNode::For { var: Box::new(var_operand1), condition, body }, info.span())
        })
}
