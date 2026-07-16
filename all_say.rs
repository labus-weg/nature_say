use std::env;
use std::io::{self, Read};
use std::thread;
use std::time::Duration;
use colored::*;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut message = String::new();

    let is_piped = !atty::is(atty::Stream::Stdin);

    if is_piped {
        let mut buffer = String::new();
        if io::stdin().read_to_string(&mut buffer).is_ok() {
            message = buffer.trim().to_string();
        }
    } else if !args.is_empty() {
        message = args.join(" ");
    }

    if message.is_empty() {
        message = "Electrifying togetherness!".to_string();
    }

    // Run the synchronized strike animation 3 times
    for _ in 0..3 {
        animate_lightning(&message);
    }
    
    // Final state: Both structures standing fully illuminated
    print_frame(&message, 999); 
    std::process::exit(0);
}

fn animate_lightning(msg: &str) {
    let total_frames = 24; 
    for current_row in 0..total_frames {
        print!("{}[2J{}[H", 27 as char, 27 as char);
        print_frame(msg, current_row);
        thread::sleep(Duration::from_millis(45));
    }
}

fn print_frame(msg: &str, bolt_row: usize) {
    let len = msg.len();
    let mut layout = String::new();
    
    // Shared speech bubble stretching across the top
    layout.push_str(&format!("  {}\n", "-".repeat(len + 2)));
    layout.push_str(&format!("< {} >\n", msg));
    layout.push_str(&format!("  {}\n", "-".repeat(len + 2)));
    layout.push_str("        \\                  /\n");
    layout.push_str("         \\                /\n");

    // Side-by-side array merging Goat (Left) and Rose (Right) row-by-row
    let split_art = vec![
        (r#"  ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀"#, r#"        @(\/)"#),
        (r#"  ⠀⠀⠀⠀⠀⠀⠀⠠⠴⠶⠾⠿⠿⠿⢶⣦⣄⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀"#, r#"     (\/)-{}-)@"#),
        (r#"  ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⢿⣿⣆⠐⣄⠀⠀⠀⠀⠀⠀⠀⠀⠀"#, r#"   @(={}=)/\)(\/)"#),
        (r#"  ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠿⠿⠆⠹⠦⠀⠀⠀⠀⠀⠀⠀⠀"#, r#"  (\/(/\)@| (-{}-)"#),
        (r#"  ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣤⣤⣤⣤⣀⠐⣶⣶⣶⣶⣶⣶⡀⢀⣀⣀⠀⠀⠀"#, r#" (={}=)@(\/)@(/\)@"#),
        (r#"  ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠛⠻⢿⣿⡆⢹⡿⠻⢿⣿⣿⣷⠈⠿⠛⠁⠀⠀"#, r#"  (/\)\(={}=)/(\/)"#),
        (r#"  ⠀⠀⠀⠀⠀⠀⠀⠀⢀⣀⣤⣴⣾⣷⣤⣉⣠⣾⣷⣦⣼⣿⣿⣿⣧⠀⠀⠀⠀⠀"#, r#"  @(\/)\(/\)/(={}=)"#),
        (r#"  ⠀⣶⣶⣶⣶⣶⣶⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣇⠀⠀⠀⠀"#, r#"  (-{}-)""""@/(/\)"#),
        (r#"  ⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡄⠀⠀⠀"#, r#"   (/\)|:   |"#),
        (r#"  ⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣛⠻⢧⣘⡷⠀⠀⠀"#, r#"      /::'   \"#),
        (r#"  ⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠋⠀⠀⣉⠛⠿⣷⣦⣌⠁⠀⠀⠀"#, r#"     /:::     \"#),
        (r#"  ⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠋⣠⠘⠀⠀⢹⣿⣶⣶⠀⠀⠀⠀⠀⠀"#, r#"    |::'       |"#),
        (r#"  ⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠋⠀⢺⣿⠀⠀⠀⠘⣿⣿⡟⠀⠀⠀⠀⠀⠀"#, r#"    |::        |"#),
        (r#"  ⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠋⠀⠀⠀⠀⠁⠀⠀⠀⠀⠻⡟⠃⠀⠀⠀⠀⠀⠀"#, r#"    \::.       /"#),
        (r#"  ⠀⠛⠛⠛⠛⠛⠛⠛⠛⠋⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀"#, r#"jgs  ':______.'"#),
        (r#"  ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀"#, r#"      `""""""`"#),
    ];

    for (left, right) in split_art {
        layout.push_str(&format!("{}{}\n", left, right));
    }

    for (i, line) in layout.lines().enumerate() {
        // Individual color maps synchronized by screen row index
        let (base_r, base_g, base_b) = match i {
            0..=4   => (220, 220, 225), // Speech Bubble & Connectors
            5..=11  => (235, 60, 100),  // Upper Goat & Crimson Rose Blossom
            12..=15 => (45, 185, 90),   // Mid Goat & Rose Foliage / Stems
            _       => (15, 175, 230),  // Base Goat Hooves & Blue Glass Vase
        };

        if i == bolt_row {
            // White-hot shock strike core
            println!("{}", line.truecolor(255, 255, 255).bold());
        } else if i == bolt_row + 1 || (bolt_row > 0 && i == bolt_row - 1) {
            // High-voltage color amplification
            let overdriven_r = (base_r as u16 + 85).min(255) as u8;
            let overdriven_g = (base_g as u16 + 85).min(255) as u8;
            let overdriven_b = (base_b as u16 + 85).min(255) as u8;
            println!("{}", line.truecolor(overdriven_r, overdriven_g, overdriven_b).bold());
        } else if bolt_row == 999 {
            // Normal static render
            println!("{}", line.truecolor(base_r, base_g, base_b));
        } else {
            // Shadowed ambient state
            println!("{}", line.truecolor(base_r / 5, base_g / 5, base_b / 5));
        }
    }
}

