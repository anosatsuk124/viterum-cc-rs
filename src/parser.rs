use crate::tokenizer::{self, consume, consume_ident, expect, expect_number, Token};

#[derive(Debug, Clone, PartialEq)]
pub enum NodeKind {
    NdAdd,    // +
    NdSub,    // -
    NdMul,    // *
    NdDiv,    // /
    NdNum,    // integer
    NdEq,     // equality
    NdNotEq,  // equality
    NdLess,   // relational
    NdLessEq, // relational
    NdAssign, // assign
    NdLVar,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    pub kind: NodeKind,
    pub lhs: Option<Box<Node>>,
    pub rhs: Option<Box<Node>>,
    pub val: Option<u32>,
    pub offset: Option<u32>,
}

impl Node {
    fn new(
        kind: NodeKind,
        lhs: Option<Box<Node>>,
        rhs: Option<Box<Node>>,
        val: Option<u32>,
        offset: Option<u32>,
    ) -> Self {
        Node {
            kind,
            lhs,
            rhs,
            val,
            offset,
        }
    }
}

/*
    program    = stmt*
    stmt       = expr ";"
    expr       = assign
    assign     = equality ("=" assign)?
    equality   = relational ("==" relational | "!=" relational)*
    relational = add ("<" add | "<=" add | ">" add | ">=" add)*
    add        = mul ("+" mul | "-" mul)*
    mul        = unary ("*" unary | "/" unary)*
    unary      = ("+" | "-")? primary
    primary    = num | ident | "(" expr ")"
*/

pub fn program(tokens: &mut Vec<Token>) -> Vec<Node> {
    let mut stmts: Vec<Node> = Vec::new();
    while !tokenizer::at_eof(tokens) {
        stmts.push(*stmt(tokens));
    }

    stmts
}

fn stmt(tokens: &mut Vec<Token>) -> Box<Node> {
    let expr = expr(tokens);
    consume(tokens, ";");

    return expr;
}

fn expr(tokens: &mut Vec<Token>) -> Box<Node> {
    assign(tokens)
}

fn assign(tokens: &mut Vec<Token>) -> Box<Node> {
    let mut node: Box<Node> = equality(tokens);
    if consume(tokens, "=") {
        node = Box::new(Node::new(
            NodeKind::NdAssign,
            Some(node),
            Some(assign(tokens)),
            None,
            None,
        ));
    }
    node
}

fn equality(tokens: &mut Vec<Token>) -> Box<Node> {
    let mut node: Box<Node> = relational(tokens);

    loop {
        if consume(tokens, "==") {
            node = Box::new(Node::new(
                NodeKind::NdEq,
                Some(node),
                Some(relational(tokens)),
                None,
                None,
            ));
        } else if consume(tokens, "!=") {
            node = Box::new(Node::new(
                NodeKind::NdNotEq,
                Some(node),
                Some(relational(tokens)),
                None,
                None,
            ));
        } else {
            return node;
        }
    }
}

fn relational(tokens: &mut Vec<Token>) -> Box<Node> {
    let mut node: Box<Node> = add(tokens);

    loop {
        if consume(tokens, "<") {
            node = Box::new(Node::new(
                NodeKind::NdLess,
                Some(node),
                Some(add(tokens)),
                None,
                None,
            ));
        } else if consume(tokens, "<=") {
            node = Box::new(Node::new(
                NodeKind::NdLessEq,
                Some(node),
                Some(add(tokens)),
                None,
                None,
            ));
        } else if consume(tokens, ">") {
            node = Box::new(Node::new(
                NodeKind::NdLess,
                Some(add(tokens)),
                Some(node),
                None,
                None,
            ));
        } else if consume(tokens, ">=") {
            node = Box::new(Node::new(
                NodeKind::NdLessEq,
                Some(add(tokens)),
                Some(node),
                None,
                None,
            ));
        } else {
            return node;
        }
    }
}

fn add(tokens: &mut Vec<Token>) -> Box<Node> {
    let mut node: Box<Node> = mul(tokens);

    loop {
        if consume(tokens, "+") {
            node = Box::new(Node::new(
                NodeKind::NdAdd,
                Some(node),
                Some(mul(tokens)),
                None,
                None,
            ));
        } else if consume(tokens, "-") {
            node = Box::new(Node::new(
                NodeKind::NdSub,
                Some(node),
                Some(mul(tokens)),
                None,
                None,
            ));
        } else {
            return node;
        }
    }
}

fn mul(tokens: &mut Vec<Token>) -> Box<Node> {
    let mut node: Box<Node> = unary(tokens);

    loop {
        if consume(tokens, "*") {
            node = Box::new(Node::new(
                NodeKind::NdMul,
                Some(node),
                Some(unary(tokens)),
                None,
                None,
            ));
        } else if consume(tokens, "/") {
            node = Box::new(Node::new(
                NodeKind::NdDiv,
                Some(node),
                Some(unary(tokens)),
                None,
                None,
            ));
        } else {
            return node;
        }
    }
}

fn unary(tokens: &mut Vec<Token>) -> Box<Node> {
    if consume(tokens, "+") {
        return primary(tokens);
    }
    if consume(tokens, "-") {
        return Box::new(Node::new(
            NodeKind::NdSub,
            Some(Box::new(Node::new(
                NodeKind::NdNum,
                None,
                None,
                Some(0),
                None,
            ))),
            Some(primary(tokens)),
            None,
            None,
        ));
    }

    primary(tokens)
}

fn primary(tokens: &mut Vec<Token>) -> Box<Node> {
    if consume(tokens, "(") {
        let node = expr(tokens);
        expect(tokens, ")");

        return node;
    }
    let offset = consume_ident(tokens);
    if offset.0 {
        let offset = Some(
            ((offset.1.unwrap().chars().collect::<Vec<char>>()[0] as u8 - 'a' as u8 + 1) * 8)
                .into(),
        );
        return Box::new(Node::new(NodeKind::NdLVar, None, None, None, offset));
    } else {
        return Box::new(Node::new(
            NodeKind::NdNum,
            None,
            None,
            expect_number(tokens),
            None,
        ));
    }
}
