use crate::deck::card::Card;
use rand::seq::SliceRandom;
use rand::thread_rng;

pub struct Deck {
    cards: Vec<Card>
}

impl Deck {
    pub fn new() -> Self {
        let mut cards = (0..54 as u8)
            .map(|i| Card(i))
            .collect::<Vec<Card>>();
        cards.shuffle(&mut thread_rng());
        Self {
            cards
        }
    }

    pub fn draw(&mut self, nb: u8) -> Vec<Card> {
        let mut res = Vec::with_capacity(nb as usize);
        for _ in 0..nb {
            match self.cards.pop() {
                Some(card) => res.push(card),
                None => return res
            }
        }
        res
    }
}