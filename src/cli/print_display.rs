use figlet_rs::FIGlet;
use crossterm::{
    execute,
    cursor::{MoveTo},
    style::{Print, Stylize},
};
use std::io::{stdout};

use crate::cli::console::{
    print_br,
    print_single_separator,
};
use crate::{Card, Deck, Player};

pub fn title_display() {
    let standard_font = FIGlet::standard().unwrap();
    let title = &format!("{}", standard_font.convert("Scoundrel").unwrap());

    print!("{}", title.clone().magenta().bold());

    print_single_separator();

    println!("  Version: {}  |  License: {}", env!("CARGO_PKG_VERSION"), env!("CARGO_PKG_LICENSE"));
    println!("  Starting Scoundrel Game Engine... 🚀");
}

/// ダンジョンカード数
pub fn dungeon_count_display(deck: &Deck) {
    println!("Dungeon Cards.");
    println!("  {} card", deck.len());
}

/// ルームカード表示
pub fn room_display(room: &Vec<Card>, is_skip: bool) {
    println!("Room Card.");
    let mut room_str: String = String::new();
    for (i, card) in room.iter().enumerate() {
        room_str = format!("{} {}: {} /", room_str, i + 1, card.get_name());
    }

    println!("{}", format!("  {}", room_str.trim_end_matches('/').trim()));

    if room.len() == 4 && is_skip == false {
        println!("  0: Skip.");
    } else {
        print_br();
    }
}

/// 装備カード表示
pub fn player_display(player: &Player) -> Result<(), Box<dyn std::error::Error>> {
    let mut stroke = stdout();

    execute!(
        stroke,
        MoveTo(0, 16),
        Print("Health."),
    )?;

    execute!(
        stroke,
        MoveTo(0, 17),
        Print(format!("  {} point", player.get_hp()))
    )?;

    execute!(
        stroke,
        MoveTo(20, 16),
        Print("Equip Card.")
    )?;

    execute!(
        stroke,
        MoveTo(20, 17),
        Print(format!("  {}", player.get_equip_name()))
    )?;

    print_br();

    execute!(
        stroke,
        MoveTo(0, 19),
        Print("Selected History.")
    )?;

    execute!(
        stroke,
        MoveTo(0, 20),
    )?;

    if player.selected_len() == 0 {
        println!("  None");
        return Ok(());
    }

    let mut selected_str: String = String::new();
    let selected = player.get_selected();
    for card in selected {
        selected_str = format!("{} {},", selected_str, card);
    }
    println!("  {}", selected_str.trim_end_matches(',').trim());

    Ok(())
}

/// ポーション効果表示
pub fn potion_display(card: &Card, player: &Player) {
    println!("Potion.");
    println!("  {} pt >> HP {} pt", card.get_rank(), player.get_hp());

    print_br();
}

/// 装備
pub fn equip_display(card: &Card, player: &Player) {
    println!("Equipment.");
    println!("  {} >> new {}", player.get_equip_name(), card.get_name());

    print_br();
}

/// 戦闘開始
pub fn battle_start_display(card: &Card, player: &Player) {
    println!("Battle.");
    println!("  {} vs. {}", player.get_equip_name(), card.get_name());

    print_br();
}

pub fn battle_result_display(player: &Player, card: &Card, damage: isize) {
    println!("Battle Result.");
    println!("  Damage: {} pt ({} - {})", damage, player.get_equip_name(), card.get_name());
    println!("    >> HP: {} pt", player.get_hp());

    print_br();
}
