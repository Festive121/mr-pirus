#[macro_use]
extern crate lazy_static;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::{io, thread};
use std::io::Write;
use std::process::Command;
use std::time::Duration;
use rand::Rng;

fn main() {
    println!("Probably pretty helpful, and I spent a long time working on it.");
    println!("https://drive.google.com/uc?export=download&id=1kiPk4ei-d_Pccx2wixir-xD1lHB8PAn9");
    println!("Enter to continue...");
    let mut temp = String::new();
    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read input (var=temp)");

    let mut rng = rand::thread_rng();

    loop {
        let file_contents = include_str!("../c4/words.txt");
        let lines: Vec<&str> = file_contents.lines().collect();

        let random_index = rng.gen_range(0..lines.len());
        let word = lines[random_index];

        let alg = rng.gen_range(1..=3);

        clear_console();

        let key = match alg {
            1 => alg1(word),
            2 => alg2(word),
            3 => alg3(word),
            _ => unreachable!(),
        };

        match append_key_to_file(&key) {
            Ok(_) => {},
            Err(e) => println!("Error appending to file: {}", e),
        }

        countdown(30, &key);
    }

}

fn alg1(input: &str) -> String {
    let charset_len = CHAR_MAP.len();

    input.chars().map(|c| {
        if let Some(&i) = CHAR_MAP.get(&c) {
            let rotated_index = (i + 13) % charset_len;
            CHAR_MAP.iter().find_map(|(&key, &val)| {
                if val == rotated_index { Some(key) } else { None }
            }).unwrap_or(c)
        } else {
            c
        }
    }).collect()
}

fn alg2(input: &str) -> String {
    let mut result = String::new();

    let input_chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let output_chars = "NOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABCDEFGHIJKLM";

    let mut map: HashMap<char, char> = HashMap::new();

    for (input_char, output_char) in input_chars.chars().zip(output_chars.chars()) {
        map.insert(input_char, output_char);
    }

    for c in input.chars() {
        match map.get(&c) {
            Some(&mapped_char) => result.push(mapped_char),
            None => result.push(c),  // or you can choose to do nothing if the character is not found
        }
    }

    result
}

fn alg3(input: &str) -> String {
    let combined_chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    input.chars().map(|c| {
        if let Some(pos) = combined_chars.find(c) {
            // Calculate the new position after rotation
            let rotated_pos = (pos + 13) % combined_chars.len();
            // Get the character at the new position
            combined_chars.chars().nth(rotated_pos).unwrap_or(c)
        } else {
            // Return the character as is if it's not found in the combined string
            c
        }
    }).collect()
}

lazy_static! {
    static ref CHAR_MAP: HashMap<char, usize> = {
        let mut m = HashMap::new();
        let chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
        for (i, c) in chars.chars().enumerate() {
            m.insert(c, i);
        }
        m
    };
}

fn append_key_to_file(key: &str) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true) // This will create the file if it doesn't exist
        .open(".invalid_keys.txt")?;

    writeln!(file, "{}", key)?;
    Ok(())
}

fn clear_console() {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", "cls"])
            .status()
            .unwrap();
    } else {
        Command::new("clear")
            .status()
            .unwrap();
    }
}

fn countdown(int: i8, key: &str) -> bool {
    for i in (1..=int).rev() {
        println!("Key: {}", key);
        println!("{}", i);

        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_secs(1));
        clear_console();
    }
    true
}
