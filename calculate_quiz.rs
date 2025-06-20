use rand::Rng; // 0.9.1

fn main() {
    let mut num_of_correct =0;
    
    while num_of_correct < 3 {
    
        //quiz_modeをランダムに1か2に決める
        let quiz_mode = rand::rng().gen_range(1..=2);
        
        match quiz_mode {
            1 => {

                let op_1 = rand::rng().random_range(0..100);
                let op_2 = rand::rng().random_range(0..100);
                
                println!("{} + {} = ??", op_1, op_2);
                
                println!("??の値を入力してください：");
                
                // ユーザーからの回答を保持する変数
                let mut ans_input = String::new();
                
                // 標準入力から一行取得してans_inputに代入する
                std::io::stdin().read_line(&mut ans_input).unwrap();
                
                // ans_inputからtrim()で改行を取り除き、parse()で整数型に変換する
                let ans_input = ans_input.trim().parse::<u32>().unwrap();
                
                println!("{}", ans_input);
                
                if dbg!(ans_input == op_1 + op_2) {
                    println!("正解！");
                    num_of_correct += 1;
                } else {
                    println!("不正解！");
                }
                
            }
            
            2 => {

                let op_1 = rand::rng().random_range(0..100);
                let op_2 = rand::rng().random_range(0..100);
                
                println!("{} - {} = ??", op_1, op_2);
                println!("??の値を入力してください：");
                let mut ans_input = String::new();
                std::io::stdin().read_line(&mut ans_input).unwrap();
                let ans_input = ans_input.trim().parse::<i32>().unwrap();
                println!("{}", ans_input);
                if dbg!(ans_input == op_1 - op_2) {
                    println!("正解！");
                    num_of_correct += 1;
                } else {
                    println!("不正解！");
                }
                
            }
            
            _ => unreachable!()
        }
    }
    println!("クリア！");
}