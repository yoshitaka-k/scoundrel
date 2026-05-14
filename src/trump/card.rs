use crate::constants::{
    ACE_RANK,

    SUIT_STR_HART,
    SUIT_STR_DIAMOND,
    SUIT_STR_CLOVER,
    SUIT_STR_SPADE,

    SUIT_ICON_HART,
    SUIT_ICON_DIAMOND,
    SUIT_ICON_CLOVER,
    SUIT_ICON_SPADE,
};

/// カードの情報
#[derive(Debug, Clone)]
pub struct Card {
    suit: String,
    rank: isize,
}

impl Card {
    pub fn new(suit: &str, rank: isize) -> Self {
        Self { suit: suit.to_string(), rank }
    }

    pub fn get_suit(&self) -> &String {
        &self.suit
    }

    pub fn get_rank(&self) -> isize {
        if self.rank == 1 {
            ACE_RANK
        } else {
            self.rank
        }
    }

    pub fn get_name(&self) -> String {
        let suit = match self.suit.as_str() {
            SUIT_STR_HART => SUIT_ICON_HART,
            SUIT_STR_DIAMOND => SUIT_ICON_DIAMOND,
            SUIT_STR_CLOVER => SUIT_ICON_CLOVER,
            SUIT_STR_SPADE => SUIT_ICON_SPADE,
            _ => "J",
        };

        format!("{}{}", suit, self.rank)
    }
}

impl std::fmt::Display for Card {
    /// スート・ランク表示
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_name())
    }
}
