use std::iter::Peekable;

fn strtou32<T: Iterator<Item = char>>(iter: &mut Peekable<T>, dig: u32) -> u32 {
    let mut val: u32 = 0;

    while let Some(pos) = iter.peek() {
        match pos.to_digit(dig) {
            Some(num) => val = val * dig + num,
            None => break,
        }
        iter.next();
    }

    return val;
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("The number of arguments are not collect.");
        std::process::exit(1)
    }

    let mut expr = args[1].chars().peekable();

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");
    println!("  mov rax, {}", strtou32(&mut expr, 10));

    while let Some(c) = expr.next() {
        match c {
            '+' => println!("  add rax, {}", strtou32(&mut expr, 10)),
            '-' => println!("  sub rax, {}", strtou32(&mut expr, 10)),
            _ => eprintln!("Invalid charactor: '{}'", c),
        }
    }
    println!("  ret");
}
