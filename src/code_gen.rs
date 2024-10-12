use crate::code_gen::math::OperationType;

use crate::parser::MathValue;

use std::{borrow::Borrow, collections::HashMap};

use crate::{
    exit_message,
    parser::*,
    tokenise::Token,
};

pub struct CodeGen {
    stack_ptr: isize,
    pub asm: String,

    variables: HashMap<String, isize>,
}

impl CodeGen {
    pub fn new() -> CodeGen {
        CodeGen {
            stack_ptr: -1,
            asm: String::from("global _start\n_start:\n"),

            variables: HashMap::new(),
        }
    }

    pub fn generate(&mut self, ast: &NodeProgram) {
        
        self.asm.push_str("    push rsp\n");
        
        for stmt in &ast.statements {
            match stmt {
                NodeStatements::Declare(declare_stmt) => {
                    self.gen_declare(&declare_stmt);
                },
                NodeStatements::Exit(exit_stmt) => {
                    self.gen_exit(&exit_stmt);
                },
                NodeStatements::PutChar(putchar_stmt) => {
                    self.gen_putchar(&putchar_stmt);
                }
                NodeStatements::Set(set_stmt) => {
                    self.gen_set(&set_stmt);
                },

                //_ => { println!("Unable to generate code for {:?}", stmt) }
            }
        }
        

        self.asm.push_str("    mov rax, 60\n");
        self.asm.push_str("    mov rdi, 0\n");
        self.asm.push_str("    syscall\n");
    }

    fn gen_declare(&mut self, declare_stmt: &NodeStmtDeclare) {
        if self.var_declared(&declare_stmt.identifier) {
            exit_message(&format!("Variable {} has already been declared!", declare_stmt.identifier.info));
        }

        self.variables.insert(declare_stmt.identifier.info.clone(), self.stack_ptr + 1);

        self.gen_expression(&declare_stmt.expression);

    }

    fn gen_set(&mut self, set_stmt: &NodeStmtSet) {
        if !self.var_declared(&set_stmt.identifier) {
            exit_message(&format!("Variable referenced before assignment {}", set_stmt.identifier.info));
            return;
        }

        self.gen_expression(&set_stmt.expression);
        self.pop("rax");

        let var_ptr = self.get_var_ptr(&set_stmt.identifier);
        self.asm.push_str(&format!("    mov [rsp + {}], rax\n", (self.stack_ptr - var_ptr) * 8));
    }

    fn gen_exit(&mut self, exit_stmt: &NodeStmtExit) {
        self.gen_expression(&exit_stmt.expression);

        self.asm.push_str("    mov rax, 60\n");
        self.pop("rdi");

        self.asm.push_str("    syscall\n");
    }
    
    fn gen_putchar(&mut self, putchar_stmt: &NodeStmtPutChar) {
        self.gen_expression(&putchar_stmt.expression);

        self.asm.push_str("    mov rax, 1\n");
        self.asm.push_str("    mov edi, 1\n");
        self.asm.push_str("    mov rsi, rsp\n");
        self.asm.push_str("    mov rdx, 1\n");

        self.asm.push_str("    syscall\n");
        self.asm.push_str("    add rsp, 8\n");
    }

    fn gen_expression(&mut self, expr: &MathValue) {
        match expr {
            MathValue::Integer(integer) => self.push(&integer.info),
            MathValue::Identifier(ident) => self.push_var_value(&ident),

            MathValue::Operation(oper) => {
                match oper.borrow() {
                    OperationType::Add(add) => {
                        self.gen_expression(&add.value_1);
                        self.gen_expression(&add.value_2);

                        self.pop("rax");
                        self.pop("rdi");

                        self.asm.push_str("    add rax, rdi\n");

                        self.push("rax");
                    },

                    OperationType::Sub(sub) => {
                        self.gen_expression(&sub.value_1);
                        self.gen_expression(&sub.value_2);

                        self.pop("rax");
                        self.pop("rdi");

                        self.asm.push_str("    sub rdi, rax\n");

                        self.push("rdi");
                    },
                    
                    OperationType::Mult(mult) => {
                        // i think this works, multiplication is strange though
                        self.gen_expression(&mult.value_1);
                        self.gen_expression(&mult.value_2);

                        self.pop("rax");
                        self.pop("rdi");

                        self.asm.push_str("    mul rdi\n");

                        self.push("rax");
                    },
                    
                    OperationType::Div(div) => {
                        self.gen_expression(&div.value_1);
                        self.gen_expression(&div.value_2);

                        self.asm.push_str("    xor rdx, rdx\n");
                        
                        // different order because division is the arg / rax i think
                        self.pop("rbx");
                        self.pop("rax");

                        self.asm.push_str("    idiv rbx\n");

                        self.push("rax");
                    },
                }
            },

           _ => { exit_message("Could not generate asm from expression"); return; }
        }
    }


    fn push(&mut self, reg_or_lit: &str) {
        self.stack_ptr += 1;
        self.asm.push_str(&format!("    push {}\n", reg_or_lit));
    }

    fn pop(&mut self, reg: &str) {
        self.stack_ptr -= 1;
        self.asm.push_str(&format!("    pop {}\n", reg));
    }

    fn var_declared(&self, identifier: &Token) -> bool {
        self.variables.contains_key(&identifier.info)
    }

    fn get_var_ptr(&self, identifier: &Token) -> isize {
        let Some(value) = self.variables.get_key_value(&identifier.info) else {
            panic!("Unknown identifier {}", identifier.info);
        };

        return *value.1;
    }

    fn push_var_value(&mut self, identifier: &Token) {
        let var_ptr = self.get_var_ptr(&identifier);
        self.asm.push_str(&format!("    mov rax, QWORD [rsp + {}]\n", (self.stack_ptr - var_ptr) * 8));
        
        self.push("rax");
    }
}
