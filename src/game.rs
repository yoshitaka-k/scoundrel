use std::io::{stdout, Write};
use crossterm::{
    execute,
    cursor::{MoveTo},
    terminal::{Clear, ClearType},
    style::{Stylize},
};

use crate::constants::{ROOM_MAX_CARD};
use crate::cli::{
    console::{
        print_br,
        print_single_separator,
        print_double_separator,
    },
    print_display::{
        title_display,
        dungeon_count_display,
        room_display,
        player_display,
    },
    indicate::execute_with_spinner,
};
use crate::logic::{GameSession};
use crate::trump::{Deck, Field, Player};
use crate::{
    wait_for_dramatic_pause,
    wait_for_long_dramatic_pause,
};

pub fn app() -> std::io::Result<()> {
    let mut deck = Deck::new();
    let mut field = Field::new();
    let mut player = Player::new("Player".to_string());

    let mut selected_count: usize = 0;
    let mut is_skip: bool = false;

    let mut stroke = stdout();

    GameSession::shuffle(&player, deck.get_cards());

    print_single_separator();

    'app: loop {
        execute!(
            stroke,
            Clear(ClearType::All),
            MoveTo(0, 0)
        )?;

        print_double_separator();

        title_display();

        print_double_separator();

        print!("{} ", "/".green());
        println!("Deck setup and {} a shuffle end.", player.get_name());

        print_single_separator();

        if selected_count == 0 {
            if !GameSession::room_setup(&mut deck, &mut field) {
                println!("Game Clear.");

                break 'app Ok(());
            }

            player.selected_clear();

            if field.get_room_len() < ROOM_MAX_CARD {
                selected_count = 1;
            }
        }

        dungeon_count_display(&deck);

        print_br();

        let _ = player_display(&player);

        print_br();

        room_display(field.get_room(), is_skip);

        print_br();

        if let Some(card) = GameSession::room_card_selected(&mut field, is_skip) {
            wait_for_dramatic_pause();

            print_br();

            GameSession::identify_suit(&mut player, &card);
            wait_for_dramatic_pause();

            selected_count = selected_count + 1;
            is_skip = false;

        } else {
            GameSession::re_room_setup(&mut deck, &mut field);
            wait_for_dramatic_pause();

            print_br();

            selected_count = 0;
            is_skip = true;
        }

        print_single_separator();

        print_br();

        if GameSession::is_gameover(&player) {
            println!("{}", format!("{} HP Empty.", player.get_name()).red().bold());
            println!("{}", "Game Over.".red().bold());
            wait_for_dramatic_pause();

            print_br();

            break 'app Ok(());
        }

        if selected_count > 2 {
            execute_with_spinner(
                "Next Terun ...",
                "",
            || {
                wait_for_long_dramatic_pause();
            });

            selected_count = 0;
        } else {

            execute_with_spinner(
                "Next card selected ...",
                "",
            || {
                wait_for_long_dramatic_pause();
            });
        }

        stdout().flush()?;
    }
}

