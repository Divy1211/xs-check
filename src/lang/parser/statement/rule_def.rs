use chumsky::prelude::*;
use crate::lang::ast::astree::{ASTreeNode, RuleOpt};
use crate::lang::ast::literal::Literal;
use crate::lang::lexer::token::Token;
use crate::lang::parser::parser_input::ParserInput;
use crate::lang::parser::statement::body::body;
use crate::lang::span::{Span, Spanned};

pub fn rule_def<'tokens>(
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
    let no_args = one_of([
        Token::Active, Token::Inactive, Token::RunImmediately, Token::HighFrequency
    ]).map_with(|tok, info| (match tok {
            Token::RunImmediately => RuleOpt::RunImmediately,
            Token::HighFrequency  => RuleOpt::HighFrequency,
            Token::Active         => RuleOpt::Active,
            _                     => RuleOpt::Inactive,
        }, info.span()));
    
    let int_arg = one_of([Token::MinInterval, Token::MaxInterval, Token::Priority])
        .then(
            select! { Token::Literal(Literal::Int(val)) => val }
                .map_with(|val, info| (val, info.span()))
        ).map_with(|(tok, val), info| (match tok {
            Token::MinInterval    => RuleOpt::MinInterval(val),
            Token::MaxInterval    => RuleOpt::MaxInterval(val),
            _                     => RuleOpt::Priority(val),
        }, info.span()));
    
    let grp = just(Token::Group)
        .ignore_then(
            select! { Token::Literal(Literal::Str(val)) => val }
                .map_with(|val, info| (val, info.span()))
        ).map_with(|val, info| (RuleOpt::Group(val), info.span()));
    
    let rule_opt = choice((no_args, int_arg, grp));
    
    just(Token::Rule)
        .ignore_then(
            select! { Token::Identifier(id) => id }
                .map_with(|id, info| (id, info.span()))
        ).then(rule_opt.repeated().collect::<Vec<Spanned<RuleOpt>>>())
        .then(body(statement))
        .map_with(|
            ((name, rule_opts), body),
             info
        | (
            ASTreeNode::RuleDef {
                name,
                rule_opts,
                body,
            },
            info.span(),
        ))
}
