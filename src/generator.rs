use crate::parser::{Node, NodeKind};

pub fn generator(node: Node) {
    if node.kind == NodeKind::NdNum {
        println!("  push {}", &node.val.unwrap());
        return;
    }

    if node.lhs != None {
        generator(*node.lhs.unwrap());
    }
    if node.rhs != None {
        generator(*node.rhs.unwrap());
    }

    println!("  pop rdi");
    println!("  pop rax");

    match node.kind {
        NodeKind::NdEq => {
            println!("  cmp rax, rdi");
            println!("  sete al");
            println!("  movzb rax, al");
        }
        NodeKind::NdNotEq => {
            println!("  cmp rax, rdi");
            println!("  setne al");
            println!("  movzb rax, al");
        }
        NodeKind::NdLess => {
            println!("  cmp rax, rdi");
            println!("  setl al");
            println!("  movzb rax, al");
        }
        NodeKind::NdLessEq => {
            println!("  cmp rax, rdi");
            println!("  setle al");
            println!("  movzb rax, al");
        }
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
