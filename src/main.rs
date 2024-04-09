use colored::Colorize;
use wordle_game::Manager;

fn main() {
    let welcome = "Welcome to the Wordle game".yellow();
    println!("{}", welcome);

    let mut player = Manager::new();
    loop {
        player.draw_board();
        let player_guess = player.take_guess();
        if player.is_game_over(&player_guess){
            break;
        }
    }
}
