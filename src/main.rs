mod clear;

use clear::clear_console;
use colored::*;
use std::collections::{HashMap, HashSet};
use std::io::{BufRead, BufReader, Write};
use std::process::{Command, exit};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, mpsc};
use std::time::Duration;
use std::{io, thread, fs};

fn main() -> io::Result<()> {
    // start();

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

        // execute_challenge(&mut lives, challenge_1);
        // execute_challenge(&mut lives, challenge_2);
        execute_challenge(&mut lives, challenge_3);

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
    let (tx, rx) = mpsc::channel();
    let countdown_duration = 60; // Example: 60 seconds for the countdown

    // Spawn a thread for the countdown
    let tx_countdown = tx.clone();
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(countdown_duration));
        tx_countdown.send("Time Up").unwrap();
    });

    let mut countdown = false;
    let mut remove_dur = 0;
    let mut hints = 3;
    loop {
        match rx.try_recv() {
            Ok(msg) => {
                if msg == "Time Up" {
                    // Handle the end of the countdown
                    println!("Countdown finished!");
                    return false // Or handle it as needed
                }
            },
            Err(_e) => {
                // No message received, or an error occurred
                // You can ignore this or handle it as needed
            }
        }

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

fn challenge_3() -> bool {
    // clear_console();
    // println!("You are now going to try to find the password through a linux terminal");
    // thread::sleep(Duration::from_secs(3));
    // println!("All of this is simulated so there might not be every command you can think of");
    // thread::sleep(Duration::from_secs(3));
    // println!("here we go!");
    // thread::sleep(Duration::from_secs(1));

    const CORRECT_PASSWORD: &str = "givejackanAforreinventinglinux";
    const ADDRESS: &str = "389d7092";
    let mut guesses: i8 = 3;
    let mut narrate: i8 = 0;
    let mut shadow_discovered = false;

    let mut dir = String::from("~");
    let mut available_dirs = HashSet::new();
    available_dirs.insert("~");
    available_dirs.insert("~/more");
    available_dirs.insert("~/secret");

    let mut files = HashMap::new();
    let readme_details = r#"let guesses: i8 = 3;
println!("use the \"pass <password>\" command to submit a password, but you only get {} guesses", guesses);
println!("I also added a \"narrate\" command that prints your thoughts.");

let cmdlist = vec![
    "clear",
    "ls",
    "cd",
    "cat",
    "grep",
    "man (ascii)",
    "pass",
    "./",
    "narrate",
    "disassemble"
];"#;

    files.insert("~/readme.txt", readme_details);

    files.insert("~/secret/SECRET_PASSWORD", "To die: to sleep; to sleep: perchance to dread of so long a life,\
    but that the slings and them? To die, to grunt and sweat under a weary life, but that the will, and them?\
    To die, to suffer the will, and moment with this regard them?\
    To die, to sleep; no more; and the spurns that the question: whether 'tis a consummation devoutly to others that sleep of death what dream:\
    ay, the unworthy takes, when we have shuffled off this mortal coil, must give us pause.\
    There's the question: whether bear the respect");

    files.insert("~/secret/.shadow", r#"�ELF���4�4�@@�@8�8���������������������������������
�>�>�:�:���������������������������������������������
��/lib64/ld-linux-x86-32.so.2��GNU��GNU�u�9�w�
�?�?�?�?�?�?�?�?�?�?�?�?�?�?�?�?�?�?�?�?�?�?�?�?�?
�?�?�?�?�?�?�?�?�?�?�?�?�?�?�?�?�?�?�?�?�?�?�?�?�?
.text�.note.gnu.build-id�.eh_frame_hdr�.eh_frame�.init_array�
.fini_array�.dynamic�.got�.got.plt�.data�.bss�.comment�"#);

    let ls_details = r#"#!/bin/bash

cat << 'EOF'
let show_all = parts.contains(&"-a");

match dir.as_str() {
    "~" => {
        if show_all {
            println!(".");
            println!("..");
        }
        println!("{}", "secret".blue());
        println!("readme.txt");
    },
    "~/secret" => {
        if show_all {
            println!(".");
            println!("..");
            println!(".shadow");
        }
        println!("SECRET_PASSWORD");
        println!("{}", "ln?".green());
    },
    _ => println!("Directory not found.")
}
EOF
echo "just one command I'm working on. I'd like to use -fa to decrypt a file with a key, but not sure how..."#;
    files.insert("~/secret/ls?.sh", ls_details);
    files.insert("~/more/no.txt", "I am not adding every linux directory and file.");
    files.insert("~/more/man_computer.txt", "At a given memory point, there is a hex value\
    Hex values contain data for the system, which could be Strings.\
    For a program checking a password vvv\
    char password[] = \"pass\"; char input[100]; ... if (strcmp(input, pass) == 0) { correct(); };\
    password has to be stored somewhere, and if left unencrypted, can be read through the memory.\
    pass -> [0x00000000]: 70 61 73 73");

    // include mem addresses
    const SHADOW: &str = include_str!("../c3/shadow.txt");
    const LS: &str = include_str!("../c3/ls_mem.txt");

    // include man commands
    const ASCII: &str = include_str!("../c3/ascii.txt");

    clear_console();

    loop {
        if guesses == 0 {
            break
        }

        print!("{}:{}$ ", "jim@linux-desktop".green(), dir.blue());
        io::stdout().flush().expect("Failed to flush stdout");
        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read command");
        command = command.trim().parse().unwrap();
        let parts: Vec<&str> = command.split_whitespace().collect();
        let cmd = parts.get(0).unwrap_or(&"");

        match *cmd {
            "ls" => {
                let show_all = parts.contains(&"-a");
                let flag_d = parts.contains(&"-fa");

                if flag_d {
                    if let Some(file_name) = parts.get(2) { // getting the file name
                        let file_path = format!("{}/{}", dir, file_name); // . and vv checking if exists in current dir
                        if files.contains_key(&file_path.as_str()) {

                        } else {
                            println!("Key not found: {}", file_name);
                        }
                    } else {
                        println!("No file specified for decryption.");
                    }
                } else {
                    match dir.as_str() {
                        "~" => {
                            if show_all {
                                println!(".");
                                println!("..");
                            }
                            println!("{}", "secret".blue());
                            println!("{}", "more".blue());
                            println!("readme.txt");
                        },
                        "~/secret" => {
                            if show_all {
                                shadow_discovered = true;

                                println!(".");
                                println!("..");
                                println!("{}", ".shadow".green());
                            }
                            println!("SECRET_PASSWORD");
                            println!("{}", "ls?.sh".green());
                        },
                        _ => println!("Directory not found.")
                    }
                }
            },
            "clear" => {
                clear_console();
            },
            "cat" => {
                if let Some(filename) = parts.get(1) {
                    let filepath = format!("{}/{}", dir, filename);
                    match files.get(filepath.as_str()) {
                        Some(content) => println!("{}", content),
                        None => println!("{}: No such file", filename),
                    }
                } else {
                    println!("cat: missing operand");
                }
            },
            "cd" => {
                if let Some(target_dir) = parts.get(1) {
                    if target_dir.to_string() == "." {
                    } else if target_dir.to_string() == ".." {
                        if dir != "~" {
                            let parts = dir.rsplitn(2, '/').collect::<Vec<&str>>();
                            let new_dir = parts.last().unwrap_or(&"~");
                            dir = (*new_dir).to_string();
                        }
                    } else if target_dir.to_string() == "-" {
                        println!("not implemented :(");
                    } else {
                        let full_path = if target_dir.starts_with("/") {
                            target_dir.to_string()
                        } else {
                            format!("{}/{}", dir.trim_start_matches('~'), target_dir)
                        };

                        if (full_path == "/secret" && dir == "~") || available_dirs.contains(&full_path.as_str()) {
                            dir = if full_path.starts_with('/') { format!("~{}", full_path) } else { full_path.clone() };                        } else {
                            println!("{}: No such directory", target_dir);
                        }

                        if (full_path == "/more" && dir == "~") || available_dirs.contains(&full_path.as_str()) {
                            dir = if full_path.starts_with('/') { format!("~{}", full_path) } else { full_path };
                        } else {
                            println!("{}: No such directory", target_dir)
                        }
                    }
                } else {
                    dir = "~".to_string();
                }
            },
            "pass" => {
                if let Some(password) = parts.get(1) {
                    if password.to_string() == CORRECT_PASSWORD {
                        println!("Password guessed correctly!");
                        return true;
                    } else {
                        println!("Incorrect password.");
                    }
                } else {
                    println!("pass: missing password argument");
                }

                guesses -= 1;
            },
            "./ls?.sh" => {
                if dir == "~/secret" {
                    println!("./ls?.sh: line 27: unexpected EOF while looking for matching `\"'\n./ls?.sh: line 27: syntax error: unexpected end of file");
                    narrate = 1;
                }
                else {
                    println!("No such file or directory");
                }
            },
            "./.shadow" => {
                if dir == "~/secret" {
                    println!("nothing to see here...");
                    narrate = 4;
                }
            }
            "narrate" => {
                match narrate {
                    0 => println!("I should look around the file system first"),
                    1 => {
                        if shadow_discovered {
                            println!("I wonder what other commands there are. What's .shadow?");
                        } else {
                            println!("I wonder what other commands there are.");
                        }
                    },
                    2 => println!("Maybe the stuff at this address translates to something."),
                    3 => println!("I bet some other files have important data."),
                    4 => println!("There is probably data hidden in the file"),
                    _ => unreachable!()
                }
            },
            "disassemble" => {
                if parts.len() == 3 {
                    let file_key = parts[1];
                    let mut address = parts[2].to_lowercase();

                    // Remove the "0x" prefix from the address if it's present
                    if address.starts_with("0x") {
                        address = address.trim_start_matches("0x").to_string();
                    }

                    match file_key {
                        ".shadow" => handle_disassemble(file_key, &address, SHADOW, &mut narrate),
                        "ls?.sh" => handle_disassemble(file_key, &address, LS, &mut narrate),
                        _ => println!("Invalid file key: {}", file_key),
                    }
                } else {
                    println!("Missing argument(s):\nUsage: disassemble <file> <address>")
                }
            },
            "grep" => {
                if parts.len() >= 2 {
                    let pattern = parts[1];

                    // If a specific file is given, search only in that file
                    if parts.len() == 3 {
                        let file_name = parts[2];
                        let full_file_path = if file_name.starts_with("~/") {
                            file_name.to_string()
                        } else {
                            format!("{}/{}", dir, file_name)
                        };

                        if let Some(file_content) = files.get(full_file_path.as_str()) {
                            for (_line_number, line) in file_content.lines().enumerate() {
                                if line.contains(pattern) {
                                    println!("{}: {}", full_file_path, line);
                                }
                            }
                        } else {
                            println!("File not found: {}", full_file_path);
                        }
                    } else {
                        for (path, content) in files.iter() {
                            if path.starts_with(&dir) {
                                for line in content.lines() {
                                    if line.contains(pattern) {
                                        println!("{}: {}", path, line);
                                    }
                                }
                            }
                        }
                    }
                } else {
                    println!("Usage: grep <pattern> [file]");
                }
            },
            "man" => {
                if let Some(cmd) = parts.get(1) {
                    if cmd.to_string() == "ascii" {
                        println!("{}", ASCII);
                    } else {
                        println!("Man page not found.\nhttps://man7.org/linux/man-pages/");
                    }
                } else {
                    println!("Missing argument:\nUsage: man <file>");
                }
            },
            _ => println!("Command not found: {}", command)
        }
    }

    thread::sleep(Duration::from_secs(1));
    clear_console();
    println!("hmm this was pretty easy...");
    thread::sleep(Duration::from_secs(3));
    println!("strange...");
    thread::sleep(Duration::from_secs(3));

    return false
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

fn find_address_in_file(address: &str, file_content: &str) -> io::Result<Option<String>> {
    let reader = BufReader::new(file_content.as_bytes());

    for line in reader.lines() {
        let line = line?;
        if line.contains(address) {
            return Ok(Some(line));
        }
    }

    Ok(None)
}

fn handle_disassemble(file_key: &str, address: &str, file_content: &str, narrate: &mut i8) {
    match find_address_in_file(address, file_content) {
        Ok(Some(line)) => {
            *narrate = match file_key {
                ".shadow" => 2,
                "ls?.sh" => 3,
                _ => *narrate,
            };
            println!("{}", line);
        },
        Ok(None) => println!("Address not found/unreadable"),
        Err(e) => println!("Error: {}", e),
    }
}
