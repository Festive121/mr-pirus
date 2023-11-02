mod clear;

use std::{io, thread, fs};
use colored::*;
use clear::clear_console;
use std::time::Duration;
use std::process::{Command, exit};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

fn main() -> io::Result<()> {
    start();

    const COUNTDOWN_BIN: &'static [u8] = include_bytes!("../target/release/countdown.exe");
    const WEB_BIN: &'static [u8] = include_bytes!("../target/release/web.exe");

    save_binary("countdown", COUNTDOWN_BIN).unwrap();
    save_binary("c2", WEB_BIN).unwrap();

    let should_exit = Arc::new(AtomicBool::new(false));
    let r = should_exit.clone();

    ctrlc::set_handler(move || {
        r.store(true, Ordering::SeqCst);
    }).expect("CTRL-C ERROR RECEIVING");

    while !should_exit.load(Ordering::SeqCst) {
        let mut lives = 3;

        execute_challenge(&mut lives, challenge_1);
        execute_challenge(&mut lives, challenge_2);

        if lives == 0 {
            println!("YOU LOSE!!! I suggest finding a sub and doing some reading...");
        }

        break;
    }

    Ok(())
}

fn start() {
    clear_console();

    println!("you found me!!");
    println!("either {} or move on.",
             "play along".red().bold()
    );
    println!("This program will (probably) not harm your computer.");
    println!("This is a test for you to determine your cyber-sec capabilities");
    println!("By continuing, you are choosing to continue.\nYou will be given {} chances to stop.\n",
             "3".red().bold());
    println!("want to continue? (y/n)");

    let mut start = String::new();

    io::stdin()
        .read_line(&mut start)
        .expect("Failed to read input (var=start)");

    start = start.trim().to_uppercase();

    if start == "Y" {
        clear_console();

        let mut sure = String::new();
        println!("are you sure? (yes/no)");

        io::stdin()
            .read_line(&mut sure)
            .expect("Failed to read input (var=sure)");

        sure = sure.trim().to_uppercase();

        if sure == "YES" {
            let mut count = 10;
            while count > 0 {
                clear_console();
                println!("ok. you have {} seconds to exit the program before it starts", count);
                count -= 1;
                thread::sleep(Duration::from_secs(1));
            }
        } else if sure == "NO" {
            no();
        } else {
            clear_console();

            println!("invalid answer");
            thread::sleep(Duration::from_secs(1));

            exit(1);
        }
    } else if start == "N" {
        no();
    } else {
        clear_console();

        println!("invalid answer");
        thread::sleep(Duration::from_secs(1));

        exit(1);
    }
}

fn challenge_1() -> bool {
    let mut countdown = false;
    let mut remove_dur = 0;
    let mut hints = 3;
    loop {
        clear_console();

        println!("{}", "Welcome Jim Pyke...".red().bold());
        thread::sleep(Duration::from_secs(3 - remove_dur));
        println!("For your first challenge, you have to reverse engineer a file and find the password.");
        thread::sleep(Duration::from_secs(3 - remove_dur));
        println!("Type \"hint\" for a hint! You only get {}.", "3".red());
        thread::sleep(Duration::from_secs(3 - remove_dur));
        println!("Here's the link:\n\
        https://mega.nz/file/frQxAIBZ#_CXc4LIDHr3N1WTToPnaPKR14BaPmHfMf0QDaAeRlzI");
        thread::sleep(Duration::from_secs(3 - remove_dur));
        println!("Your time starts now...");
        thread::sleep(Duration::from_secs(3 - remove_dur));
        if countdown == false {
            run_binary("countdown").unwrap();
            countdown = true;
            remove_dur = 3;
        }

        println!("Enter the password to continue.");
        let mut pass = String::new();
        io::stdin()
            .read_line(&mut pass)
            .expect("Failed to read input (var=pass)");

        if pass.trim() == "integratedtc.net" {
            clear_console();
            println!("{}", "You got it!".bold().green());
            thread::sleep(Duration::from_secs(3));

            return true;
        } else if pass.trim().to_uppercase() == "HINT" {
            if hints == 3 {
                clear_console();
                println!("Use IDA Free. It is very clear and easy.");
                println!("{} hints left (press ENTER to continue)", hints);
                hints -= 1;
                io::stdin().read_line(&mut String::new()).expect("Failed to break program");
            } else if hints == 2 {
                clear_console();
                println!("Check the .data section. I bet you will find some strings there.");
                println!("{} hints left (press ENTER to continue)", hints);
                hints -= 1;
                io::stdin().read_line(&mut String::new()).expect("Failed to break program");
            } else if hints == 1 {
                clear_console();
                println!("I think some processors convert ASCII text to Hex.");
                println!("{} hints left (press ENTER to continue)", hints);
                hints -= 1;
                io::stdin().read_line(&mut String::new()).expect("Failed to break program");
            } else if hints == 0 {
                clear_console();
                println!("No hints left :( (press ENTER to continue)");
                io::stdin().read_line(&mut String::new()).expect("Failed to break program");
            }
        } else {
            clear_console();
            println!("NOPE!!!");
            thread::sleep(Duration::from_secs(1));
        }
    }
}

