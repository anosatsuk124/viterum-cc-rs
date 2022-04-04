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

#[derive(PartialEq, Eq, Clone, Debug)]
enum TokenKind {
    TkReserved, // symbol
    TkNum,      // integer token
    TkEOF,      // end token
}

#[derive(Clone, Debug)]
pub struct Token {
    kind: TokenKind,
    val: Option<u32>,
    op: Option<char>,
}

impl Token {
    fn push(kind: TokenKind, val: Option<u32>, op: Option<char>, vec: &mut Vec<Self>) -> Vec<Self> {
        let token = Token { kind, val, op };
        vec.push(token);

        vec.clone()
    }

    pub fn tokenize(pos: Vec<char>) -> Vec<Token> {
        let mut p = pos.into_iter().peekable();
        let mut cur: Vec<Token> = Vec::new();

        while let Some(c) = p.peek() {
            match c {
                sp if sp.is_whitespace() => {
                    p.next();
                    continue;
                }
                '+' | '-' | '*' | '/' | '(' | ')' => {
                    cur = Token::push(TokenKind::TkReserved, None, Some(c.clone()), &mut cur);
                    p.next();
                    continue;
                }
                '0'..='9' => {
                    cur = Token::push(TokenKind::TkNum, Some(strtou32(&mut p, 10)), None, &mut cur);
                    continue;
                }
                _ => panic!("Could not tokenize"),
            }
        }
        cur = Token::push(TokenKind::TkEOF, None, None, &mut cur);
        return cur.into_iter().rev().collect();
    }
}

pub fn consume(tokens: &mut Vec<Token>, op: char) -> bool {
    if tokens[tokens.len() - 1].kind != TokenKind::TkReserved
        || tokens[tokens.len() - 1].op != Some(op)
    {
        return false;
    }

    tokens.pop();
    return true;
}

pub fn expect_number(tokens: &mut Vec<Token>) -> Option<u32> {
    if tokens[tokens.len() - 1].kind != TokenKind::TkNum {
        panic!("it is not a number.");
    }

    tokens.pop().unwrap().val
}

pub fn expect(tokens: &mut Vec<Token>, op: char) {
    if tokens[tokens.len() - 1].kind != TokenKind::TkReserved
        || tokens[tokens.len() - 1].op != Some(op)
    {
        panic!("it is not ~.");
    }
    tokens.pop();
}

fn at_eof(token: &Vec<Token>) -> bool {
    token[token.len() - 1].kind == TokenKind::TkEOF
}

/*

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("The number of arguments is not correct.");
        std::process::exit(1)
    }

    let mut token = Token::tokenize(args[1].chars().collect());
    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");
    println!("  mov rax, {}", expect_number(&mut token).unwrap());

    while !(at_eof(&token)) {
        if consume(&mut token, '+') {
            println!("  add rax, {}", expect_number(&mut token).unwrap());
            continue;
        } else if consume(&mut token, '-') {
            println!("  sub rax, {}", expect_number(&mut token).unwrap());
            continue;
        }
    }

    println!("  ret");
}

*/
