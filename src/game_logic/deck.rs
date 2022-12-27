use std::ops::{Deref, DerefMut};
use crate::game_logic::card::Card;
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

    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }
}

impl Deref for Deck {
    type Target = Vec<Card>;

    fn deref(&self) -> &Self::Target {
        &self.cards
    }
}

impl DerefMut for Deck {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cards
    }
}