use std::io::stdin;

fn main() {
    let mut memories = vec![0.0; 10];
    let mut prev_result: f64 = 0.0;

    for line in stdin().lines() {
        // 一行読み取って空なら終了
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }
        // 空白で分割
        let tokens: Vec<&str> = line.split(char::is_whitespace).collect();

        // メモリの値を更新
        let is_memory = tokens[0].starts_with("mem");
        if is_memory && tokens[0].ends_with('+') {
            add_and_print_memory(
                &mut memories,
                tokens[0],
                prev_result,
            );
            continue;
        } else if is_memory && tokens[0].ends_with('-') {
            add_and_print_memory(
                &mut memories,
                tokens[0],
                -prev_result,
            );
            continue;
        }

        // 式の計算
        let left = eval_token(tokens[0], memory);

        let right = eval_token(tokens[2], memory);
        
        let result = eval_expression(left, right, tokens[1]);

        print_output(result);

        prev_result = result;
    }
}

fn eval_token(token: &str, memory: Vec<f64>) -> f64 {
    if token.starts_with("mem") {
        let slot_index: usize = token[3..].parse().unwrap();
        memories[slot_index]
    } else {
        token.parse().unwrap()
    }
}

fn eval_expression(left: f64, right: f64, operator: &str) -> f64 {
    match operator {
        "+" => left + right,
        "-" => left - right,
        "*" => left * right,
        "/" => left / right,
        _ => {
            unreachable!();
        }
    }
}

fn print_output(result: f64) {
    println!(" => {}", result);
}

fn add_and_print_memory(
    memories: &mut Vec<f64>,
    token: &str,
    prev_result: f64
) {
    let slot_index: usize = token[3..token.len() - 1].parse().unwrap();
    memories[slot_index] += prev_result;
    print_output(memories[slot_index]);
}
