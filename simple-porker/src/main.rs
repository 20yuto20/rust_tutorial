# [derive(Debug, Clone, Copy, PartialEq)]
enum Suit {
    Club,
    Diamond,
    Heart,
    Spade,
}

# [derive(Debug, Clone, Copy, PartialEq)]
struct Card {
    suit: Suit,
    rank: i32
}

use rand::seq::SliceRandom;

fn main() {
    // Vecの用意
    let mut deck: Vec<Card> = Vec::new();
    let suits = [Suit::Club, Suit::Diamond, Suit::Heart, Suit::Spade];

    // Deckを作成
    for suit in suits {
        for rank in 1..=13 {
            //Vecに追加
            deck.push(Card { suit, rank });
        }
    }

    // dockをシャッフル
    let mut rng = rand::thread_rng();
    deck.shuffle(&mut rng);

    // 手札用のVecを作成
    let mut hand: Vec<Card> = Vec::new();

    // 5枚引く
    for _ in 0..5 {
        hand.push(deck.pop().unwrap());
    }

    // 手札をソート
    hand.sort_by(|a, b| a.rank.cmp(&b.rank));

    //  手札を表示する
    println!("==== 手札 ====");
    for (i, card) in hand.iter().enumerate() {
        println!("{:}: {:?} {:}", i, card.suit, card.rank);
    }
    

    // println!("{:?}", deck);

    // 手札交換の実装
    println!("==== 手札交換 ====");
    println!("入れ替えたいカードの番号を入力してください（例；0 1 2 3 4）");

    // ユーザーからの入力を受け取る関数
    let mut input = String::new();

    // ユーザーからの入力を変数に書き込む
    std::io::stdin().read_line(&mut input).unwrap();

    // 選ばれたカードを山札から引いたカードで置き換える
    let numbers: Vec<usize> = input
        .split_whitespace() // 文字列を空白で分割 // 例："0 1 2 3 4" -> ["0", "1", "2", "3", "4"]
        .map(|x| x.parse().unwrap()) // 文字列を数値に変換 // 例：["0", "1", "2", "3", "4"] -> [0, 1, 2, 3, 4]
        .collect::<Vec<usize>>(); // 数値のベクターに変換 // 例：[0, 1, 2, 3, 4]

    // 与えられた数字の箇所をデッキから取り出したカードに置き換える
    for number in numbers {
        hand[number -1] = deck.pop().unwrap();
    }

    // 手札をソート
    hand.sort_by(|a, b| a.rank.cmp(&b.rank));

    //  手札を表示する
    println!("==== 手札 ====");
    for (i, card) in hand.iter().enumerate() {
        println!("{:}: {:?} {:}", i, card.suit, card.rank);
    }

    // フラッシュのチェック
    let suit = hand.first().unwrap().suit;
    let is_flush = hand.iter().all(|card| card.suit == suit);

    // ペア数の確認
    let mut count = 0;
    for i in 0..hand.len() - 1 {
        for j in i + 1..hand.len() {
            if hand[i].rank == hand[j].rank {
                count += 1;
            }
        }
    } 

    // ペア数の表示
    if is_flush {
        println!("フラッシュ！");   
    } else if count >= 3 {
        println!("スリーカード！");
    } else if count >= 2 {
        println!("ワンペア！");
    } else {
        println!("役なし...😭")
    }

}

