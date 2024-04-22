use std::io::{self, Write};

use crate::game::{CardVec, Game};

pub fn game_loop(mut game: Game) {
    display_help();
    loop {
        display_game_state(&game);

        // Get user input
        print_and_flush("Enter your action (Type 'h' or 'help' for Commands): ");
        let input = get_user_input().to_string();
        match update_game_state(game, input) {
            Some(updated_game) => game = updated_game,
            None => break,
        }

        // Check if the game has ended
        if is_round_over(&game) {
            println!("Game over!");
            break;
        }

        // implement curr_player logic
    }
}

fn display_game_state(game: &Game) {
    println!("\n******************************");
    println!("Player {}'s Turn:\n", game.cur_plr + 1);
    for (i, player) in game.plrs.iter().enumerate() {
        if game.cur_plr == i {
            println!("Player {}: {}", i + 1, CardVec(&player.cards))
        } else {
            println!("Player {}: {}", i + 1, player.cards.len())
        }
    }

    println!("\nDeck: {} Cards", game.deck.cards.len());
    println!("Discard: {}", CardVec(&game.discard.cards));
    println!("\n******************************");
}

fn update_game_state(mut game: Game, input: String) -> Option<Game> {
    match &input.to_lowercase()[..] {
        "draw" | "d" => game.plrs[game.cur_plr].draw(&mut game.deck),
        "melds" | "ms" => {
            for (i, player) in game.plrs.iter().enumerate() {
                println!("Player {} Melds: {}", i + 1, CardVec(&player.melds))
            }
        }
        "meld" | "m" => {
            // Implement meld logic
            print_and_flush("Which cards would you like to play? (e.g., JH, JS, JC): ");
            let card_input = get_user_input();
            if let Err(error) = play_cards(&mut game, &card_input) {
                println!("{}", error)
            }
        }
        "layoff" | "l" => {
            // Implement layoff logic.
        }
        "discard" | "disc" => {
            print_and_flush("Which card would you like to discard? (e.g., JH): ");
            let card_input = get_user_input();

            match discard_card(&mut game, &card_input) {
                Ok(_) => game.next_player(), // Move to the next player after discarding
                Err(error) => println!("{}", error),
            }
        }
        "help" | "h" => display_help(),
        "quit" | "q" => return None,
        _ => println!("Not implemented yet."),
    }

    Some(game)
}

fn discard_card(game: &mut Game, card_input: &str) -> Result<(), String> {
    let card_str = card_input.trim().to_ascii_uppercase();
    let current_player = &mut game.plrs[game.cur_plr];

    if let Some(index) = current_player
        .cards
        .iter()
        .position(|card| format!("{}", card) == card_str)
    {
        let card = current_player.cards.remove(index);
        game.discard.cards.push(card);
        Ok(())
    } else {
        Err(format!("Card not found in hand: {}", card_str))
    }
}

fn play_cards(game: &mut Game, card_inputs: &str) -> Result<(), String> {
    let card_strs = card_inputs
        .split(',')
        .map(|card| card.trim().to_ascii_uppercase())
        .collect::<Vec<String>>();

    let current_player = &mut game.plrs[game.cur_plr];
    let mut cards_to_play = Vec::new();

    for card_str in card_strs {
        match current_player
            .cards
            .iter()
            .position(|card| format!("{}", card) == card_str)
        {
            Some(index) => cards_to_play.push(current_player.cards.remove(index)),
            None => return Err(format!("Card not found in hand: {}", card_str)),
        }
    }

    // Logic to add the cards to the appropriate meld or layoff
    // ...

    Ok(())
}

fn is_round_over(game: &Game) -> bool {
    game.deck.cards.is_empty() || game.plrs.iter().any(|player| player.cards.is_empty())
}

pub fn display_help() {
    println!("Available commands:");
    println!("  draw, d     - Draw a card from the stockpile");
    println!("  layoff, l   - Lay off a card from your hand");
    println!("  meld, m     - Meld cards from your hand");
    println!("  melds, ms   - Display the melds in play");
    println!("  discard, disc - Discard a card to the discard pile (ends your turn)");
    println!("  quit, q     - Quit the game");
    // Add more commands here as needed
}

fn get_user_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap(); // Read user input
    input.trim().to_owned() // Remove newline character and return the input
}

fn print_and_flush(msg: &str) {
    print!("{}", msg);
    io::stdout().flush().unwrap();
}
