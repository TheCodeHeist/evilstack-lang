use crate::{
    error::Error,
    tokenizer::{Symbol, SymbolType},
};
use rand::Rng;
use std::{collections::HashMap, io};

#[derive(Debug, Clone)]
enum ConstType {
    Integer(i32),
    Float(f32),
    String(String),
}

#[derive(Debug)]
enum Instruction {
    // Stack operations
    Push(ConstType, String),
    Pop(String),
    Duplicate(String),
    Swap(String),

    // Binary operations
    Add(String),
    Sub(String),
    Mul(String),
    Div(String),
    IDiv(String),
    Mod(String),

    // Control flow
    Label(String, String),
    CmpInStack(String),
    Cmp(ConstType, String),
    Return(String),
    Jump(String, String),
    JumpEq(String, String),
    JumpNotEq(String, String),
    JumpGt(String, String),
    JumpLt(String, String),
    JumpGtEq(String, String),
    JumpLtEq(String, String),
    JumpZero(String, String),
    JumpNotZero(String, String),
    JumpNeg(String, String),
    Exit(String),

    // I/O
    Print(String),
    Read(String),

    // Type conversion
    AToI(String),
    IToA(String),
    IToF(String),
    FToI(String),

    // Built-in functions
    Random(String),
    Time(String),
}

pub struct Flags {
    pub zero: bool,
    pub negative: bool,
    pub equal: bool,
    pub not_equal: bool,
    pub greater_than: bool,
    pub less_than: bool,
    pub greater_than_or_equal: bool,
    pub less_than_or_equal: bool,
}

pub struct StackFkVM {
    stack: Vec<ConstType>,
    ip: usize,
    symbols: Vec<Symbol>,
    program: Vec<Instruction>,
    labels: HashMap<String, usize>,
    flags: Flags,
    return_stack: Vec<usize>,
}

impl StackFkVM {
    pub fn new(symbols: Vec<Symbol>) -> StackFkVM {
        StackFkVM {
            stack: Vec::new(),
            ip: 0,
            symbols,
            program: Vec::new(),
            labels: HashMap::new(),
            flags: Flags {
                zero: false,
                negative: false,
                equal: false,
                not_equal: false,
                greater_than: false,
                less_than: false,
                greater_than_or_equal: false,
                less_than_or_equal: false,
            },
            return_stack: Vec::new(),
        }
    }

