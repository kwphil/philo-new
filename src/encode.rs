use std::{
    collections::HashMap,
    error::Error,
    sync::LazyLock,
};

use crate::statement::*;

static variables: LazyLock<HashMap<String, VariableDeclaration>> = LazyLock::new(|| HashMap::new());
static used_reg: LazyLock<Vec<String>> = LazyLock::new(|| Vec::new());

const registers64: [&str; 18] = [
    "rax", "rbx", "rcx", "rdx", "rsi", "rdi", /* rsp and rbp won't be used */
    "r8", "r9", "r10", "r11", "r12", "r13", "r14", "r15", "", "", "", ""
];

const registers32: [&str; 18] = [
    "eax", "ebx", "ecx", "edx", "esi", "edi",
    "r8d", "r9d", "r10d", "r11d", "r12d", "r13d", "r14d", "r15d", "", "", "", ""
];

const registers8: [&str; 18] = [
    "al", "ah", "bl", "bh", "ch", "cl", "dh", "dl",
    "sil", "dil", "r8b", "r9b", "r10b", "r11b", "r12b", "r13b", "r14b", "r15b"
];

pub fn binary_operation(ref left: &Box<Expression>, ref operator: &str, ref right: &Box<Expression>) -> String {
    let left_out = expression(left);
    let right_out = expression(right);
    
    return match *operator {
        "+" => format!("    add {}, {}\n", left_out, right_out),
        "-" => format!("    sub {}, {}\n", left_out, right_out),
        "*" => format!("    mul {}, {}\n", left_out, right_out),
        "/" => format!("    cqo\n    idiv {}, {}", left_out, right_out),
        "<" => {
            let r = unused_register(8);
            format!("    cmp {}, {}\n    
                         setl {}\n    
                         movzb {}, {}\n", 
                         left_out, right_out, r, r, right_out)
        },
        _ => unimplemented!("operation not implemented");
    }
}

fn get_variable(s: &str) -> Result<VariableDeclaration, Box<dyn Error>> {
    match variables.get(s) {
        Some(v) => return Ok((*v).clone()),
        None => return Err("Variable was not found!".into()),
    }
}

fn unused_register(bits: u16) -> String {
    let a = &match bits {
        64 => registers64,
        32 => registers32,
        8  => registers8,
    };

    for s in a {
        if !used_reg.contains(&(**s.to_string())) {
            return s.to_string();
        }
    }

    panic!("you shouldn't be here");
}

pub fn expression(ref expr: &Box<Expression>) -> String {
    match ***expr {
        Expression::Number(n) => return format!("${}", n),
        Expression::Identifier(s) => {
            let v = get_variable(&s)
                        .unwrap()
                        .var_type;

            if v.chars() // Checks if it is a codename
                .next()
                .unwrap()
                == '%'
            {
                return v;
            }
        },
        _ => unimplemented!("Unexpected expression"),
    }

    panic!("You shouldn't be here!");
}