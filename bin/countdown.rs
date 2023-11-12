use rand::Rng;
use std::{fs, io::Write, process::Command, thread, time::Duration};
use tempfile::NamedTempFile;
use std::sync::mpsc;

fn main() {
    let mut seconds = 1;
    let mut minutes = 10;

    // embed images
    const IMG_DATA_1: &'static [u8] = include_bytes!("../images/1 a33q f0zr p0afhygvat cym.png");
    const IMG_DATA_2: &'static [u8] = include_bytes!("../images/c00e q3f1ta.png");
    const IMG_DATA_3: &'static [u8] = include_bytes!("../images/choy1p q1fcy4l bs 1as0ez4g10a.png");
    const IMG_DATA_4: &'static [u8] = include_bytes!("../images/je1gg3a ol w1z clx3.png");

    clear_console();

    while (minutes + seconds) >= 0 {
        let n = rand::thread_rng().gen_range(0..50);

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
            let img = rand::thread_rng().gen_range(1..=4);
            let img_data = match img {
                1 => IMG_DATA_1,
                2 => IMG_DATA_2,
                3 => IMG_DATA_3,
                4 => IMG_DATA_4,
                _ => unreachable!(),
            };

            // Spawn a new thread for handling the image display

            thread::spawn(move || {
                let temp_file = NamedTempFile::new().expect("Failed to create temporary file");
                temp_file.as_file().write_all(img_data).expect("Failed to write image data to temporary file");

                // Get the path of the temporary file
                let temp_path = temp_file.into_temp_path();

                // Create a new path with the .png extension
                let img_path = temp_path.with_extension("png");

                // Rename the temporary file to have the .png extension
                fs::rename(&temp_path, &img_path).expect("Failed to rename temporary file");

                let img_path_str = img_path.to_str().expect("Failed to convert path to string");

                Command::new("cmd")
                    .args(&["/C", "start", img_path_str])
                    .spawn()
                    .expect("Failed to open image");

                // Keep the file from being deleted for a longer period, adjust as needed
                thread::sleep(Duration::from_secs(60));
            });
        }
    }
}

fn end() {
    let (tx, _rx) = mpsc::channel();
    let tx_thread = tx.clone();

    clear_console();

    tx_thread.send("Timer end").unwrap();
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
