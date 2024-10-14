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
    pub post_asm: String,

    variables: HashMap<String, isize>,
    functions: Vec<String>,
}

impl CodeGen {
    pub fn new() -> CodeGen {
        CodeGen {
            stack_ptr: -1,

            asm: String::from("global _start\n_start:\n    push rsp\n"),
            post_asm: String::from("    mov rax, 60\n    mov rdi, 0\n    syscall\n"),

            variables: HashMap::new(),
            functions: vec!(),
        }
    }

    pub fn gen_output(&mut self, ast: &NodeProgram) -> String {
        self.generate(ast);
        let mut output = String::new();
        output.push_str(&self.asm);
        output.push_str(&self.post_asm);

        return output;
    }

    pub fn generate(&mut self, ast: &NodeProgram) {
        for stmt in &ast.statements {
            match stmt {
                NodeStatements::Declare(declare_stmt) => {
                    self.gen_declare(declare_stmt);
                },
                NodeStatements::Exit(exit_stmt) => {
                    self.gen_exit(exit_stmt);
                },
                NodeStatements::PutChar(putchar_stmt) => {
                    self.gen_putchar(putchar_stmt);
                }
                NodeStatements::Set(set_stmt) => {
                    self.gen_set(set_stmt);
                },
                NodeStatements::Function(func_stmt) => {
                    self.gen_function(func_stmt);
                }
                NodeStatements::FunctionCall(func_call_stmt) => {
                    self.gen_func_call(func_call_stmt);
                }
            }
        }
    }


    fn gen_func_call(&mut self, func_call_stmt: &NodeStmtFunctionCall) {
        self.asm.push_str(&format!("    call fn_{}\n", func_call_stmt.identifier.info));
    }


    fn gen_function(&mut self, func_stmt: &NodeStmtFunction) {
        self.functions.push(func_stmt.identifier.info.clone());

        let identifier: &str = &func_stmt.identifier.info;
        let mut assembly = format!("fn_{}:\n", identifier);

        let scope_asm = self.gen_scope(&func_stmt.scope);

        assembly.push_str(&scope_asm);
        assembly.push_str("    ret\n");


        self.post_asm.push_str(&assembly);
    }

    fn gen_scope(&mut self, program: &NodeProgram) -> String {
        let mut new_generator = CodeGen {
            stack_ptr: -1,
            asm: String::new(),
            post_asm: String::new(),
            variables: self.variables.clone(),
            functions: self.functions.clone(),
        };

        new_generator.generate(program);

        new_generator.asm.push_str("    ; fix stack pointer\n");
        new_generator.add_stack_pointer(new_generator.stack_ptr - self.stack_ptr);

        return new_generator.asm + &new_generator.post_asm;
    }

    fn gen_declare(&mut self, declare_stmt: &NodeStmtDeclare) {
        if self.var_declared(&declare_stmt.identifier) {
            exit_message(&format!("Variable {} has already been declared!", declare_stmt.identifier.info));
        }

        // insert into the variables hashmap
        self.variables.insert(declare_stmt.identifier.info.clone(), self.stack_ptr + 1);

        // comment
        self.asm.push_str(&format!("    ; declare variable {}\n", declare_stmt.identifier.info));

        if let Some(expression) = &declare_stmt.expression {
            self.asm.push_str("    ; initial value for variable\n");
            self.gen_expression(expression);
        } else {
            // we can just sub the stack pointer to allocate the space
            self.asm.push_str("    ; allocate space for variable\n");
            self.sub_stack_pointer(1);
        }

    }

    fn gen_set(&mut self, set_stmt: &NodeStmtSet) {
        if !self.var_declared(&set_stmt.identifier) {
            exit_message(&format!("Variable referenced before assignment {}", set_stmt.identifier.info));
            return;
        }

        self.asm.push_str("    ; setting a variable\n");
        self.gen_expression(&set_stmt.expression);
        self.asm.push_str("    ; value is at the top of the stack\n");
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
        self.asm.push_str("    ; put char\n");
        self.gen_expression(&putchar_stmt.expression);

        self.asm.push_str("    mov rax, 1\n");
        self.asm.push_str("    mov edi, 1\n");
        self.asm.push_str("    mov rsi, rsp\n");
        self.asm.push_str("    mov rdx, 1\n");

        self.asm.push_str("    syscall\n");
        self.add_stack_pointer(1);
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

    /// Allocates more space
    fn sub_stack_pointer(&mut self, amount: isize) {
        self.stack_ptr += amount;
        self.asm.push_str(&format!("    sub rsp, {}\n", amount * 8));
    }

    /// Un-allocates space
    fn add_stack_pointer(&mut self, amount: isize) {
        self.stack_ptr -= amount;
        self.asm.push_str(&format!("    add rsp, {}\n", amount * 8));
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
