use std::io;
use rand::Rng;

const OPTIONS: [&str; 3] = ["Rock", "Paper", "Scissors"];

fn main() {
    println!("=== Rock - Paper - Scissor ===");

    let npc_choice: usize = rand::thread_rng().gen_range(0..OPTIONS.len());

    println!("Enter your choice:");
    print_options(&OPTIONS);

    let mut player_choice = String::new();

    loop {
        io::stdin()
            .read_line(&mut player_choice)
            .expect("Failed to read line");

        let player_choice: usize = match player_choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if player_choice < 1 || player_choice > OPTIONS.len() as usize {
            println!("Invalid choice. Please try again.");
            continue;
        }

        // convert player's choice from menu id to array index
        let player_choice: usize = player_choice - 1;
        let result: u32 = is_win(player_choice, npc_choice);
        let player_choice: &str = OPTIONS[player_choice];
        let npc_choice: &str = OPTIONS[npc_choice];

        print_result(player_choice, npc_choice, result);

        break;
    }
}

fn print_options(options: &[&str]) {
    for (index, &option) in options.iter().enumerate() {
        println!("{a}. {b}", a = index + 1, b = option);
    }
}

fn is_win(player: usize, npc: usize) -> u32 {
    match (player, npc) {
        (0, 1) | (1, 2) | (2, 0) => 0,
        (0, 2) | (1, 0) | (2, 1) => 1,
        _ => 2,
    }
}

fn print_result(player_choice: &str, npc_choice: &str, result: u32) {
    println!("You: {}", player_choice);
    println!("NPC: {}", npc_choice);

    if result == 2 {
        println!("DRAW!");
    } else if result == 1 {
        println!("YOU WIN!");
    } else {
        println!("YOU LOSE!");
    }
}