use rand::Rng;
use std::fs;

fn main() {
    match fs::read_to_string(
        dirs::document_dir()
            .expect("Documents directory not found!")
            .join("Paradox Interactive")
            .join("Stellaris")
            .join("user_empire_designs_v3.4.txt"),
    ) {
        Ok(f) => {
            let text_tape = jomini::TextTape::from_slice(f.as_bytes())
                .expect("Failed to parse user_empire_designs_v3.4.txt. Is your file corrupt?");
            let mut empires = text_tape
                .utf8_reader()
                .fields()
                .map(|f| f.0.read_str())
                .collect::<Vec<_>>();
            println!("Empire data found, {} eligible candidates:", empires.len());
            println!();
            for empire in &empires {
                println!("{}", empire);
            }
            let mut number_input = None;
            while number_input.is_none() {
                println!();
                println!("How many would you like me to randomly select? (q to exit)");
                let mut input = String::new();
                if let Err(e) = std::io::stdin().read_line(&mut input) {
                    eprintln!("Failed to read line from stdin, shutting down.");
                    eprintln!("{:?}", e);
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
            let mut r = rand::thread_rng();
            for _ in 0..number_input.expect("infallible") {
                let sel = r.gen_range(0..empires.len());
                let selected = empires.remove(sel);
                println!("{}", selected);
            }
            println!();
            println!("Done! Press enter to exit");
            let mut input = String::new();
            let _ = std::io::stdin().read_line(&mut input);
        }
        Err(e) => {
            eprintln!("user_empire_designs_v3.4.txt not found.");
            eprintln!("Please make a custom empire using Stellaris 3.4 or newer");
            eprintln!("{:?}", e);
        }
    }
}
