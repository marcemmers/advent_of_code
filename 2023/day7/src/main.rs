use std::cmp::{Ordering, Reverse};
use std::fs;
use std::time::Instant;

const EXAMPLE_FILENAME: &'static str = "./src/example.txt";
const PUZZLE_FILENAME: &'static str = "./src/puzzle.txt";

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

fn hand_from_cards(cards: &str) -> HandType {
    let mut occurences = vec![0; 15];
    cards
        .chars()
        .for_each(|x| occurences[x.to_digit(16).unwrap() as usize] += 1);

    // println!("Occurences: {:?}", occurences);

    let jokers = occurences[0];
    occurences[0] = 0;

    occurences.sort_by_key(|w| Reverse(*w));

    return match occurences[0] + jokers {
        5 => HandType::FiveOfAKind,
        4 => HandType::FourOfAKind,
        3 if occurences[1] == 2 => HandType::FullHouse,
        3 => HandType::ThreeOfAKind,
        2 if occurences[1] == 2 => HandType::TwoPair,
        2 => HandType::OnePair,
        _ => HandType::HighCard,
    };
}

#[derive(Debug, Clone, Ord, Eq)]
struct Hand {
    cards: String,
    rank: HandType,
    bet: u32,
}

impl Hand {
    fn new(input: &str, has_joker: bool) -> Self {
        let (cards, bet) = input.split_once(" ").unwrap();
        let copy = cards
            .to_string()
            .replace("A", "E")
            .replace("K", "D")
            .replace("Q", "C")
            .replace("J", if has_joker { "0" } else { "B" })
            .replace("T", "A");
        Hand {
            rank: hand_from_cards(copy.as_str()),
            cards: copy,
            bet: bet.parse::<u32>().unwrap(),
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.rank != other.rank {
            return self.rank.partial_cmp(&other.rank);
        }
        return other.cards.partial_cmp(&self.cards);
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.rank == other.rank && self.cards == other.cards
    }
}

fn solve(filename: &str, has_joker: bool) {
    println!("Solving for file: {filename}");
    let input = fs::read_to_string(filename).expect("Should have been read");

    let lines = input.lines();

    let mut hands: Vec<Hand> = lines.map(|x| Hand::new(x, has_joker)).collect();

    hands.sort_unstable();

    let result = hands
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (idx, hand)| acc + (idx as u32 + 1) * hand.bet);

    // println!("Hands: {:?}", hands);
    println!("Result: {result}");
}

fn main() {
    let start = Instant::now();

    solve(EXAMPLE_FILENAME, false);
    solve(PUZZLE_FILENAME, false);

    println!("Solved 1 in {:?}\n\n", start.elapsed());
    let start = Instant::now();

    solve(EXAMPLE_FILENAME, true);
    solve(PUZZLE_FILENAME, true);

    println!("Solved 2 in {:?}", start.elapsed());
}
