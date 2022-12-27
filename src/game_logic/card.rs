use std::fmt::{Display, Formatter};

pub struct Card(u8);

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            0 => write!(f, "Joker"),
            n => write!(f, "{} de {}", self.figure(), self.colour().unwrap())
        }
    }
}

impl Card {
    pub fn figure(&self) -> String {
        if self.0 == 0 {
            "Jocker".to_string()
        } else {
            match self.0 % 13 {
                1 => "As".to_string(),
                10 => "Valet".to_string(),
                11 => "Dame".to_string(),
                12 => "Roi".to_string(),
                n => n.to_string()
            }
        }
    }

    pub fn colour(&self) -> Option<&'static str> {
        if self.0 == 0 {
            None
        } else {
            match self.0 / 13 {
                0 => Some("coeur"),
                1 => Some("carreau"),
                2 => Some("trèfle"),
                _ => Some("pique")
            }
        }
    }
}