use chumsky::input::Input;
use chumsky::Parser;
use xsc_core::parsing::lexer::lexer;
use xsc_core::parsing::parser::parser;

fn main() {
    let src = "void main() { int c = 23; int d = 20; }";
    
    let (tokens, errs) = lexer().parse(src).into_output_errors();
    
    let Some(tokens) = tokens else {
        println!("errs: {:?}", errs);
        return;
    };


    println!("toks: {:?}", tokens);
    
    let (ast, errs) = parser()
        .map_with(|ast, e| (ast, e.span()))
        .parse(tokens.as_slice().spanned((src.len()..src.len()).into()))
        .into_output_errors();


    let Some(ast) = ast else {
        println!("errs: {:?}", errs);
        return;
    };


    println!("ast: {:?}", ast);
}
