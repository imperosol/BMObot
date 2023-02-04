use std::ops::{Deref, DerefMut};
use crate::game_logic::card::Card;
use rand::seq::SliceRandom;
use rand::thread_rng;

pub struct Deck {
    cards: Vec<Card>
}

impl Deck {
    pub fn new() -> Self {
        let mut cards = (0..=52_u8)
            .map(Card)
            .collect::<Vec<Card>>();
        cards.shuffle(&mut thread_rng());
        Self {
            cards
        }
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