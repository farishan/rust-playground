use std::io;
use rand::Rng;

fn main() {
    println!("=== Rock - Paper - Scissor ===");

    // @TODO Optimize OPTIONS handling
    const OPTIONS: [&str; 3] = ["Rock", "Paper", "Scissor"];

    let rock = OPTIONS[0];
    let paper = OPTIONS[1];
    let scissor = OPTIONS[2];

    let npc_choice: u32 = rand::thread_rng().gen_range(0..=2);

    loop {
        println!("Enter your choice:");
        println!("1. {rock}");
        println!("2. {paper}");
        println!("3. {scissor}");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let c_index = choice - 1; // convert user choice from menu id to array index

        println!("You: {}", get_choice(c_index));
        println!("NPC: {}", if npc_choice == 0 { "Rock" } else if npc_choice == 1 { "Paper" } else { "Scissor" });

        let result: u32 = is_win(c_index, npc_choice);

        if result == 2 {
            println!("DRAW!");
        } else if result == 1 {
            println!("YOU WIN!");
        } else {
            println!("YOU LOSE!");
        }

        break;
    }
}

fn get_choice(choice: u32) -> &'static str {
    if choice == 0 {
        "Rock"
    } else if choice == 1 {
        "Paper"
    } else if choice == 2 {
        "Scissor"
    } else {
        "Unknown"
    }
}

fn is_win(player: u32, npc: u32) -> u32 {
    if player == 0 && npc == 1 {
        0
    } else if player == 0 && npc == 2 {
        1
    } else if player == 1 && npc == 0 {
        1
    } else if player == 1 && npc == 2 {
        0
    } else if player == 2 && npc == 0 {
        0
    } else if player == 2 && npc == 1 {
        1
    } else {
        2
    }
}
