use super::oper::*;

struct Assembly {
    asm: Vec<String>,
}

impl Into<String> for Assembly {
    fn into(self) -> String {
        self.build()
    }
}

impl Assembly {
    fn new() -> Assembly {
        Assembly { asm: Vec::new() }
    }

    fn build(&self) -> String {
        self.asm.join("\n")
    }

    fn add<S: Into<String>>(&mut self, string: S) {
        self.asm.push(string.into());
    }

    fn add_all<S: Into<String>>(&mut self, strings: Vec<S>) {
        for string in strings {
            self.asm.push(string.into());
        }
    }
}

pub struct Generator {
    asm: Assembly,
}

impl Generator {
    pub fn new() -> Generator {
        Generator { asm : Assembly::new() }
    }

    pub fn generate(&mut self, prog: Program) -> String {
        match prog {
            Program { functions, globals } => {
                for func in functions {
                    let g_func = self.generate_func(func);
                    self.asm.add(g_func);
                }
            }
        };
        self.asm.build()
    }

    fn generate_func(&mut self, func: Function) -> Assembly {
        let mut tmp_asm = Assembly::new();
        match func {
            Function { name, args, statements } => {
                tmp_asm.add(format!(".globl _{}", name));
                tmp_asm.add(format!("_{}:", name));

                let has_return: bool = statements.iter().any(|s| if let Statement::Return(_) = *s { true } else { false });
                for statement in statements {
                    tmp_asm.add(self.gen_statement(statement));
                }
                if !has_return {
                    tmp_asm.add("mov\trsp, rbp");
                    tmp_asm.add("pop rpb");
                    tmp_asm.add("ret\n");
                }
            }
        };
        tmp_asm
    }

    fn gen_statement(&mut self, s: Statement) -> Assembly {
        let mut tmp_asm = Assembly::new();
        match s {
            Statement::Return(exp) => {
                match exp {
                    Expression::Int(val) => {
                        tmp_asm.add(format!{"movl\t${}, %eax", val});
                        tmp_asm.add("ret\n");
                    },
                    _ => {}
                }
            },
            _ => {}
        };
        tmp_asm
    }
}