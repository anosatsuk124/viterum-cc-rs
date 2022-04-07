use std::io::BufRead;

mod generator;
mod parser;
mod tokenizer;

fn main() {
    let mut stdin = std::io::BufReader::new(std::io::stdin());

    let mut input = String::new();
    loop {
        match stdin.read_line(&mut input) {
            Ok(0) => break,
            Ok(_) => continue,
            Err(e) => {
                eprintln!("{}", e);
                break;
            }
        }
    }

    let mut tokens = tokenizer::Token::tokenize(input.chars().collect());
    let mut varibles: Vec<parser::LVar> = Vec::new();
    let nodes = parser::program(&mut tokens, &mut varibles);

    println!(".intel_syntax noprefix");
    println!(".globl _start");
    println!("_start:");

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
    println!("  mov rbx, rax");
    println!("  mov rax, 1");
    println!("  int 0x80");
}