    pub fn execute(&mut self) {
        self.compile();
        self.analyze_labels();

        // println!("Instructions:");
        // for instruction in &self.program {
        //     println!("{:?}", instruction);
        // }
        // println!("");
        // println!("Labels:");
        // for (label, ip) in &self.labels {
        //     println!("{}: {}", label, ip);
        // }
        // println!("");

        while self.ip < self.program.len() {
            // println!("IP: {}", self.ip);
            // println!("Stack: {:?}", self.stack);
            // println!("Return stack: {:?}", self.return_stack);

            match self.program[self.ip] {
                Instruction::Push(ref value, ref pos) => match value {
                    ConstType::Integer(i) => {
                        self.stack.push(ConstType::Integer(*i));
                        // println!("[DEBUG] Pushed {:?}", value);
                    }
                    ConstType::Float(f) => {
                        self.stack.push(ConstType::Float(*f));
                        // println!("[DEBUG] Pushed {:?}", value);
                    }
                    ConstType::String(ref s) => {
                        self.stack.push(ConstType::String(s.clone()));
                        // println!("[DEBUG] Pushed {:?}", value);
                    }
                },
                Instruction::Pop(ref pos) => {
                    if self.stack.is_empty() {
                        // panic!("Cannot pop from an empty stack");
                        Error::new("Cannot pop from an empty stack", pos.clone()).print();
                        return;
                    }
                    self.stack.pop();
                }
                Instruction::Duplicate(ref pos) => {
                    if self.stack.is_empty() {
                        // panic!("Cannot duplicate from an empty stack");
                        Error::new("Cannot duplicate from an empty stack", pos.clone()).print();
                        return;
                    }

                    let value = self.stack.pop().unwrap();
                    self.stack.push(value.clone());
                    self.stack.push(value);
                }
                Instruction::Swap(ref pos) => {
                    if self.stack.len() < 2 {
                        // panic!("Not enough operands for SWAP instruction");
                        Error::new("Not enough operands for SWAP instruction", pos.clone()).print();
                        return;
                    }

                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();

                    self.stack.push(a);
                    self.stack.push(b);
                }
                Instruction::Add(ref pos) => {
                    if self.stack.len() < 2 {
                        // panic!("Not enough operands for ADD instruction");
                        Error::new("Not enough operands for ADD instruction", pos.clone()).print();
                        return;
                    }

                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();

                    match (a, b) {
                        (ConstType::Integer(a), ConstType::Integer(b)) => {
                            self.stack.push(ConstType::Integer(a + b));
                        }
                        (ConstType::Float(a), ConstType::Float(b)) => {
                            self.stack.push(ConstType::Float(a + b));
                        }
                        (ConstType::Float(a), ConstType::Integer(b)) => {
                            self.stack.push(ConstType::Float(a + b as f32))
                        }
                        (ConstType::Integer(a), ConstType::Float(b)) => {
                            self.stack.push(ConstType::Float(a as f32 + b))
                        }
                        (ConstType::String(a), ConstType::String(b)) => {
                            self.stack.push(ConstType::String(b + &a));
                        }
                        _ => {
                            // panic!("Type mismatch for ADD instruction");
                            Error::new("Type mismatch for ADD instruction", pos.clone()).print();
                            return;
                        }
                    }
                }
                Instruction::Sub(ref pos) => {
                    if self.stack.len() < 2 {
                        // panic!("Not enough operands for SUB instruction");
                        Error::new("Not enough operands for SUB instruction", pos.clone()).print();
                        return;
                    }

                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();

                    match (a, b) {
                        (ConstType::Integer(a), ConstType::Integer(b)) => {
                            self.stack.push(ConstType::Integer(b - a));
                        }
                        (ConstType::Float(a), ConstType::Float(b)) => {
                            self.stack.push(ConstType::Float(b - a));
                        }
                        (ConstType::Float(a), ConstType::Integer(b)) => {
                            self.stack.push(ConstType::Float(b as f32 - a))
                        }
                        (ConstType::Integer(a), ConstType::Float(b)) => {
                            self.stack.push(ConstType::Float(a as f32 - b))
                        }
                        _ => {
                            // panic!("Type mismatch for SUB instruction");
                            Error::new("Type mismatch for SUB instruction", pos.clone()).print();
                            return;
                        }
                    }
                }
                Instruction::Mul(ref pos) => {
                    if self.stack.len() < 2 {
                        // panic!("Not enough operands for MUL instruction");
                        Error::new("Not enough operands for MUL instruction", pos.clone()).print();
                        return;
                    }

                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();

                    match (a, b) {
                        (ConstType::Integer(a), ConstType::Integer(b)) => {
                            self.stack.push(ConstType::Integer(a * b));
                        }
                        (ConstType::Float(a), ConstType::Float(b)) => {
                            self.stack.push(ConstType::Float(a * b));
                        }
                        (ConstType::Float(a), ConstType::Integer(b)) => {
                            self.stack.push(ConstType::Float(a * b as f32))
                        }
                        (ConstType::Integer(a), ConstType::Float(b)) => {
                            self.stack.push(ConstType::Float(a as f32 * b))
                        }
                        _ => {
                            // panic!("Type mismatch for MUL instruction");
                            Error::new("Type mismatch for MUL instruction", pos.clone()).print();
                            return;
                        }
                    }
                }
                Instruction::Div(ref pos) => {
                    if self.stack.len() < 2 {
                        // panic!("Not enough operands for DIV instruction");
                        Error::new("Not enough operands for DIV instruction", pos.clone()).print();
                        return;
                    }

                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();

                    match (a, b) {
                        (ConstType::Integer(a), ConstType::Integer(b)) => {
                            self.stack.push(ConstType::Integer(b / a));
                        }
                        (ConstType::Float(a), ConstType::Float(b)) => {
                            self.stack.push(ConstType::Float(b / a));
                        }
                        (ConstType::Float(a), ConstType::Integer(b)) => {
                            self.stack.push(ConstType::Float(b as f32 / a))
                        }
                        (ConstType::Integer(a), ConstType::Float(b)) => {
                            self.stack.push(ConstType::Float(b / a as f32))
                        }
                        _ => {
                            // panic!("Type mismatch for DIV instruction");
                            Error::new("Type mismatch for DIV instruction", pos.clone()).print();
                            return;
                        }
                    }
                }
                Instruction::IDiv(ref pos) => {
                    if self.stack.len() < 2 {
                        // panic!("Not enough operands for IDIV instruction");
                        Error::new("Not enough operands for IDIV instruction", pos.clone()).print();
                        return;
                    }

                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();

                    match (a, b) {
                        (ConstType::Integer(a), ConstType::Integer(b)) => {
                            self.stack.push(ConstType::Integer(b / a));
                        }
                        (ConstType::Float(a), ConstType::Float(b)) => {
                            self.stack.push(ConstType::Integer((b / a).floor() as i32));
                        }
                        (ConstType::Float(a), ConstType::Integer(b)) => self
                            .stack
                            .push(ConstType::Integer((b as f32 / a).floor() as i32)),
                        (ConstType::Integer(a), ConstType::Float(b)) => self
                            .stack
                            .push(ConstType::Integer((b / a as f32).floor() as i32)),
                        _ => {
                            // panic!("Type mismatch for IDIV instruction");
                            Error::new("Type mismatch for IDIV instruction", pos.clone()).print();
                            return;
                        }
                    }
                }
                Instruction::Mod(ref pos) => {
                    if self.stack.len() < 2 {
                        // panic!("Not enough operands for MOD instruction");
                        Error::new("Not enough operands for MOD instruction", pos.clone()).print();
                        return;
                    }

                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();

                    match (a, b) {
                        (ConstType::Integer(a), ConstType::Integer(b)) => {
                            self.stack.push(ConstType::Integer(b % a));
                        }
                        (ConstType::Float(a), ConstType::Float(b)) => {
                            self.stack.push(ConstType::Float(b % a));
                        }
                        (ConstType::Float(a), ConstType::Integer(b)) => {
                            self.stack.push(ConstType::Float(b as f32 % a))
                        }
                        (ConstType::Integer(a), ConstType::Float(b)) => {
                            self.stack.push(ConstType::Float(b % a as f32))
                        }
                        _ => {
                            // panic!("Type mismatch for MOD instruction");
                            Error::new("Type mismatch for MOD instruction", pos.clone()).print();
                            return;
                        }
                    }
                }
                Instruction::Print(ref pos) => {
                    if self.stack.is_empty() {
                        // panic!("Cannot print from an empty stack");
                        Error::new("Cannot print from an empty stack", pos.clone()).print();
                        return;
                    }

                    let value = self.stack.pop().unwrap();
                    match value {
                        ConstType::Integer(i) => {
                            println!("{}", i);
                        }
                        ConstType::Float(f) => {
                            println!("{}", f);
                        }
                        ConstType::String(s) => {
                            println!("{}", s);
                        }
                    }
                }
                Instruction::Read(ref pos) => {
                    let mut input = String::new();
                    match io::stdin().read_line(&mut input) {
                        Ok(_) => {
                            self.stack.push(ConstType::String(input.trim().to_string()));
                        }
                        Err(_) => {
                            // panic!("Failed to read input");
                            Error::new("Failed to read input", pos.clone()).print();
                            return;
                        }
                    }
                }
                Instruction::AToI(ref pos) => {
                    if self.stack.is_empty() {
                        // panic!("Cannot convert emptiness to an integer!");
                        Error::new("Cannot convert emptiness to an integer!", pos.clone()).print();
                        return;
                    }

                    let value = self.stack.pop().unwrap();
                    match value {
                        ConstType::String(s) => match s.parse::<i32>() {
                            Ok(i) => {
                                self.stack.push(ConstType::Integer(i));
                            }
                            Err(_) => {
                                // panic!("Invalid integer: {}", s);
                                Error::new(&format!("Invalid integer: {}", s), pos.clone()).print();
                                return;
                            }
                        },
                        _ => {
                            // panic!("Type mismatch for ATOI instruction");
                            Error::new("Type mismatch for ATOI instruction", pos.clone()).print();
                            return;
                        }
                    }
                }
                Instruction::FToI(ref pos) => {
                    if self.stack.is_empty() {
                        // panic!("Cannot convert emptiness to an integer!");
                        Error::new("Cannot convert emptiness to an integer!", pos.clone()).print();
                        return;
                    }

                    let value = self.stack.pop().unwrap();
                    match value {
                        ConstType::Float(f) => {
                            self.stack.push(ConstType::Integer(f as i32));
                        }
                        _ => {
                            // panic!("Type mismatch for FTOI instruction");
                            Error::new("Type mismatch for FTOI instruction", pos.clone()).print();
                            return;
                        }
                    }
                }
                Instruction::Jump(ref label, ref pos) => match self.labels.get(label) {
                    Some(ip) => {
                        self.return_stack.push(self.ip + 1);
                        self.ip = *ip;
                    }
                    None => {
                        // panic!("Unknown label: {}", label);
                        Error::new(&format!("Unknown label: {}", label), pos.clone()).print();
                        return;
                    }
                },
                Instruction::JumpEq(ref label, ref pos) => {
                    if self.flags.equal {
                        match self.labels.get(label) {
                            Some(ip) => {
                                self.return_stack.push(self.ip + 1);
                                self.ip = *ip;
                            }
                            None => {
                                // panic!("Unknown label: {}", label);
                                Error::new(&format!("Unknown label: {}", label), pos.clone())
                                    .print();
                            }
                        }
                    }
                }
                Instruction::JumpNotEq(ref label, ref pos) => {
                    if self.flags.not_equal {
                        match self.labels.get(label) {
                            Some(ip) => {
                                self.return_stack.push(self.ip + 1);
                                self.ip = *ip;
                            }
                            None => {
                                // panic!("Unknown label: {}", label);
                                Error::new(&format!("Unknown label: {}", label), pos.clone())
                                    .print();
                                return;
                            }
                        }
                    }
                }
                Instruction::JumpGt(ref label, ref pos) => {
                    if self.flags.greater_than {
                        match self.labels.get(label) {
                            Some(ip) => {
                                self.return_stack.push(self.ip + 1);
                                self.ip = *ip;
                            }
                            None => {
                                // panic!("Unknown label: {}", label);
                                Error::new(&format!("Unknown label: {}", label), pos.clone())
                                    .print();
                                return;
                            }
                        }
                    }
                }
                Instruction::JumpLt(ref label, ref pos) => {
                    if self.flags.less_than {
                        match self.labels.get(label) {
                            Some(ip) => {
                                self.return_stack.push(self.ip + 1);
                                self.ip = *ip;
                            }
                            None => {
                                // panic!("Unknown label: {}", label);
                                Error::new(&format!("Unknown label: {}", label), pos.clone())
                                    .print();
                                return;
                            }
                        }
                    }
                }
                Instruction::JumpGtEq(ref label, ref pos) => {
                    if self.flags.greater_than_or_equal {
                        match self.labels.get(label) {
                            Some(ip) => {
                                self.return_stack.push(self.ip + 1);
                                self.ip = *ip;
                            }
                            None => {
                                // panic!("Unknown label: {}", label);
                                Error::new(&format!("Unknown label: {}", label), pos.clone())
                                    .print();
                                return;
                            }
                        }
                    }
                }
                Instruction::JumpLtEq(ref label, ref pos) => {
                    if self.flags.less_than_or_equal {
                        match self.labels.get(label) {
                            Some(ip) => {
                                self.return_stack.push(self.ip + 1);
                                self.ip = *ip;
                            }
                            None => {
                                // panic!("Unknown label: {}", label);
                                Error::new(&format!("Unknown label: {}", label), pos.clone())
                                    .print();
                                return;
                            }
                        }
                    }
                }
                Instruction::JumpZero(ref label, ref pos) => {
                    if self.flags.zero {
                        match self.labels.get(label) {
                            Some(ip) => {
                                self.return_stack.push(self.ip + 1);
                                self.ip = *ip;
                            }
                            None => {
                                // panic!("Unknown label: {}", label);
                                Error::new(&format!("Unknown label: {}", label), pos.clone())
                                    .print();
                                return;
                            }
                        }
                    }
                }
                Instruction::JumpNotZero(ref label, ref pos) => {
                    if !self.flags.zero {
                        match self.labels.get(label) {
                            Some(ip) => {
                                self.return_stack.push(self.ip + 1);
                                self.ip = *ip;
                            }
                            None => {
                                // panic!("Unknown label: {}", label);
                                Error::new(&format!("Unknown label: {}", label), pos.clone())
                                    .print();
                            }
                        }
                    }
                }
                Instruction::JumpNeg(ref label, ref pos) => {
                    if self.flags.negative {
                        match self.labels.get(label) {
                            Some(ip) => {
                                self.return_stack.push(self.ip + 1);
                                self.ip = *ip;
                            }
                            None => {
                                // panic!("Unknown label: {}", label);
                                Error::new(&format!("Unknown label: {}", label), pos.clone())
                                    .print();
                            }
                        }
                    }
                }
                Instruction::Return(ref pos) => {
                    if let Some(ip) = self.return_stack.pop() {
                        self.ip = ip - 1;
                    } else {
                        // panic!("Cannot return anything from the main function!");
                        Error::new(
                            "Cannot return anything from the main function!",
                            pos.clone(),
                        )
                        .print();
                        return;
                    }
                }
                Instruction::Exit(ref pos) => {
                    return;
                }
                Instruction::Cmp(ref value, ref pos) => {
                    let a = self.stack.pop().unwrap();

                    match (&a, value) {
                        (ConstType::Integer(a), ConstType::Integer(b)) => {
                            self.flags.zero = *a == *b;
                            self.flags.negative = *a < *b;
                            self.flags.equal = *a == *b;
                            self.flags.not_equal = *a != *b;
                            self.flags.greater_than = *a > *b;
                            self.flags.less_than = *a < *b;
                            self.flags.greater_than_or_equal = *a >= *b;
                            self.flags.less_than_or_equal = *a <= *b;
                        }
                        (ConstType::Float(a), ConstType::Float(b)) => {
                            self.flags.zero = *a == *b;
                            self.flags.negative = *a < *b;
                            self.flags.equal = *a == *b;
                            self.flags.not_equal = *a != *b;
                            self.flags.greater_than = *a > *b;
                            self.flags.less_than = *a < *b;
                            self.flags.greater_than_or_equal = *a >= *b;
                            self.flags.less_than_or_equal = *a <= *b;
                        }
                        (ConstType::String(a), ConstType::String(b)) => {
                            self.flags.zero = *a == *b;
                            self.flags.negative = *a < *b;
                            self.flags.equal = *a == *b;
                            self.flags.not_equal = *a != *b;
                            self.flags.greater_than = *a > *b;
                            self.flags.less_than = *a < *b;
                            self.flags.greater_than_or_equal = *a >= *b;
                            self.flags.less_than_or_equal = *a <= *b;
                        }
                        _ => {
                            // panic!("Type mismatch for CMP instruction");
                            Error::new("Type mismatch for CMP instruction", pos.clone()).print();
                            return;
                        }
                    }

                    self.stack.push(a);
                }
                Instruction::CmpInStack(ref pos) => {
                    if self.stack.len() < 2 {
                        // panic!("Not enough operands for SCMP instruction");
                        Error::new("Not enough operands for SCMP instruction", pos.clone()).print();
                        return;
                    }

                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();

                    match (&a, &b) {
                        (ConstType::Integer(a), ConstType::Integer(b)) => {
                            self.flags.zero = *a == *b;
                            self.flags.negative = *a < *b;
                            self.flags.equal = *a == *b;
                            self.flags.not_equal = *a != *b;
                            self.flags.greater_than = *a > *b;
                            self.flags.less_than = *a < *b;
                            self.flags.greater_than_or_equal = *a >= *b;
                            self.flags.less_than_or_equal = *a <= *b;
                        }
                        (ConstType::Float(a), ConstType::Float(b)) => {
                            self.flags.zero = *a == *b;
                            self.flags.negative = *a < *b;
                            self.flags.equal = *a == *b;
                            self.flags.not_equal = *a != *b;
                            self.flags.greater_than = *a > *b;
                            self.flags.less_than = *a < *b;
                            self.flags.greater_than_or_equal = *a >= *b;
                            self.flags.less_than_or_equal = *a <= *b;
                        }
                        (ConstType::String(a), ConstType::String(b)) => {
                            self.flags.zero = *a == *b;
                            self.flags.negative = *a < *b;
                            self.flags.equal = *a == *b;
                            self.flags.not_equal = *a != *b;
                            self.flags.greater_than = *a > *b;
                            self.flags.less_than = *a < *b;
                            self.flags.greater_than_or_equal = *a >= *b;
                            self.flags.less_than_or_equal = *a <= *b;
                        }
                        _ => {
                            // panic!("Type mismatch for SCMP instruction");
                            Error::new("Type mismatch for SCMP instruction", pos.clone()).print();
                            return;
                        }
                    }

                    self.stack.push(b);
                    self.stack.push(a);
                }
                Instruction::Label(_, ref pos) => {}
                Instruction::Random(ref pos) => {
                    let mut rng = rand::thread_rng();
                    let random_number = rng.gen_range(0.0..1.0);
                    self.stack.push(ConstType::Float(random_number));
                }
                _ => {
                    // panic!("Unimplemented instruction: {:?}", self.program[self.ip]);
                    if let Instruction::Label(_, _) = self.program[self.ip] {
                    } else {
                        Error::new("Unimplemented instruction", "".to_string()).print();
                    }
                }
            }

            self.ip += 1;
        }
    }

