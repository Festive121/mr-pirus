mod clear;

use std::{io, thread};
use colored::*;
use clear::clear_console;
use std::time::Duration;
use std::process::{Command, exit};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

fn main() -> io::Result<()> {
    start();

    let should_exit = Arc::new(AtomicBool::new(false));
    let r = should_exit.clone();

    ctrlc::set_handler(move || {
        r.store(true, Ordering::SeqCst);
    }).expect("CTRL-C ERROR RECEIVING");

    while !should_exit.load(Ordering::SeqCst) {
        let mut lives = 3;

        execute_challenge(&mut lives, challenge_1);
        execute_challenge(&mut lives, challenge_2);

        break;
    }

    println!("YOU LOSE!!! I suggest finding a sub and doing some reading...");
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

fn open_file(path: &str) -> io::Result<()> {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "start", path])
            .status()?;
    } else if cfg!(target_os = "macos") {
        Command::new("open")
            .arg(path)
            .status()?;
    } else {
        Command::new("xdg-open")
            .arg(path)
            .status()?;
    }
    Ok(())
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

fn challenge_1() -> bool {
    let mut c1 = false;
    let mut countdown = false;
    let mut remove_dur = 0;
    let mut hints = 3;
    while c1 == false {
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
            if let Err(err) = open_file("Desktop/countdown") {
                eprintln!("Error opening file\n{}", err);
            }
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
            c1 = true;
            thread::sleep(Duration::from_secs(3));

            true;
            break;
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
                println!("I think some processors convert ACSII text to Hex.");
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
    false
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

    if let Err(err) = open_file("Desktop/web") {
        eprintln!("Error opening file\n{}", err);
    }

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

fn end_c1() { }
