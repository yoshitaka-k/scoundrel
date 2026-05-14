use crate::constants::{
    ROOM_MAX_CARD,
};
use crate::cli::{
    console::{
        print_br,
        print_single_separator,
    },
    print_display::{
        dungeon_count_display,
        room_display,
        player_display,
    },
};
use crate::logic::GameSession;
use crate::trump::{Deck, Field, Player};
use crate::wait_for_dramatic_pause;

pub fn app() -> std::io::Result<()> {
    let mut deck = Deck::new();
    let mut field = Field::new();
    let mut player = Player::new("Player".to_string());

    let mut selected_count: usize = 0;
    let mut is_skip: bool = false;

    GameSession::shuffle(&player, deck.get_cards());

    print_single_separator();

    'app: loop {
        if selected_count == 0 {
            if !GameSession::room_setup(&mut deck, &mut field) {
                println!("Game Clear.");
                wait_for_dramatic_pause();

                break 'app Ok(());
            }

            player.selected_clear();

            if field.get_room_len() < ROOM_MAX_CARD {
                selected_count = 1;
            }
        }

        dungeon_count_display(&deck);
        wait_for_dramatic_pause();

        print_br();

        room_display(field.get_room(), is_skip);
        wait_for_dramatic_pause();

        print_br();

        player_display(&player);
        wait_for_dramatic_pause();

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
            println!("{} HP Empty.", player.get_name());
            println!("Game Over.");
            wait_for_dramatic_pause();

            break 'app Ok(());
        }

        if selected_count > 2 {
            selected_count = 0;
        }
    }
}