fn challenge_2() -> bool {
    clear_console();
    println!("now lets try a quiz! you need a 100 to continue.");
    thread::sleep(Duration::from_secs(3));
    println!("you will get 1 try");
    thread::sleep(Duration::from_secs(3));
    println!("soon, I will open an webserver");
    thread::sleep(Duration::from_secs(3));
    println!("there is a js function that will detect if you leave the page.");
    thread::sleep(Duration::from_secs(3));
    println!("good luck!");

    run_binary("c2").unwrap();

    thread::sleep(Duration::from_secs(10));

    clear_console();
    println!("Im gonna be honest:");
    println!("idk how to do that");
    println!("so type your score here (0, 25, 50, 75, 100): ");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let score: i32 = input.trim().parse().expect("Failed to parse input to an integer");

    match score {
        0 => {
            println!("do you not know your subject?");
            false
        }
        25 => {
            println!("that's pretty bad");
            false
        }
        50 => {
            println!("umm not good");
            false
        }
        75 => {
            println!("so close but still a C");
            false
        }
        100 => {
            println!("good job! unless you cheated. in that case bad job.");
            true
        }
        _ => {
            println!("default--match score");
            false
        }
    }
}

fn no() {
    clear_console();

    let mut handle = String::new();
    println!("Please type \"Y\" if it is too much for you to handle");
    io::stdin()
        .read_line(&mut handle)
        .expect("Failed to read line (var=handle)");
    handle = handle.trim().to_uppercase();

    clear_console();

    if handle == "Y" {
        println!("Thanks for your honesty!");
        thread::sleep(Duration::from_secs(2));

        exit(0);
    } else if handle == "N" {
        println!("Don't lie...");
        thread::sleep(Duration::from_secs(2));

        exit(0);
    } else {
        println!("invalid answer... goodbye!");

        thread::sleep(Duration::from_secs(2));
        exit(0);
    }
}

// fn open_file(path: &str) -> io::Result<()> {
//     Command::new("cmd")
//         .args(&["/C", "start", path])
//         .status()?;
//
//     Ok(())
// }

fn execute_challenge<F: Fn() -> bool>(lives: &mut i32, challenge: F) {
    let current_lives = *lives;
    if !challenge() {
        *lives -= 1;
        if *lives != current_lives {
            println!("Lives left: {}", lives.to_string().red().bold());
            thread::sleep(Duration::from_secs(3));
        }
    }
}

fn save_binary(name: &str, data: &[u8]) -> io::Result<()> {
    let path = std::env::temp_dir().join(format!("{}.exe", name));
    fs::write(&path, data)?;
    Ok(())
}


fn run_binary(name: &str) -> io::Result<()> {
    let path = std::env::temp_dir().join(format!("{}.exe", name));
    let path_str = path.to_str().unwrap();

    Command::new("cmd.exe")
        .args(&["/C", "start", "cmd.exe", "/K", path_str])
        .spawn()?
        .wait()?;

    Ok(())
}
