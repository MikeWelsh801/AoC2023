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
        let mut wilds = 0;
        cards.iter().for_each(|card| {
            if *card == Card::Wild {
                wilds += 1;
            } else if !found.contains_key(card) {
                found.insert(card.clone(), 1);
            } else {
                *found.get_mut(card).unwrap() += 1;
            }
        });

        let max = *found.values().max().unwrap_or(&0);
        let kind = match max {
            5 | 4 => max + 1,
            3 if found.values().len() == 2 && wilds == 0 => max + 1,
            3 => max,
            2 if found.values().len() == 3 && wilds == 0 => max, // two pair
            2 if found.values().len() == 2 && wilds == 1 => max, // two pair
            2 => max - 1,
            _ => 0,
        };
        if wilds == 0 {
            return kind;
        }
        Self::get_wilds(kind, wilds)
    }

    fn get_wilds(kind: u8, wilds: u8) -> u8 {
        match (kind, wilds) {
            (6, _) => panic!("too many cards!"), // 5 of a kind and some jacks
            (4, _) => panic!("full house and some jacks!"),
            (5, 1) | (0, 1) => kind + wilds,
            (3, _) | (2, 1) | (1, 1) | (0, 2) => kind + wilds + 1,
            (1, 2) | (1, 3) | (0, 3) => kind + wilds + 2,
            (0, 4) | (0, 5) => 6, // 0 pairs and 4 or 5 jacks (five of a kind)
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

/// Returns the total winnings in from a sorted list of hands.
///
/// * `hands`: A sorted list of hands
pub fn get_total_winings(hands: &[Hand]) -> u32 {
    hands
        .iter()
        .enumerate()
        .map(|(index, hand)| (index + 1) as u32 * hand.bid)
        .sum()
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
