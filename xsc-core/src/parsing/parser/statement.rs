mod include;
mod var_def;
mod var_assign;
mod body;
mod if_else;
mod while_;
mod for_;
mod switch;
mod fn_def;
mod break_or_continue;
mod return_;
mod rule_def;
mod postfix;
mod label_def_or_goto_or_dbg;
mod discarded_expr;
mod class_def;

use chumsky::prelude::*;

use crate::parsing::ast::ASTreeNode;
use crate::parsing::lexer::Token;
use crate::parsing::parser::parser_input::ParserInput;
use fn_def::fn_def;
use for_::for_;
use include::include;
use if_else::if_else;
use break_or_continue::break_or_continue_or_breakpt;
use class_def::class_def;
use discarded_expr::discarded_expr;
use label_def_or_goto_or_dbg::label_def_or_goto_or_dbg;
use postfix::postfix;
use return_::return_;
use rule_def::rule_def;
use switch::switch;
use var_assign::var_assign;
use var_def::var_def;
use while_::while_;
use crate::parsing::span::{Span, Spanned};

pub fn statement<'tokens>() -> impl Parser<
    'tokens,
    ParserInput<'tokens>,
    Spanned<ASTreeNode>,
    extra::Err<Rich<'tokens, Token, Span>>,
> + Clone {
    recursive(|statement| {
        choice((
            include(),
            var_def(),
            var_assign(),
            if_else(statement.clone()),
            while_(statement.clone()),
            for_(statement.clone()),
            switch(statement.clone()),
            break_or_continue_or_breakpt(),
            fn_def(statement.clone()),
            return_(),
            rule_def(statement.clone()),
            postfix(),
            label_def_or_goto_or_dbg(),
            discarded_expr(),
            class_def(),
        ))
    })
}