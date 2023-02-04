use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use lazy_static::lazy_static;
use serenity::model::id::UserId;
use serenity::model::user::User;
use tokio::sync::Mutex;
use crate::game_logic::card::Card;

use crate::game_logic::player::{Player, PromoteError};

lazy_static! {
    pub static ref GAME: Mutex<Game> = Mutex::new(Game::new());
}

pub struct Game {
    pub is_over: bool,
    pub players: HashMap<UserId, Player>,
}

#[derive(Debug)]
pub struct NotPlayer(String);

impl Display for NotPlayer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Le joueur {} n'est pas dans la partie", self.0)
    }
}

impl Game {
    pub fn new() -> Self {
        Self {
            is_over: false,
            players: HashMap::new(),
        }
    }

    pub fn reset(&mut self) {
        self.players.clear();
    }

    pub fn add_player(&mut self, discord_user: &User) -> Result<(), &'static str> {
        let id = discord_user.id;
        if let Entry::Vacant(e) = self.players.entry(id) {
            e.insert(Player::new(discord_user));
            Ok(())
        } else {
            Err("Le joueur est déjà dans cette partie")
        }
    }

    pub fn player_exists(&self, discord_user: &User) -> bool {
        self.players.contains_key(&discord_user.id)
    }

    pub fn player_draw_cards(&mut self, discord_user: &User) -> Option<Card> {
        match self.players.get_mut(&discord_user.id) {
            None => None,
            Some(player) => player.draw_card()
        }
    }

    pub fn player_get_hand(&self, discord_user: &User) -> Result<Option<Vec<Card>>, NotPlayer> {
        match self.players.get(&discord_user.id) {
            Some(player) => {
                match player.hand.is_empty() {
                    true => Ok(None),
                    false => Ok(Some(player.hand.to_vec()))
                }
            }
            None => Err(NotPlayer(discord_user.name.clone()))
        }
    }

    pub fn player_set_hand(&mut self, discord_user: &User, cards: Vec<Card>) -> Result<(), NotPlayer> {
        match self.players.get_mut(&discord_user.id) {
            Some(player) => {
                player.set_hand(cards);
                Ok(())
            },
            None => Err(NotPlayer(discord_user.name.clone()))
        }
    }

    pub fn player_remaining_cards(&self, discord_user: &User) -> Result<usize, NotPlayer> {
        match self.players.get(&discord_user.id) {
            Some(player) => Ok(player.deck.len()),
            None => Err(NotPlayer(discord_user.name.clone()))
        }
    }

    pub fn promote_player(&mut self, discord_user: &User) -> Result<Result<(), PromoteError>, NotPlayer> {
        match self.players.get_mut(&discord_user.id) {
            Some(player) => Ok(player.promote()),
            None => Err(NotPlayer(discord_user.name.clone()))
        }
    }

    pub fn get_player_string(&self, discord_user: &User) -> Result<String, NotPlayer> {
        match self.players.get(&discord_user.id) {
            Some(player) => Ok(player.to_string()),
            None => Err(NotPlayer(discord_user.name.clone()))
        }
    }
}