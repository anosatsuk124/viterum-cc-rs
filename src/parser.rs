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

#[derive(Clone)]
pub struct LVar {
    name: String,
    offset: u32,
}

impl LVar {
    fn new(name: String, offset: u32) -> Self {
        LVar { name, offset }
    }

    fn find(name: String, vars: Vec<Self>) -> Option<Self> {
        for var in vars.into_iter() {
            if var.name == name {
                return Some(var.clone());
            }
        }
        None
    }
}

pub fn program(tokens: &mut Vec<Token>, vars: &mut Vec<LVar>) -> Vec<Node> {
    let mut stmts: Vec<Node> = Vec::new();
    while !tokenizer::at_eof(tokens) {
        stmts.push(*stmt(tokens, vars));
    }

    stmts
}

fn stmt(tokens: &mut Vec<Token>, vars: &mut Vec<LVar>) -> Box<Node> {
    let expr = expr(tokens, vars);
    consume(tokens, ";");

    return expr;
}

fn expr(tokens: &mut Vec<Token>, vars: &mut Vec<LVar>) -> Box<Node> {
    assign(tokens, vars)
}

fn assign(tokens: &mut Vec<Token>, vars: &mut Vec<LVar>) -> Box<Node> {
    let mut node: Box<Node> = equality(tokens, vars);
    if consume(tokens, "=") {
        node = Box::new(Node::new(
            NodeKind::NdAssign,
            Some(node),
            Some(assign(tokens, vars)),
            None,
            None,
        ));
    }
    node
}

fn equality(tokens: &mut Vec<Token>, vars: &mut Vec<LVar>) -> Box<Node> {
    let mut node: Box<Node> = relational(tokens, vars);

    loop {
        if consume(tokens, "==") {
            node = Box::new(Node::new(
                NodeKind::NdEq,
                Some(node),
                Some(relational(tokens, vars)),
                None,
                None,
            ));
        } else if consume(tokens, "!=") {
            node = Box::new(Node::new(
                NodeKind::NdNotEq,
                Some(node),
                Some(relational(tokens, vars)),
                None,
                None,
            ));
        } else {
            return node;
        }
    }
}

fn relational(tokens: &mut Vec<Token>, vars: &mut Vec<LVar>) -> Box<Node> {
    let mut node: Box<Node> = add(tokens, vars);

    loop {
        if consume(tokens, "<") {
            node = Box::new(Node::new(
                NodeKind::NdLess,
                Some(node),
                Some(add(tokens, vars)),
                None,
                None,
            ));
        } else if consume(tokens, "<=") {
            node = Box::new(Node::new(
                NodeKind::NdLessEq,
                Some(node),
                Some(add(tokens, vars)),
                None,
                None,
            ));
        } else if consume(tokens, ">") {
            node = Box::new(Node::new(
                NodeKind::NdLess,
                Some(add(tokens, vars)),
                Some(node),
                None,
                None,
            ));
        } else if consume(tokens, ">=") {
            node = Box::new(Node::new(
                NodeKind::NdLessEq,
                Some(add(tokens, vars)),
                Some(node),
                None,
                None,
            ));
        } else {
            return node;
        }
    }
}

fn add(tokens: &mut Vec<Token>, vars: &mut Vec<LVar>) -> Box<Node> {
    let mut node: Box<Node> = mul(tokens, vars);

    loop {
        if consume(tokens, "+") {
            node = Box::new(Node::new(
                NodeKind::NdAdd,
                Some(node),
                Some(mul(tokens, vars)),
                None,
                None,
            ));
        } else if consume(tokens, "-") {
            node = Box::new(Node::new(
                NodeKind::NdSub,
                Some(node),
                Some(mul(tokens, vars)),
                None,
                None,
            ));
        } else {
            return node;
        }
    }
}

fn mul(tokens: &mut Vec<Token>, vars: &mut Vec<LVar>) -> Box<Node> {
    let mut node: Box<Node> = unary(tokens, vars);

    loop {
        if consume(tokens, "*") {
            node = Box::new(Node::new(
                NodeKind::NdMul,
                Some(node),
                Some(unary(tokens, vars)),
                None,
                None,
            ));
        } else if consume(tokens, "/") {
            node = Box::new(Node::new(
                NodeKind::NdDiv,
                Some(node),
                Some(unary(tokens, vars)),
                None,
                None,
            ));
        } else {
            return node;
        }
    }
}

fn unary(tokens: &mut Vec<Token>, vars: &mut Vec<LVar>) -> Box<Node> {
    if consume(tokens, "+") {
        return primary(tokens, vars);
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
            Some(primary(tokens, vars)),
            None,
            None,
        ));
    }

    primary(tokens, vars)
}

fn primary(tokens: &mut Vec<Token>, vars: &mut Vec<LVar>) -> Box<Node> {
    if consume(tokens, "(") {
        let node = expr(tokens, vars);
        expect(tokens, ")");

        return node;
    }

    let offset = consume_ident(tokens);
    if offset.is_some() {
        match LVar::find(offset.clone().unwrap(), vars.to_vec()) {
            Some(var) => {
                return Box::new(Node::new(
                    NodeKind::NdLVar,
                    None,
                    None,
                    None,
                    Some(var.offset),
                ));
            }
            None => {
                if vars.len() != 0 {
                    let next_offset = vars[vars.len() - 1].offset + 8;
                    vars.push(LVar::new(offset.clone().unwrap(), next_offset));
                    return Box::new(Node::new(
                        NodeKind::NdLVar,
                        None,
                        None,
                        None,
                        Some(next_offset),
                    ));
                } else {
                    let next_offset = 8;
                    vars.push(LVar::new(offset.clone().unwrap(), next_offset));
                    return Box::new(Node::new(
                        NodeKind::NdLVar,
                        None,
                        None,
                        None,
                        Some(next_offset),
                    ));
                }
            }
        }
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
