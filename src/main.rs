use rand::seq::SliceRandom;
use std::fs;

fn main() {
    let base_dir = if cfg!(target_os = "linux") {
        dirs::data_local_dir()
    } else {
        dirs::document_dir()
    };
    match fs::read_to_string(
        base_dir
            .expect("Base directory not found!")
            .join("Paradox Interactive")
            .join("Stellaris")
            .join("user_empire_designs_v3.4.txt"),
    ) {
        Ok(f) => {
            let text_tape = match jomini::TextTape::from_slice(f.as_bytes()) {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("Failed to parse user_empire_designs_v3.4.txt. Is your file corrupt? {e:#?}");
                    press_enter_to_continue();
                    return;
                }
            };
            let mut empires = text_tape
                .utf8_reader()
                .fields()
                .map(|f| f.0.read_str())
                .collect::<Vec<_>>();
            let mut r = rand::thread_rng();
            empires.shuffle(&mut r);
            println!("Empire data found, {} total candidates.", empires.len());
            println!();
            println!("Data includes");
            const PREVIEW_COUNT: usize = 3;
            for empire in empires.iter().take(PREVIEW_COUNT) {
                println!("  {}", empire);
            }
            if empires.len() > PREVIEW_COUNT {
                let remaining = empires.len() - PREVIEW_COUNT;
                println!();
                println!("...and {remaining} other empires.");
            }
            let mut number_input = None;
            while number_input.is_none() {
                println!();
                println!("How many would you like me to randomly select? (q to exit)");
                let mut input = String::new();
                if let Err(e) = std::io::stdin().read_line(&mut input) {
                    eprintln!("Failed to read line from stdin, shutting down.");
                    eprintln!("{e:#?}");
                    return;
                }
                let input = input.trim().to_string();
                if input.to_lowercase() == "q" {
                    println!("Goodbye");
                    return;
                }
                match input.parse::<usize>() {
                    Ok(r) => {
                        if r <= empires.len() {
                            number_input = Some(r);
                        } else {
                            eprintln!("You don't have that many empires! Input a smaller number.");
                        }
                    }
                    Err(_) => {
                        eprintln!("Input wasn't a non-negative number.")
                    }
                }
            }
            println!();
            empires.shuffle(&mut r);
            for (i, selected) in empires
                .iter()
                .take(number_input.expect("infallible"))
                .enumerate()
            {
                println!("  {:02}:  {selected}", i + 1);
            }
            println!();
            println!("Done!");
            press_enter_to_continue();
        }
        Err(e) => {
            eprintln!("user_empire_designs_v3.4.txt not found.");
            eprintln!("Please make a custom empire using Stellaris 3.4 or newer");
            eprintln!("{e:#?}");
            press_enter_to_continue();
        }
    }
}

fn press_enter_to_continue() {
    println!("Press enter to exit.");
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
}
