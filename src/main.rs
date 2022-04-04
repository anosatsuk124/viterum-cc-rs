mod generator;
mod parser;
mod tokenizer;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        panic!("The number of arguments is not correct.");
    }

    let mut tokens = tokenizer::Token::tokenize(args[1].chars().collect());
    let node = parser::expr(&mut tokens);

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");

    generator::generator(node);

    println!("  pop rax");
    println!("  ret");
}
