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
    TkIdent,
    TkEOF, // end token
}

#[derive(Clone, Debug)]
pub struct Token {
    kind: TokenKind,
    val: Option<u32>,
    op: Option<String>,
}

impl Token {
    fn push(
        kind: TokenKind,
        val: Option<u32>,
        op: Option<String>,
        vec: &mut Vec<Self>,
    ) -> Vec<Self> {
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
                asc if asc.is_ascii_alphabetic() => {
                    let var = c.to_string().clone();
                    p.next();
                    cur = Token::push(TokenKind::TkIdent, None, Some(var), &mut cur);

                    continue;
                }
                '+' | '-' | '*' | '/' | '(' | ')' => {
                    cur = Token::push(
                        TokenKind::TkReserved,
                        None,
                        Some(c.clone().to_string()),
                        &mut cur,
                    );
                    p.next();
                    continue;
                }
                '=' | '!' | '<' | '>' => {
                    let mut op = c.to_string();
                    p.next();
                    if Some(&'=') == p.peek() {
                        p.next();
                        op.push('=');
                        cur = Token::push(TokenKind::TkReserved, None, Some(op.clone()), &mut cur);
                    } else {
                        cur = Token::push(TokenKind::TkReserved, None, Some(op.clone()), &mut cur);
                    }
                    continue;
                }
                ';' => {
                    cur = Token::push(
                        TokenKind::TkReserved,
                        None,
                        Some(c.to_string().clone()),
                        &mut cur,
                    );
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

pub fn consume_ident(tokens: &mut Vec<Token>) -> (bool, Option<String>) {
    if tokens[tokens.len() - 1].kind != TokenKind::TkIdent {
        return (false, None);
    }

    (true, tokens.pop().unwrap().op)
}

pub fn consume(tokens: &mut Vec<Token>, op: &str) -> bool {
    if tokens[tokens.len() - 1].kind != TokenKind::TkReserved
        || tokens[tokens.len() - 1].op != Some(op.to_string())
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

pub fn expect(tokens: &mut Vec<Token>, op: &str) {
    if tokens[tokens.len() - 1].kind != TokenKind::TkReserved
        || tokens[tokens.len() - 1].op != Some(op.to_string())
    {
        panic!("it is not ~.");
    }
    tokens.pop();
}

pub fn at_eof(tokens: &Vec<Token>) -> bool {
    tokens[tokens.len() - 1].kind == TokenKind::TkEOF
}
