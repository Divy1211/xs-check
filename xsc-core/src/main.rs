// use chumsky::input::Input;
// use chumsky::Parser;
// use xsc_core::parsing::lexer::lexer;
// use xsc_core::parsing::parser::parser;
// 
// fn main() {
//     let src = "void main() { int c = 23; int d = 20; }";
//     
//     let (tokens, errs) = lexer().parse(src).into_output_errors();
//     
//     let Some(tokens) = tokens else {
//         println!("errs: {:?}", errs);
//         return;
//     };
// 
// 
//     println!("toks: {:?}", tokens);
//     
//     let (ast, errs) = parser()
//         .map_with(|ast, e| (ast, e.span()))
//         .parse(tokens.as_slice().spanned((src.len()..src.len()).into()))
//         .into_output_errors();
// 
// 
//     let Some(ast) = ast else {
//         println!("errs: {:?}", errs);
//         return;
//     };
// 
// 
//     println!("ast: {:?}", ast);
// }

use xsc_core::doxygen::Doc;

fn main() {
    let doc = Doc::parse(r#"/**
* Adds a new (or edits an existing) task with the fields previously defined by calls to [xsTaskAmount](./#532-xstaskamount) for the specified unit at the end of the task list (see A.G.E.). If a task with the specified `actionType`, `unitId`, and `Search Wait Time` (set by `xsTaskAmount`) already exists, it is edited instead of a new one being added.
* 
* Note that `xsTaskAmount` modifies a global task struct which is re-used every time `#!xs xsTask` is called (For non programmers, this is similar to filling out a form once (the calls to [xsTaskAmount](./#532-xstaskamount)) and then submitting multiple copies of it for different people)
*
* @param unitId The unit to add the task to
* @param actionType Task type. Eg.: 105 for heal, 155 for aura and etc. Look in the A.G.E.
* @param targetUnitId Target unitId for the task if exists. Values 9xx refer to classes.
* @param playerId The player to whose units the task will be inserted. If unspecified or -1, applies to all players except Gaia.
*
* @returns void
*/"#);
    
    println!("{:?}", doc);
}