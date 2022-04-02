fn main() {
    if std::env::args().len() != 2 {
        eprintln!("The number of arguments are not collect.");
        std::process::exit(1)
    }
    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");
    println!(
        "  mov rax, {}",
        std::env::args().collect::<Vec<String>>()[1]
            .parse::<u32>()
            .unwrap()
    );
    println!("  ret");
    std::process::exit(0)
}
