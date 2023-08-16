use crate::game_logic::card::Card;
use crate::game_logic::player::MagicLevel::Veteran;
use crate::game_logic::player::PromoteError::{AlreadyPromoted, BecomeInsane};
use rand::prelude::SliceRandom;
use rand::thread_rng;
use serde::{Deserialize, Serialize};
use serenity::model::user::User;
use std::fmt::{Display, Formatter};

#[derive(PartialEq, Copy, Clone, Debug, Serialize, Deserialize)]
pub enum MagicLevel {
    Beginner,
    Veteran,
}

pub enum PromoteError {
    AlreadyPromoted,
    BecomeInsane,
}

#[derive(Serialize, Deserialize)]
pub struct Player {
    pub deck: Vec<Card>,
    pub hand: Vec<Card>,
    pub magic_level: MagicLevel,
    pub is_sane: bool,
    #[serde(skip)]
    pub discord: User,
}

impl Player {
    pub fn new(discord_user: &User) -> Self {
        let mut deck = (0..=52_u8).map(Card).collect::<Vec<Card>>();
        deck.shuffle(&mut thread_rng());
        Player {
            deck,
            hand: Vec::new(),
            magic_level: MagicLevel::Beginner,
            is_sane: true,
            discord: discord_user.clone(),
        }
    }

    pub fn draw_card(&mut self) -> Option<Card> {
        let card = self.deck.pop();
        if card.is_none() {
            self.is_sane = false;
            println!("Le joueur devient fou");
        }
        card
    }

    pub fn promote(&mut self) -> Result<(), PromoteError> {
        if self.magic_level == Veteran {
            return Err(AlreadyPromoted);
        }
        if self.deck.len() <= 2 {
            self.is_sane = false;
            return Err(BecomeInsane);
        }
        self.magic_level = Veteran;
        self.hand.push(self.deck.pop().unwrap());
        self.hand.push(self.deck.pop().unwrap());
        Ok(())
    }

    pub fn set_hand(&mut self, cards: Vec<Card>) {
        self.hand = cards
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut msg = format!(
            "cartes dans le paquet : {}\nniveau de magie : {}\n",
            self.deck.len(),
            match self.magic_level {
                MagicLevel::Beginner => "débutant",
                MagicLevel::Veteran => "intermédiaire",
            }
        );
        if self.magic_level == Veteran {
            msg.push_str("Cartes en main : \n");
            for card in self.hand.iter() {
                msg.push_str(format!("\t- {}", card).as_str());
            }
        }
        write!(f, "{}", msg)
    }
}
