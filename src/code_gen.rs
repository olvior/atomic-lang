use crate::parser::*;

pub fn code_gen(ast: &NodeExit) -> String {
    let mut asm = String::from("global _start\n_start:\n");
    
    asm.push_str("    mov rax, 60\n");
    asm.push_str(&format!("    mov rdi, {}\n", ast.expr.int_lit.info));
    asm.push_str("    syscall\n");

    return asm
}

