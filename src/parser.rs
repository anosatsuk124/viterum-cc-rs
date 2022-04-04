use crate::tokenizer::{consume, expect, expect_number, Token};

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
}

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    pub kind: NodeKind,
    pub lhs: Option<Box<Node>>,
    pub rhs: Option<Box<Node>>,
    pub val: Option<u32>,
}

impl Node {
    fn new(
        kind: NodeKind,
        lhs: Option<Box<Node>>,
        rhs: Option<Box<Node>>,
        val: Option<u32>,
    ) -> Self {
        Node {
            kind,
            lhs,
            rhs,
            val,
        }
    }
}

pub fn expr(tokens: &mut Vec<Token>) -> Node {
    *equality(tokens)
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
            ));
        } else if consume(tokens, "!=") {
            node = Box::new(Node::new(
                NodeKind::NdNotEq,
                Some(node),
                Some(relational(tokens)),
                None,
            ));
        } else {
            return node.clone();
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
            ));
        } else if consume(tokens, "<=") {
            node = Box::new(Node::new(
                NodeKind::NdLessEq,
                Some(node),
                Some(add(tokens)),
                None,
            ));
        } else if consume(tokens, ">") {
            node = Box::new(Node::new(
                NodeKind::NdLess,
                Some(add(tokens)),
                Some(node),
                None,
            ));
        } else if consume(tokens, ">=") {
            node = Box::new(Node::new(
                NodeKind::NdLessEq,
                Some(add(tokens)),
                Some(node),
                None,
            ));
        } else {
            return node.clone();
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
            ));
        } else if consume(tokens, "-") {
            node = Box::new(Node::new(
                NodeKind::NdSub,
                Some(node),
                Some(mul(tokens)),
                None,
            ));
        } else {
            return node.clone();
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
            ));
        } else if consume(tokens, "/") {
            node = Box::new(Node::new(
                NodeKind::NdDiv,
                Some(node),
                Some(unary(tokens)),
                None,
            ));
        } else {
            return node.clone();
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
            Some(Box::new(Node::new(NodeKind::NdNum, None, None, Some(0)))),
            Some(primary(tokens)),
            None,
        ));
    }

    primary(tokens)
}

fn primary(tokens: &mut Vec<Token>) -> Box<Node> {
    if consume(tokens, "(") {
        let node = expr(tokens);
        expect(tokens, ")");

        return Box::new(node);
    }

    Box::new(Node::new(
        NodeKind::NdNum,
        None,
        None,
        expect_number(tokens),
    ))
}
