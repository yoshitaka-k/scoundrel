use crate::constants::{
    ROOM_MAX_CARD,

    DEFAULT_SELECT_CARD,
    MIN_SELECT_CARD,
    MAX_SELECT_CARD,

    SUIT_STR_HART,
    SUIT_STR_DIAMOND,
    SUIT_STR_CLOVER,
    SUIT_STR_SPADE,
};
use crate::cli::{
    input::{input_usize_read_line},
    print_display::{
        potion_display,
        equip_display,
        battle_start_display,
        battle_result_display,
    },
    indicate::execute_with_spinner,
};
use crate::logic::shuffle::{
    double_cut,
    hindu_shuffle,
    riffle_shuffle,
    deal_shuffle,
    HinduParams,
    RiffleParams,
    DealParams,
};
use crate::trump::{Card, Deck, Field, Player};

pub struct GameSession();
impl GameSession {
    /// 体力無くなったか確認
    pub fn is_gameover(player: &Player) -> bool {
        player.get_hp() < 1
    }

    /// 山札を切る
    pub fn shuffle(player: &Player, cards: &mut Vec<Card>) {
            // Deck Shuffle.
        execute_with_spinner(
            &format!("Deck setup and {} a shuffle...", player.get_name()),
            &format!("Deck setup and {} a shuffle end.", player.get_name()),
        || {
            hindu_shuffle(cards, &HinduParams::default());
            riffle_shuffle(cards, &RiffleParams::default());
            deal_shuffle(cards, &DealParams::default());
            double_cut(cards);
        });
    }

    /// ルームカードの設定
    pub fn room_setup(deck: &mut Deck, field: &mut Field) -> bool {
        while field.get_room_len() < ROOM_MAX_CARD {
            if let Some(card) = deck.draw() {
                field.add_room_card(card);
            } else {
                return false;
            }
        }

        true
    }

    /// ルームカードの再設定
    pub fn re_room_setup(deck: &mut Deck, field: &mut Field) {
        println!("  Room card Re:Setup. ");

        let room = field.get_room();
        for card in room {
            deck.push(card.clone());
        }

        field.room_clear();
    }

    /// ルームカードの選択
    pub fn room_card_selected(field: &mut Field, is_skip: bool) -> Option<Card> {
        let mut min_num = MIN_SELECT_CARD;
        let mut max_num = MAX_SELECT_CARD;

        if field.get_room_len() < ROOM_MAX_CARD || is_skip {
            min_num = 1;
            max_num = field.get_room_len();
        }

        println!("Room card selected.");
        let index: usize = input_usize_read_line(
            &format!(
                "Input: {}-{}.  Default: {}",
                min_num,
                max_num,
                DEFAULT_SELECT_CARD
            ),
            DEFAULT_SELECT_CARD,
            min_num,
            max_num
        );

        if index == 0 {
            return None
        }

        if let Some(card) = field.get_room_index(index - 1) {
            println!("  >> {}", card);
            return Some(card);
        }

        return None
    }

    // スート判定して処理割り振り
    pub fn identify_suit(player: &mut Player, card: &Card) {
        player.add_selected(card.clone());

        match card.get_suit().as_str() {
            SUIT_STR_HART => Self::potion(player, card),
            SUIT_STR_DIAMOND => Self::equip(player, card),
            SUIT_STR_CLOVER => Self::battle(player, card),
            SUIT_STR_SPADE => Self::battle(player, card),
            _ => {}
        }
    }

    /// 回復
    fn potion(player: &mut Player, card: &Card) {
        player.update_hp(card.get_rank());

        potion_display(card, player);
    }

    /// 装備
    fn equip(player: &mut Player, card: &Card) {
        equip_display(card, player);

        player.update_equip(Some(card.clone()));
    }

    /// 銭湯
    fn battle(player: &mut Player, card: &Card) {
        battle_start_display(card, player);

        let mut equip: isize = 0;
        if let Some(_card) = player.get_equip() {
            equip = _card.get_rank();
        }

        let damage: isize = equip - card.get_rank();
        if damage < 0 {
            player.damage_hp(damage);
        }

        battle_result_display(player, card, damage);
    }
}
