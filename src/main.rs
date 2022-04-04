mod generator;
mod parser;
mod tokenizer;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        panic!("The number of arguments is not correct.");
    }

    let mut tokens = tokenizer::Token::tokenize(args[1].chars().collect());
    let nodes = parser::program(&mut tokens);

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");

    // prologue
    println!("  push rbp");
    println!("  mov rbp, rsp");
    println!("  sub rsp, 208");

    for node in nodes.into_iter() {
        generator::generator(node);

        println!("  pop rax");
    }

    // epilogue
    println!("  mov rsp, rbp");
    println!("  pop rbp");
    println!("  ret");
}
