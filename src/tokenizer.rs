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
                    if let Some('=') = p.peek() {
                        op.push('=');
                        p.next();
                    }
                    cur = Token::push(TokenKind::TkReserved, None, Some(op), &mut cur);
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

fn at_eof(token: &Vec<Token>) -> bool {
    token[token.len() - 1].kind == TokenKind::TkEOF
}
