use std::io::stdin;u78
use std::collections::{hash_map::Entry, HashMap};

struct Memory {
    slots: HashMap<String, f64>,
}

fn main() {
    let mut memory = Memory::new();
    let mut prev_result: f64 = 0.0;

    for line in stdin().lines() {
        // 一行読み取って空なら終了
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }
        // 空白で分割
        let tokens = Token::split(&line);
        
        // 式の評価
        match &tokens[0] {
            Token::MemoryPlus(memory_name) => {
                let memory_name = memory_name.to_string();
                let result = memory.add(memory_name, prev_result);
                print_output(result);
            }
            Token::MemoryMinus(memory_name) => {
                let memory_name = memory_name.to_string();
                let result = memory.add(memory_name, prev_result);
                print_output(result);
            }
            _ => {
                let left = Token::eval_token(tokens[0], &memory);
                let right = Token::eval_token(tokens[2], &memory);
                let result = Token::eval_expression(left, right, tokens[1]);
                print_output(result);
                prev_result = result;
            }
        };
    }
}

struct Memory {
    slots: HashMap<String, f64>,
}

impl Memory {
    fn new() -> Self {
        Self {
            slots: HashMap::new(),
        }
    }
    fn add(
        &mut self,
        slot_name: &str,
        prev_result: f64
    ) -> f64 {
        match self.slots.entry(slot_name) {
            Entry::Occupied(mut entry) => {
                // メモリが見つかったので値を更新して終了
                *entry.get_mut() += prev_result;
                *entry.get()
            }
            Entry::Vacant(entry) => {
                // メモリが見つからなかったので新しいメモリを作成（最後の要素に追加する）
                entry.insert(prev_result);
                prev_result
            }
        }
    }
    fn get(&self, slot_name: &str) -> f64 {
        self.slots.get(slot_name).copied().unwrap_or(0.0)
    }
}
#[derive(Debug, PartialEq)]
enum Token {
    Number(f64),
    MemoryRef(String),
    MemoryPlus(String),
    MemoryMinus(String),
    // 演算子
    Plus,
    Minus,
    Asterisk,
    Slash,
}

impl Token {
    fn parse(value: &str) -> Self {
        match value {
            "+" => Self::Plus,
            "-" => Self::Minus,
            "*" => Self::Asterisk,
            "/" => Self::Slash
            _ if value.starts_with("mem" => {
                let mut memory_name = value[3..].to_string();
                if value.ends_with('+') {
                    memory_name.pop();
                    Self::MemoryPlus(memory_name)
                } else if value.ends_with('-') {
                    memory_name.pop();
                    Self::MemoryMinus(memory_name)
                } else {
                    Self::MemoryRef(memory_name)
                }
            }
            _ => Self::Number(value.parse().unwrap()),
        }
    }

    fn split(text: &str) -> Vec<Self> {
        text.split(char::is_whitespace)
            .map(Self::parse)
            .collect();
    }

    fn eval_token(token: &Token, memory: &Memory) -> f64 {
        match token {
            Token::Number(value) => {
                // 数値の場合はそのまま値を返す
                *value
            }
            Token::MemoryRef(memory_name) => {
                memory.get(memory_name)
            }
            _ => {
                // 入力が正しいならここには来ない
                unreachable!();
            }
        }
    }

    fn eval_expression(left: f64, right: f64, operator: &Token) -> f64 {
        match operator {
            Token::Plus => left + right,
            Token::Minus => left - right,
            Token::Asterisk => left * right,
            Token::Slash => left / right,
            _ => {
                // 入力が正しいならここには来ない
                unreachable!()
            }
        }
    }
}

fn print_output(result: f64) {
    println!(" => {}", result);
}