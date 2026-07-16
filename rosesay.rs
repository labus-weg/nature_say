use std::env;
use std::io::{self, Read};
use std::thread;
use std::time::Duration;
use colored::*;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut message = String::new();

    // Check if data is coming from a terminal pipe
    let is_piped = !atty::is(atty::Stream::Stdin);

    if is_piped {
        let mut buffer = String::new();
        // Read from stdin exactly once, then drop the connection
        if io::stdin().read_to_string(&mut buffer).is_ok() {
            message = buffer.trim().to_string();
        }
    } else if !args.is_empty() {
        message = args.join(" ");
    }

    if message.is_empty() {
        message = "A beautiful rose for you!".to_string();
    }

    // Explicitly run the strike animation exactly 3 times
    for _ in 0..3 {
        animate_lightning(&message);
    }
    
    // Print the final static frame
    print_frame(&message, 999); 
    
    // Force a clean program exit back to the terminal prompt
    std::process::exit(0);
}

fn animate_lightning(msg: &str) {
    let total_frames = 24; 
    for current_row in 0..total_frames {
        // Clear screen and reset cursor
        print!("{}[2J{}[H", 27 as char, 27 as char);
        print_frame(msg, current_row);
        thread::sleep(Duration::from_millis(45));
    }
}

fn print_frame(msg: &str, bolt_row: usize) {
    let len = msg.len();
    let mut layout = String::new();
    
    layout.push_str(&format!("  {}\n", "-".repeat(len + 2)));
    layout.push_str(&format!("< {} >\n", msg));
    layout.push_str(&format!("  {}\n", "-".repeat(len + 2)));
    layout.push_str("        \\\n");
    layout.push_str("         \\\n");

    let rose_lines = vec![
        r#"        @(\/)"#,
        r#"     (\/)-{}-)@"#,
        r#"   @(={}=)/\)(\/)"#,
        r#"  (\/(/\)@| (-{}-)"#,
        r#" (={}=)@(\/)@(/\)@"#,
        r#"  (/\)\(={}=)/(\/)"#,
        r#"  @(\/)\(/\)/(={}=)"#,
        r#"  (-{}-)""""@/(/\)"#,
        r#"   (/\)|:   |"#,
        r#"      /::'   \"#,
        r#"     /:::     \"#,
        r#"    |::'       |"#,
        r#"    |::        |"#,
        r#"    \::.       /"#,
        r#"jgs  ':______.'"#,
        r#"      `""""""`"#,
    ];

    for line in rose_lines {
        layout.push_str(&format!("{}\n", line));
    }

    for (i, line) in layout.lines().enumerate() {
        let (base_r, base_g, base_b) = match i {
            0..=4  => (220, 220, 225), 
            5..=11 => (240, 20, 60),   
            12..=13=> (40, 190, 80),   
            _      => (0, 210, 235),   
        };

        if i == bolt_row {
            println!("{}", line.truecolor(255, 255, 255).bold());
        } else if i == bolt_row + 1 || (bolt_row > 0 && i == bolt_row - 1) {
            let overdriven_r = (base_r as u16 + 80).min(255) as u8;
            let overdriven_g = (base_g as u16 + 80).min(255) as u8;
            let overdriven_b = (base_b as u16 + 80).min(255) as u8;
            println!("{}", line.truecolor(overdriven_r, overdriven_g, overdriven_b).bold());
        } else if bolt_row == 999 {
            println!("{}", line.truecolor(base_r, base_g, base_b));
        } else {
            println!("{}", line.truecolor(base_r / 5, base_g / 5, base_b / 5));
        }
    }
}
