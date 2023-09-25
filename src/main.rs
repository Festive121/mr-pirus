mod clear;

use std::{io, thread};
use colored::*;
use clear::clear_console;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use std::process::{Command, exit};

fn main() -> io::Result<()> {
    start();

    let should_exit = Arc::new(AtomicBool::new(false));
    let r = should_exit.clone();

    ctrlc::set_handler(move || {
        r.store(true, Ordering::SeqCst);
    }).expect("CTRL-C ERROR RECEIVING");

    while !should_exit.load(Ordering::SeqCst) {
        clear_console();

        println!("{}", "Welcome Jim Pike...".red().bold());
        thread::sleep(Duration::from_secs(1));
        println!("For your first challenge, you have to reverse engineer the file on your desktop and find the password");
        thread::sleep(Duration::from_secs(3));
        println!("Your time starts now...");
        thread::sleep(Duration::from_secs(1));
        if let Err(err) = open_file("Desktop/countdown") {
            eprintln!("Error opening file\n{}", err);
        }

        println!("Enter the password to continue.");
        let mut pass = String::new();
        io::stdin()
            .read_line(&mut pass)
            .expect("Failed to read input (var=pass)");

        if pass == "integratedtc.net" {
            println!("You got it!");
            break;
        } else {
            println!("NOPE!!!");
            break;
        }
    }

    println!("Exiting.");
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
