use std::io::stdin;

fn main() {
    let mut memory = Memory {
        slots: vec![],
    };
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
                &mut memory,
                tokens[0],
                prev_result,
            );
            continue;
        } else if is_memory && tokens[0].ends_with('-') {
            add_and_print_memory(
                &mut memory,
                tokens[0],
                -prev_result,
            );
            continue;
        }

        // 式の計算
        let left = eval_token(tokens[0], &memory);

        let right = eval_token(tokens[2], &memory);

        let result = eval_expression(left, right, tokens[1]);

        print_output(result);

        prev_result = result;
    }
}

struct Memory {
    slots: Vec<String, f64>,
}

fn eval_token(token: &str, memory: &Memory) -> f64 {
    if token.starts_with("mem") {
        let slot_index: usize = token[3..];
        // すべてのメモリを探索する
        for slot in &memory.slots {
            if slot.0 == slot_name {
                // メモリが見つかったので値を返す
                return slot.1;
            }
        }
        0.0
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
    memory: &mut Memory,
    token: &str,
    prev_result: f64
) {
    let slot_index: usize = token[3..token.len() - 1].parse().unwrap();
    // メモリの探索
    for slot in memory.slots.iter_mut() {
        if slot.o == slot_name {
            // メモリが見つかったので値を更新して終了
            slot.1 += prev_result;
            print_output(slot.1);
            return;
        }
    }

    // メモリが見つからなかったので新しいメモリを作成（最後の要素に追加する）
    memory.slots.push((slot_name.to_string(), prev_result));
    print_output(prev_result);
}
