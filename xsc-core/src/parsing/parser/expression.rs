use chumsky::prelude::*;
use crate::parsing::ast::Expr;
use crate::parsing::lexer::Token;
use crate::parsing::parser::parser_input::ParserInput;
use crate::parsing::span::{Span, Spanned};

pub fn expression<'tokens>() -> impl Parser<
    'tokens,
    ParserInput<'tokens>,
    Spanned<Expr>,
    extra::Err<Rich<'tokens, Token, Span>>,
> + Clone {
    recursive(|expr| {
        let paren_expr = expr.clone()
            .delimited_by(just(Token::LParen), just(Token::RParen))
            .map_with(|exp, info| (Expr::Paren(Box::new(exp)), info.span()));
        
        let lit = select! { Token::Literal(lit) => Expr::Literal(lit) }
            .map_with(|exp, info| (exp, info.span()));

        let id = select! {
            Token::Identifier(id) => Expr::Identifier(id),
        }.map_with(|exp, info| (exp, info.span()));
        
        let vec_lit = just(Token::Vector).ignore_then(
            expr.clone().then_ignore(just(Token::Comma))
                .then(expr.clone().then_ignore(just(Token::Comma)))
                .then(expr.clone())
                .delimited_by(just(Token::LParen), just(Token::RParen))
            ).map_with(|((x, y), z), info| {
                (Expr::Vec { x: Box::new(x), y: Box::new(y), z: Box::new(z) }, info.span())
            });
        
        let fn_call = select! { Token::Identifier(id) => id }
            .map_with(|id, info| (id, info.span()))
            .then(
                expr.clone()
                .separated_by(just(Token::Comma))
                .allow_trailing()
                .collect::<Vec<Spanned<Expr>>>()
                .delimited_by(just(Token::LParen), just(Token::RParen))
            ).map_with(|(name, args), info| {
                (Expr::FnCall { name, args }, info.span())
            });

        let expr7 = choice((
            vec_lit,
            fn_call,
            paren_expr,
            lit,
            id,
        )).boxed();

        let unary = one_of([Token::Minus, Token::Excl]).or_not()
            .then(expr7)
            .map_with(|(sign, exp), info| match sign {
                Some(Token::Minus) => (Expr::Neg(Box::new(exp)), info.span()),
                Some(_)  =>           (Expr::Not(Box::new(exp)), info.span()),
                None    => exp,
            }).boxed();

        let expr6 = unary.clone()
            .foldl_with(
                one_of([Token::Star, Token::FSlash, Token::PCent]).then(unary).repeated(),
                |a, (op, b), info| (match op {
                    Token::Star   => Expr::Star(Box::new(a), Box::new(b)),
                    Token::FSlash => Expr::FSlash(Box::new(a), Box::new(b)),
                    _             => Expr::PCent(Box::new(a), Box::new(b)),
                }, info.span())
            ).boxed();

        let expr5 = expr6.clone()
            .foldl_with(
                one_of([Token::Plus, Token::Minus]).then(expr6).repeated(),
                |a, (op, b), info| (match op {
                    Token::Plus  => Expr::Plus(Box::new(a), Box::new(b)),
                    _            => Expr::Minus(Box::new(a), Box::new(b)),
                }, info.span())
            ).boxed();

        let expr4 = expr5.clone()
            .foldl_with(
                one_of([Token::Lt, Token::Le, Token::Gt, Token::Ge]).then(expr5).repeated(),
                |a, (op, b), info| (match op {
                    Token::Lt => Expr::Lt(Box::new(a), Box::new(b)),
                    Token::Le => Expr::Le(Box::new(a), Box::new(b)),
                    Token::Gt => Expr::Gt(Box::new(a), Box::new(b)),
                    _         => Expr::Ge(Box::new(a), Box::new(b)),
                }, info.span())
            ).boxed();

        let expr3 = expr4.clone()
            .foldl_with(
                one_of([Token::Deq, Token::Neq]).then(expr4).repeated(),
                |a, (op, b), info| (match op {
                    Token::Deq => Expr::Eq(Box::new(a), Box::new(b)),
                    _          => Expr::Ne(Box::new(a), Box::new(b)),
                }, info.span())
            ).boxed();

        let expr2 = expr3.clone()
            .foldl_with(
                just(Token::DAmp).ignore_then(expr3).repeated(),
                |a, b, info| {
                    (Expr::And(Box::new(a), Box::new(b)), info.span())
                }
            ).boxed();

        let expr1 = expr2.clone()
            .foldl_with(
                just(Token::DPipe).ignore_then(expr2).repeated(),
                |a, b, info| {
                    (Expr::Or(Box::new(a), Box::new(b)), info.span())
                }
            ).boxed();

        expr1
    })
}