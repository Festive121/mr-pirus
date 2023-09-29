use std::thread;
use std::time::Duration;
use rand::Rng;
use std::process::Command;

fn main() {
    let mut seconds = 1;
    let mut minutes = 10;
    let n = rand::thread_rng().gen_range(0..100);

    clear_console();

    while (minutes + seconds) >= 0 {
        if seconds == 0 && minutes != 0{
            seconds = 59;
            minutes -= 1;
            thread::sleep(Duration::from_secs(1));
            clear_console();
        } else if seconds == 0 && minutes == 0 {
            end();
            break;
        } else {
            seconds -= 1;
            thread::sleep(Duration::from_secs(1));
            clear_console();
        }
        if count_digits(seconds) == 1 {
            println!("{}:0{}", minutes, seconds);
        } else if count_digits(minutes) == 1 {
            println!("0{}:{}", minutes, seconds);
        } else {
            println!("{}:{}", minutes, seconds);
        }

        if n == 0 {
            let img = rand::thread_rng().gen_range(0..3);
            // // Get the current executable's directory
            // let current_dir = env::current_dir().expect("Failed to get current directory");
            //
            // // Create the full path to the image
            // let image_path0 = current_dir.join("images").join("1 a33q f0zr p0afhygvat cym.png");
            // let image_path1 = current_dir.join("images").join("c00e q3f1ta.png");
            // let image_path2 = current_dir.join("images").join("choy1p q1fcy4l bs 1as0ez4g10a.png");
            // let image_path3 = current_dir.join("images").join("je1gg3a ol w1z clx3.png");
            //
            // let image_path_str;
            //
            // match img {
            //     1 => image_path_str = Some(image_path0.to_str().expect("Failed to convert path to string")),
            //     2 => image_path_str = Some(image_path1.to_str().expect("Failed to convert path to string")),
            //     3 => image_path_str = Some(image_path2.to_str().expect("Failed to convert path to string")),
            //     4 => image_path_str = Some(image_path3.to_str().expect("Failed to convert path to string")),
            //     _ => {
            //         println!("ERR IN IMG PATH MATCH");
            //         return;
            //     }
            // }
            //
            // if let Some(path) = image_path_str {
            //     Command::new("cmd")
            //         .args(&["/C", "start", path])
            //         .spawn()
            //         .expect("Failed to open image");
            // } else {
            //     println!("No valid image path was provided.");
            // }

            println!("random: {}", img);
        }
    }
}

fn end() {
    clear_console();
    println!("end");
}

fn count_digits(mut n: i32) -> usize {
    if n == 0 {
        return 1;
    }
    let mut count = 0;
    if n < 0 {
        n = -n;
    }
    while n > 0 {
        n /= 10;
        count += 1;
    }
    count
}

pub fn clear_console() {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "cls"])
            .status()
            .unwrap();
    } else {
        Command::new("clear")
            .status()
            .unwrap();
    }
}
