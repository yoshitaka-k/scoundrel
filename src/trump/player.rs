use crate::constants::{MAX_HIT_POINT};

use crate::{Card};

/// プレイヤー
pub struct Player {
    name: String,
    hp: isize,
    equip: Option<Card>,
    selected: Vec<Card>,
}

impl Player {
    pub fn new(name: String) -> Self {
        Self {
            name,
            hp: 20,
            equip: None,
            selected: vec![],
        }
    }

    /// プレイヤー名取得
    pub fn get_name(&self) -> &String {
        &self.name
    }

    /// HPの取得
    pub fn get_hp(&self) -> isize {
        self.hp
    }

    /// HPの足し算
    pub fn update_hp(&mut self, hp: isize) {
        self.hp = self.hp + hp;
        if self.hp > MAX_HIT_POINT {
            self.hp = MAX_HIT_POINT;
        }
    }

    /// HPの引き算
    pub fn damage_hp(&mut self, damage: isize) {
        self.hp = self.hp + damage;
    }

    /// カードを装備する
    pub fn update_equip(&mut self, card: Option<Card> ) {
        self.equip = card;
    }

    /// 装備しているカード
    pub fn get_equip(&self) -> &Option<Card> {
        &self.equip
    }

    /// 装備しているカード
    pub fn get_equip_name(&self) -> String {
        if let Some(card) = &self.equip {
            return card.get_name();
        }
        String::from("None")
    }

    /// 手札のカードがない？
    pub fn equip_is_empty(&mut self) -> bool {
        self.equip.is_none()
    }

    /// 選択したカード
    pub fn add_selected(&mut self, card: Card) {
        self.selected.push(card);
    }

    /// 選択したカード参照
    pub fn get_selected(&self) -> &Vec<Card> {
        &self.selected
    }

    pub fn selected_len(&self) -> usize {
        self.selected.len()
    }

    /// 選択したカードリセット
    pub fn selected_clear(&mut self) {
        self.selected.clear();
    }
}
