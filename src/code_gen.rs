use crate::parser::*;

pub fn code_gen(ast: &NodeProgram) -> String {
    let mut asm = String::from("global _start\n_start:\n");
    
    for stmt in &ast.statements {
        match stmt {
            NodeStatements::Declare(_declare_stmt) => {
            },
            NodeStatements::Exit(exit_stmt) => {
                let int_lit = &exit_stmt.expression.int_lit;
                asm.push_str("    mov rax, 60\n");
                asm.push_str(&format!("    mov rdi, {}\n", int_lit.info));
                asm.push_str("    syscall\n");

            }
            //_ => { println!("Unable to generate code for {:?}", stmt) }
        }
    }

    return asm
}

