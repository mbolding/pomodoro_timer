use std::fs::OpenOptions;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;
use std::process::Command;
use chrono::Local;

fn main() {
    println!("🍅 Pomodoro Timer");
    println!("=================\n");

    loop {
        println!("Choose an option:");
        println!("1. Start 25-minute work session");
        println!("2. Start 5-minute break");
        println!("3. Start 15-minute long break");
        println!("4. Start 1-minute test timer");
        println!("5. View session log");
        println!("6. Exit");
        print!("\nYour choice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => start_timer(25, "Work"),
            "2" => start_timer(5, "Short Break"),
            "3" => start_timer(15, "Long Break"),
            "4" => start_timer(1, "Test"),
            "5" => view_log(),
            "6" => {
                println!("Goodbye! 👋");
                break;
            }
            _ => println!("Invalid choice. Please try again.\n"),
        }
    }
}

fn start_timer(minutes: u32, session_type: &str) {
    println!("\n⏰ Starting {} session: {} minutes", session_type, minutes);
    println!("Press Ctrl+C to cancel\n");

    let start_time = Local::now();
    let total_seconds = minutes * 60;

    for remaining in (1..=total_seconds).rev() {
        let mins = remaining / 60;
        let secs = remaining % 60;
        print!("\r⏱️  Time remaining: {:02}:{:02}", mins, secs);
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_secs(1));
    }

    let end_time = Local::now();

    println!("\n\n🔔 {} session complete!", session_type);
    
    // Play alert sound and show notification
    play_alert(session_type);
    
    // Ask for optional note
    print!("\nAdd a note about what you did (press Enter to skip): ");
    io::stdout().flush().unwrap();
    let mut note = String::new();
    io::stdin().read_line(&mut note).unwrap();
    let note = note.trim();
    
    // Log the session with optional note
    log_session(session_type, minutes, &start_time, &end_time, note);
    
    println!("Session logged! ✓");
    println!("Press Enter to continue...");
    let mut _pause = String::new();
    io::stdin().read_line(&mut _pause).unwrap();
    println!();
}

fn play_alert(session_type: &str) {
    let message = format!("{} session complete!", session_type);
    
    // Cross-platform notification using native commands
    #[cfg(target_os = "macos")]
    {
        // macOS notification using osascript
        let _ = Command::new("osascript")
            .arg("-e")
            .arg(format!(
                "display notification \"{}\" with title \"🍅 Pomodoro Timer\" sound name \"Glass\"",
                message
            ))
            .spawn();
            
        // Play sound
        let _ = Command::new("afplay")
            .arg("/System/Library/Sounds/Glass.aiff")
            .spawn();
    }
    
    #[cfg(target_os = "linux")]
    {
        // Linux notification using notify-send
        let _ = Command::new("notify-send")
            .arg("🍅 Pomodoro Timer")
            .arg(&message)
            .spawn();
            
        // Play sound - try paplay first, then aplay
        let _ = Command::new("paplay")
            .arg("/usr/share/sounds/freedesktop/stereo/complete.oga")
            .spawn()
            .or_else(|_| {
                Command::new("aplay")
                    .arg("/usr/share/sounds/alsa/Front_Center.wav")
                    .spawn()
            });
    }
    
    // Terminal bell as fallback
    print!("\x07");
    io::stdout().flush().unwrap();
}

fn log_session(session_type: &str, duration: u32, start_time: &chrono::DateTime<Local>, end_time: &chrono::DateTime<Local>, note: &str) {
    let log_entry = if note.is_empty() {
        format!(
            "{} | {} | {} minutes | Started: {} | Ended: {}\n",
            start_time.format("%Y-%m-%d"),
            session_type,
            duration,
            start_time.format("%H:%M:%S"),
            end_time.format("%H:%M:%S")
        )
    } else {
        format!(
            "{} | {} | {} minutes | Started: {} | Ended: {} | Note: {}\n",
            start_time.format("%Y-%m-%d"),
            session_type,
            duration,
            start_time.format("%H:%M:%S"),
            end_time.format("%H:%M:%S"),
            note
        )
    };

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("pomodoro_log.txt")
        .expect("Unable to open log file");

    file.write_all(log_entry.as_bytes())
        .expect("Unable to write to log file");
}

fn view_log() {
    println!("\n📊 Session Log");
    println!("==============\n");

    match std::fs::read_to_string("pomodoro_log.txt") {
        Ok(contents) => {
            if contents.is_empty() {
                println!("No sessions logged yet.\n");
            } else {
                println!("{}", contents);
            }
        }
        Err(_) => {
            println!("No sessions logged yet.\n");
        }
    }

    println!("Press Enter to continue...");
    let mut _pause = String::new();
    io::stdin().read_line(&mut _pause).unwrap();
    println!();
}