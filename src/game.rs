use rand::{seq::SliceRandom, thread_rng};
use std::fmt;

pub const STARTING_CARDS: usize = 13;

pub struct Game {
    pub deck: Stockpile,
    pub discard: Discard,
    pub plrs: Vec<Player>,
    pub cur_plr: usize,
}

impl Game {
    pub fn new(num_players: usize, cards_per_player: usize) -> Self {
        let discard = Discard::new();
        let mut deck = Stockpile::new();
        deck.shuffle();

        let mut players = Vec::new();
        for _ in 0..num_players {
            let mut player = Player::new();
            deck.deal(&mut player, cards_per_player);
            players.push(player);
        }
        Self {
            deck,
            discard,
            plrs: players,
            cur_plr: 0,
        }
    }

    pub fn next_player(&mut self) {
        self.cur_plr += 1;
        self.cur_plr %= self.plrs.len();
    }

    pub fn play_cards(&mut self, card_inputs: &str) -> Result<(), String> {
        let card_strs = card_inputs
            .split(',')
            .map(|card| card.trim().to_ascii_uppercase())
            .collect::<Vec<String>>();

        let current_player = &mut self.plrs[self.cur_plr];
        let mut cards_to_play = Vec::new();

        for card_str in card_strs {
            match current_player
                .cards
                .iter()
                .position(|card| format!("{}", card) == card_str)
            {
                Some(index) => cards_to_play.push(current_player.cards.remove(index)),
                None => return Err(format!("Card not found in hand: {}", card_str)),
            }
        }

        // Logic to add the cards to the appropriate meld or layoff
        // ...

        Ok(())
    }

    pub fn discard_card(&mut self, card_input: &str) -> Result<(), String> {
        let card_str = card_input.trim().to_ascii_uppercase();
        let current_player = &mut self.plrs[self.cur_plr];

        if let Some(index) = current_player
            .cards
            .iter()
            .position(|card| format!("{}", card) == card_str)
        {
            let card = current_player.cards.remove(index);
            self.discard.cards.push(card);
            Ok(())
        } else {
            Err(format!("Card not found in hand: {}", card_str))
        }
    }

    pub fn is_round_over(&self) -> bool {
        self.deck.cards.is_empty() || self.plrs.iter().any(|player| player.cards.is_empty())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Suit {
    Diamonds,
    Clubs,
    Spades,
    Hearts,
}

#[derive(Debug, Clone, Copy)]
pub enum Rank {
    Number(u8),
    Jack,
    Queen,
    King,
    Ace,
}

impl Rank {
    pub fn value(&self) -> u32 {
        match self {
            Rank::Number(n) => *n as u32,
            Rank::Ace => 15,
            _ => 10,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.rank.value() == other.rank.value()
    }
}

impl Card {
    pub fn new(suit: Suit, rank: Rank) -> Self {
        Card { suit, rank }
    }

    pub fn value(&self) -> u32 {
        self.rank.value()
    }
}

#[derive(Clone)]
pub struct Player {
    pub cards: Vec<Card>,
    pub melds: Vec<Card>,
    pub value: u32,
}

impl Player {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            cards: vec![],
            melds: vec![],
            value: 0,
        }
    }

    pub fn draw(&mut self, stockpile: &mut Stockpile) {
        self.cards.push(stockpile.cards.pop().unwrap());
    }

    pub fn play(&mut self, cards: Vec<Card>) {
        for card in cards {
            let index = self
                .cards
                .iter()
                .position(|&c| c == card)
                .expect("Card not found in hand");
            self.melds.push(self.cards.remove(index));
        }
    }

    pub fn discard(&mut self, card: Card) {
        self.cards.push(card);
    }
}

#[derive(Clone)]
pub struct Stockpile {
    pub cards: Vec<Card>,
}

impl Stockpile {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let mut deck = vec![];
        let suits = [Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades];
        let ranks = [Rank::Ace, Rank::Jack, Rank::Queen, Rank::King];

        for &suit in &suits {
            for number in 2..=10 {
                deck.push(Card::new(suit, Rank::Number(number)))
            }
            for &rank in &ranks {
                deck.push(Card::new(suit, rank))
            }
        }

        Self { cards: deck }
    }

    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    pub fn deal(&mut self, player: &mut Player, num_cards: usize) {
        for _ in 0..num_cards {
            player.draw(self);
        }
    }
}

pub struct Discard {
    pub cards: Vec<Card>,
}

impl Discard {
    fn new() -> Self {
        Self { cards: vec![] }
    }
}

pub struct CardVec<'a>(pub &'a Vec<Card>);

impl<'a> fmt::Display for CardVec<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for card in self.0 {
            write!(f, "{} | ", card)?;
        }
        Ok(())
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rank_first_letter = match self.rank {
            Rank::Number(n) => n.to_string().chars().next().unwrap(),
            Rank::Jack => 'J',
            Rank::Queen => 'Q',
            Rank::King => 'K',
            Rank::Ace => 'A',
        };

        let suit_first_letter = match self.suit {
            Suit::Diamonds => 'D',
            Suit::Clubs => 'C',
            Suit::Spades => 'S',
            Suit::Hearts => 'H',
        };

        write!(f, "{}{}", rank_first_letter, suit_first_letter)
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Rank::Number(n) => write!(f, "{}", n),
            _ => write!(f, "{:?}", self),
        }
    }
}
