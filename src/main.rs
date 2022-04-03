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
struct Token {
    kind: TokenKind,
    next: Vec<Token>,
    val: Option<u32>,
    op: Option<char>,
}

impl Token {
    fn new(kind: TokenKind, cur: &mut Vec<Token>, val: Option<u32>, op: Option<char>) -> Vec<Self> {
        let token = Token {
            kind,
            next: cur.to_vec(),
            val,
            op,
        };

        cur.push(token);
        return cur.clone();
    }

    fn tokenize(pos: Vec<char>) -> Vec<Token> {
        let mut p = pos.into_iter().peekable();
        let mut cur = Token::new(TokenKind::TkEOF, &mut Vec::new(), None, None);
        cur.pop();

        while let Some(c) = p.peek() {
            match c {
                sp if sp.is_whitespace() => {
                    p.next();
                    continue;
                }
                '+' | '-' => {
                    cur = Token::new(TokenKind::TkReserved, &mut cur, None, Some(c.clone()));
                    p.next();
                    continue;
                }
                '0'..='9' => {
                    cur = Token::new(TokenKind::TkNum, &mut cur, Some(strtou32(&mut p, 10)), None);
                    continue;
                }
                _ => panic!("Could not tokenize"),
            }
        }
        cur = Token::new(TokenKind::TkEOF, &mut cur, None, None);
        return cur.into_iter().rev().collect();
    }
}

fn consume(token: &mut Vec<Token>, op: char) -> bool {
    if token[token.len() - 1].kind != TokenKind::TkReserved || token[token.len() - 1].op != Some(op)
    {
        return false;
    }

    token.pop();
    return true;
}

fn expect_number(token: &mut Vec<Token>) -> (Option<u32>, &mut Vec<Token>) {
    if token[token.len() - 1].kind != TokenKind::TkNum {
        panic!("it is not a number.");
    }

    return (token.pop().unwrap().val, token);
}

fn at_eof(token: &Vec<Token>) -> bool {
    token[token.len() - 1].kind == TokenKind::TkEOF
}

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
    println!("  mov rax, {}", expect_number(&mut token).0.unwrap());

    while !(at_eof(&token)) {
        if consume(&mut token, '+') {
            println!("  add rax, {}", expect_number(&mut token).0.unwrap());
            continue;
        } else if consume(&mut token, '-') {
            println!("  sub rax, {}", expect_number(&mut token).0.unwrap());
            continue;
        }
    }

    println!("  ret");
}
