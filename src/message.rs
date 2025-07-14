use std::str::FromStr;

use serde::{Deserialize, Serialize};

/// Struct Message
/// Feito para ser passado entre threads,
/// contem um texto e um id que identifica quem mandou
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Message {
    pub text: String,
    pub sender_id: u16
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FromStrErr;

impl Message {

    /// Cria uma nova instância de Message
    pub fn new<T: ToString, U: Into<u16>>(text: T, id: U) -> Message {
        return Message {text: text.to_string(), sender_id: id.into() };
    }
}

// Formato de String retornada:
// "id texto"
impl ToString  for Message {
    fn to_string(&self) -> String {
        return format!("{} {}", self.sender_id, self.text);
    }
}

// Formato de String aceita:
// "id texto"
impl FromStr for Message {
    type Err = FromStrErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let text_id: &str;
        let text_content: &str;

        (text_id, text_content) = s.split_once(" ").ok_or(FromStrErr)?;

        match text_id.parse::<u16>() {
            Ok(sender_id) => return Ok(Message::new(text_content, sender_id)),
            Err(_) => return Err(FromStrErr),
        }
    }
}