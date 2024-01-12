use std::collections::HashMap;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct Hand {
    kind: u8,
    cards: Vec<Card>,
    pub bid: u32,
}

impl Hand {
    /// Constructs a hand of cards
    ///
    /// * `cards`: the Cards in the hand
    /// * `bid`: the bid of the hand
    pub fn new(cards: Vec<Card>, bid: u32) -> Self {
        let kind = Hand::get_kind(&cards);

        Hand { cards, bid, kind }
    }

    /// Gets the kind of card (e.g. 3 of a kind, 2 pair)
    ///
    /// * `cards`: a vec of Card enums
    fn get_kind(cards: &[Card]) -> u8 {
        let mut found: HashMap<Card, u8> = HashMap::new();
        cards.iter().for_each(|card| {
            if !found.contains_key(card) {
                found.insert(card.clone(), 1);
            } else {
                *found.get_mut(card).unwrap() += 1;
            }
        });

        let max = *found.values().max().unwrap();
        match max {
            5 | 4 => max + 1,
            3 if found.values().len() == 2 => max + 1,
            3 => max,
            2 if found.values().len() == 3 => max, // two pair
            2 => max - 1,
            _ => 0,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub enum Card {
    Wild,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    J,
    Q,
    K,
    A,
}

/// parses hands from input
///
/// * `contents`: input string
pub fn parse_hands(contents: &str, wild_cards: bool) -> Vec<Hand> {
    contents
        .lines()
        .map(|line| {
            let mut split = line.split_whitespace();
            let cards = split.next().expect("No cards in hand.");
            let bid: u32 = split
                .next()
                .expect("No bid in hand.")
                .parse()
                .expect("couldn't parse bid.");
            let cards = parse_cards(cards, wild_cards);

            Hand::new(cards, bid)
        })
        .collect()
}

fn parse_cards(card_string: &str, wild_cards: bool) -> Vec<Card> {
    card_string
        .chars()
        .map(|card| match card {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'J' if wild_cards => Card::Wild,
            'J' => Card::J,
            'Q' => Card::Q,
            'K' => Card::K,
            'A' => Card::A,
            _ => panic!("unexpected card type found."),
        })
        .collect()
}
