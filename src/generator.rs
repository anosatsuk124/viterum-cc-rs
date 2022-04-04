use crate::parser::{Node, NodeKind};

fn generator_lval(val: Option<u32>) {
    match val {
        Some(v) => {
            println!("  mov rax, rbp");
            println!("  sub rax, {}", v);
            println!("  push rax");
        }
        None => panic!("The left hand value is not a varible"),
    }
}

pub fn generator(node: Node) {
    match node.kind {
        NodeKind::NdNum => {
            println!("  push {}", &node.val.unwrap());

            return;
        }
        NodeKind::NdLVar => {
            generator_lval(node.offset);

            println!("  pop rax");
            println!("  mov rax, [rax]");
            println!("  push rax");

            return;
        }
        NodeKind::NdAssign => {
            let lhs = *node.lhs.unwrap().clone();
            let rhs = *node.rhs.unwrap().clone();
            generator_lval(lhs.offset);
            generator(rhs);

            println!("  pop rdi");
            println!("  pop rax");
            println!("  mov [rax], rdi");
            println!("  push rdi");

            return;
        }
        _ => (),
    }

    if node.lhs.is_some() {
        generator(*node.lhs.unwrap());
    }
    if node.rhs.is_some() {
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
