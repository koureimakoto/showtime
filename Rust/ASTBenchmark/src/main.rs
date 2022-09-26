use ast_benchmark::ast;


fn main() {

    let st = String::from("</>");
    
    let mut lex: ast::lexer::Engine = ast::lexer::Engine::tokenize_this(&st);

    println!( "{}", lex.token() );
    println!( "{}", lex.token() );
    println!( "{}", lex.token() );
}