    fn compile(&mut self) {
        let mut arg_required = false;
        let mut arg_required_by = String::new();

        for symbol in &self.symbols {
            let pos = format!(
                "{}:{}",
                symbol.line_number as i32, symbol.column_number as i32
            );
            // println!("{}, {}", symbol.line_number, symbol.column_number);
            match symbol.symbol_type {
                SymbolType::Instruction => {
                    if arg_required {
                        // panic!("Missing argument for instruction: {}", arg_required_by);
                        Error::new(
                            &format!("Missing argument for instruction: {}", arg_required_by),
                            pos.clone(),
                        )
                        .print();
                        return;
                    }

                    match symbol.value.as_str() {
                        "push" => {
                            arg_required = true;
                            arg_required_by = String::from("push");
                        }
                        "pop" => {
                            self.program.push(Instruction::Pop(pos.clone()));
                        }
                        "add" => {
                            self.program.push(Instruction::Add(pos.clone()));
                        }
                        "sub" => {
                            self.program.push(Instruction::Sub(pos.clone()));
                        }
                        "mul" => {
                            self.program.push(Instruction::Mul(pos.clone()));
                        }
                        "div" => {
                            self.program.push(Instruction::Div(pos.clone()));
                        }
                        "idiv" => {
                            self.program.push(Instruction::IDiv(pos.clone()));
                        }
                        "mod" => {
                            self.program.push(Instruction::Mod(pos.clone()));
                        }
                        "print" => {
                            self.program.push(Instruction::Print(pos.clone()));
                        }
                        "read" => {
                            self.program.push(Instruction::Read(pos.clone()));
                        }
                        "atoi" => {
                            self.program.push(Instruction::AToI(pos.clone()));
                        }
                        "jmp" => {
                            arg_required = true;
                            arg_required_by = String::from("jmp");
                        }
                        "jeq" => {
                            arg_required = true;
                            arg_required_by = String::from("jeq");
                        }
                        "jne" => {
                            arg_required = true;
                            arg_required_by = String::from("jne");
                        }
                        "jgt" => {
                            arg_required = true;
                            arg_required_by = String::from("jgt");
                        }
                        "jlt" => {
                            arg_required = true;
                            arg_required_by = String::from("jlt");
                        }
                        "jge" => {
                            arg_required = true;
                            arg_required_by = String::from("jge");
                        }
                        "jle" => {
                            arg_required = true;
                            arg_required_by = String::from("jle");
                        }
                        "jz" => {
                            arg_required = true;
                            arg_required_by = String::from("jz");
                        }
                        "jnz" => {
                            arg_required = true;
                            arg_required_by = String::from("jnz");
                        }
                        "jneg" => {
                            arg_required = true;
                            arg_required_by = String::from("jneg");
                        }
                        "cmp" => {
                            arg_required = true;
                            arg_required_by = String::from("cmp");
                        }
                        "scmp" => {
                            self.program.push(Instruction::CmpInStack(pos.clone()));
                        }
                        "exit" => {
                            self.program.push(Instruction::Exit(pos.clone()));
                        }
                        "ret" => {
                            self.program.push(Instruction::Return(pos.clone()));
                        }
                        "rand" => {
                            self.program.push(Instruction::Random(pos.clone()));
                        }
                        "time" => {
                            self.program.push(Instruction::Time(pos.clone()));
                        }
                        _ => {
                            // panic!("Unknown instruction: {}", symbol.value);
                            Error::new(
                                &format!("Unknown instruction: {}", symbol.value),
                                pos.clone(),
                            )
                            .print();
                            return;
                        }
                    }
                }
                SymbolType::String => {
                    if !arg_required {
                        // panic!("Unexpected string literal: {}", symbol.value);
                        Error::new(
                            &format!("Unexpected string literal: {}", symbol.value),
                            pos.clone(),
                        )
                        .print();
                        return;
                    }

                    match arg_required_by.as_str() {
                        "push" => {
                            self.program.push(Instruction::Push(
                                ConstType::String(symbol.value.clone()),
                                pos.clone(),
                            ));
                            arg_required = false;
                        }
                        "cmp" => {
                            let value = symbol.value.clone();
                            self.program
                                .push(Instruction::Cmp(ConstType::String(value), pos.clone()));
                            arg_required = false;
                        }
                        _ => {
                            // panic!("Unexpected string literal: {}", symbol.value);
                            Error::new(
                                &format!("Unexpected string literal: {}", symbol.value),
                                pos.clone(),
                            )
                            .print();
                            return;
                        }
                    }
                }
                SymbolType::Integer => {
                    if !arg_required {
                        // panic!("Unexpected integer literal: {}", symbol.value);
                        Error::new(
                            &format!("Unexpected integer literal: {}", symbol.value),
                            pos.clone(),
                        )
                        .print();
                        return;
                    }

                    match arg_required_by.as_str() {
                        "push" => {
                            let value = symbol.value.parse::<i32>().unwrap();
                            self.program
                                .push(Instruction::Push(ConstType::Integer(value), pos.clone()));
                            arg_required = false;
                        }
                        "cmp" => {
                            let value = symbol.value.parse::<i32>().unwrap();
                            self.program
                                .push(Instruction::Cmp(ConstType::Integer(value), pos.clone()));
                            arg_required = false;
                        }
                        _ => {
                            // panic!("Unexpected integer literal: {}", symbol.value);
                            Error::new(
                                &format!("Unexpected integer literal: {}", symbol.value),
                                pos.clone(),
                            )
                            .print();
                            return;
                        }
                    }
                }
                SymbolType::Float => {
                    if !arg_required {
                        // panic!("Unexpected float literal: {}", symbol.value);
                        Error::new(
                            &format!("Unexpected float literal: {}", symbol.value),
                            pos.clone(),
                        )
                        .print();
                        return;
                    }

                    match arg_required_by.as_str() {
                        "push" => {
                            let value = symbol.value.parse::<f32>().unwrap();
                            self.program
                                .push(Instruction::Push(ConstType::Float(value), pos.clone()));
                            arg_required = false;
                        }
                        "cmp" => {
                            let value = symbol.value.parse::<f32>().unwrap();
                            self.program
                                .push(Instruction::Cmp(ConstType::Float(value), pos.clone()));
                            arg_required = false;
                        }
                        _ => {
                            // panic!("Unexpected float literal: {}", symbol.value);
                            Error::new(
                                &format!("Unexpected float literal: {}", symbol.value),
                                pos.clone(),
                            )
                            .print();
                            return;
                        }
                    }
                }
                SymbolType::Label => {
                    if arg_required {
                        // panic!("Missing argument for instruction: {}", arg_required_by);
                        Error::new(
                            &format!("Missing argument for instruction: {}", arg_required_by),
                            pos.clone(),
                        )
                        .print();
                        return;
                    }

                    let label = &symbol.value[..symbol.value.len() - 1];
                    self.program
                        .push(Instruction::Label(label.to_string(), pos.clone()));
                }
                SymbolType::LabelReference => {
                    if !arg_required {
                        // panic!("Unexpected label reference: {}", symbol.value);
                        Error::new(
                            &format!("Unexpected label reference: {}", symbol.value),
                            pos.clone(),
                        )
                        .print();
                        return;
                    }

                    let label = &symbol.value[1..];

                    match arg_required_by.as_str() {
                        "jmp" => {
                            self.program
                                .push(Instruction::Jump(label.to_string(), pos.clone()));
                            arg_required = false;
                        }
                        "jeq" => {
                            self.program
                                .push(Instruction::JumpEq(label.to_string(), pos.clone()));
                            arg_required = false;
                        }
                        "jne" => {
                            self.program
                                .push(Instruction::JumpNotEq(label.to_string(), pos.clone()));
                            arg_required = false;
                        }
                        "jgt" => {
                            self.program
                                .push(Instruction::JumpGt(label.to_string(), pos.clone()));
                            arg_required = false;
                        }
                        "jlt" => {
                            self.program
                                .push(Instruction::JumpLt(label.to_string(), pos.clone()));
                            arg_required = false;
                        }
                        "jge" => {
                            self.program
                                .push(Instruction::JumpGtEq(label.to_string(), pos.clone()));
                            arg_required = false;
                        }
                        "jle" => {
                            self.program
                                .push(Instruction::JumpLtEq(label.to_string(), pos.clone()));
                            arg_required = false;
                        }
                        "jz" => {
                            self.program
                                .push(Instruction::JumpZero(label.to_string(), pos.clone()));
                            arg_required = false;
                        }
                        "jnz" => {
                            self.program
                                .push(Instruction::JumpNotZero(label.to_string(), pos.clone()));
                            arg_required = false;
                        }
                        "jneg" => {
                            self.program
                                .push(Instruction::JumpNeg(label.to_string(), pos.clone()));
                            arg_required = false;
                        }
                        _ => {
                            // panic!("Unexpected label reference: {}", symbol.value);
                            Error::new(
                                &format!("Unexpected label reference: {}", symbol.value),
                                pos.clone(),
                            )
                            .print();
                            return;
                        }
                    }
                }
            }
        }

        if arg_required {
            // panic!("Missing argument for instruction: {}", arg_required_by);
            Error::new(
                &format!("Missing argument for instruction: {}", arg_required_by),
                "".to_string(),
            )
            .print();
            return;
        }
    }

    fn analyze_labels(&mut self) {
        for (i, instruction) in self.program.iter().enumerate() {
            if let Instruction::Label(ref label, pos) = instruction {
                self.labels.insert(label.clone(), i);
            }
        }
    }
}
