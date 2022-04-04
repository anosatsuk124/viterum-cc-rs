use tokenizer::{consume, expect, expect_number};

use crate::tokenizer::Token;

mod tokenizer;

#[derive(Debug, Clone, PartialEq)]
enum NodeKind {
    NdAdd, // +
    NdSub, // -
    NdMul, // *
    NdDiv, // /
    NdNum, // integer
    NdExpr,
}

#[derive(Debug, Clone, PartialEq)]
struct Node {
    kind: NodeKind,
    lhs: Option<Box<Node>>,
    rhs: Option<Box<Node>>,
    val: Option<u32>,
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

fn expr(tokens: &mut Vec<tokenizer::Token>) -> Node {
    let mut node: Node = *mul(tokens);

    loop {
        if consume(tokens, '+') {
            node = Node::new(
                NodeKind::NdAdd,
                Some(Box::new(node)),
                Some(mul(tokens)),
                None,
            );
        } else if consume(tokens, '-') {
            node = Node::new(
                NodeKind::NdSub,
                Some(Box::new(node)),
                Some(mul(tokens)),
                None,
            );
        } else {
            return node.clone();
        }
    }
}

fn mul(tokens: &mut Vec<tokenizer::Token>) -> Box<Node> {
    let mut node: Box<Node> = primary(tokens);

    loop {
        if consume(tokens, '*') {
            node = Box::new(Node::new(
                NodeKind::NdMul,
                Some(node),
                Some(primary(tokens)),
                None,
            ));
        } else if consume(tokens, '/') {
            node = Box::new(Node::new(
                NodeKind::NdDiv,
                Some(node),
                Some(primary(tokens)),
                None,
            ));
        } else {
            return node.clone();
        }
    }
}

fn primary(tokens: &mut Vec<tokenizer::Token>) -> Box<Node> {
    if consume(tokens, '(') {
        let node = expr(tokens);
        expect(tokens, ')');

        return Box::new(node);
    }

    Box::new(Node::new(
        NodeKind::NdNum,
        None,
        None,
        expect_number(tokens),
    ))
}

fn gen(node: Node) {
    if node.kind == NodeKind::NdNum {
        println!("  push {}", &node.val.unwrap());
        return;
    }

    if node.lhs != None {
        gen(*node.lhs.unwrap());
    }
    if node.rhs != None {
        gen(*node.rhs.unwrap());
    }

    println!("  pop rdi");
    println!("  pop rax");

    match node.kind {
        NodeKind::NdAdd => {
            println!("  add rax, rdi");
        }
        NodeKind::NdSub => {
            println!("  sub rax, rdi");
        }
        NodeKind::NdMul => {
            println!("  imul rax, rdi");
        }
        NodeKind::NdDiv => {
            println!("  cqo");
            println!("  idiv rdi");
        }
        _ => (),
    }
    println!("  push rax");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("The number of arguments is not correct.");
        std::process::exit(1)
    }

    let mut tokens = Token::tokenize(args[1].chars().collect());
    let node = expr(&mut tokens);

    //println!("test: {:?}", node);

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");

    gen(node);

    println!("  pop rax");
    println!("  ret");
}
