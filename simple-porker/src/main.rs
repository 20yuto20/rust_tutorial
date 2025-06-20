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
    for _ in 0..11 {
        hand.push(deck.pop().unwrap());
    }

    //  手札を表示する
    println!("==== 手札 ====");
    for (i, card) in hand.iter().enumerate() {
        println!("{:}: {:?} {:}", i, card.suit, card.rank);
    }
    

    println!("{:?}", deck);
}

