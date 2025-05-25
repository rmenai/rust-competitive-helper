mod codeforces;
mod dmoj;
mod kattis;
mod oj;

// Changed from clipboard to arboard
use arboard::Clipboard;
use crossterm::execute;
use crossterm::style::{Color, ResetColor, SetForegroundColor};
use rust_competitive_helper_util::{read_from_file, read_lines};
use std::process::Command;

pub fn submit() {
    let file = "main/src/main.rs";
    let url = read_lines(file)
        .unwrap()
        .into_iter()
        .next()
        .unwrap()
        .split_at(2)
        .1
        .trim()
        .to_string();
    let site = url.split('/').nth(2).unwrap_or("Manual");
    match site {
        "codeforces.com" => {
            if codeforces::submit(&url) {
                return;
            }
        }
        "atcoder.jp" | "www.hackerrank.com" | "yukicoder.me" => {
            oj::submit(&url);
            return;
        }
        "open.kattis.com" => {
            kattis::submit(&url);
            return;
        }
        "dmoj.ca" => {
            dmoj::submit(&url);
            return;
        }
        _ => {}
    }
    println!("Unsupported site, code copied to clipboard: {}", site);

    // Changed from clipboard::ClipboardContext to arboard::Clipboard
    let mut clipboard = match Clipboard::new() {
        Ok(cb) => cb,
        Err(e) => {
            eprintln!("Failed to initialize clipboard: {}", e);
            // Optionally, handle the error more gracefully, e.g., by not attempting to use the clipboard
            // For now, we'll panic as the original code did with unwrap()
            panic!("Clipboard initialization failed");
        }
    };

    match read_from_file("main/src/main.rs") {
        Some(content) => {
            if let Err(e) = clipboard.set_text(content) {
                eprintln!("Failed to set clipboard contents: {}", e);
            }
        }
        None => {
            eprintln!("Failed to read file for clipboard");
        }
    }
}

fn success(s: &str) -> usize {
    let mut stdout = std::io::stdout();
    let _ = execute!(stdout, SetForegroundColor(Color::Green));
    println!("{s}");
    let _ = execute!(stdout, ResetColor);
    s.len()
}

fn failure(s: &str) {
    let mut stdout = std::io::stdout();
    let _ = execute!(stdout, SetForegroundColor(Color::Red));
    println!("{s}");
    let _ = execute!(stdout, ResetColor);
}

fn pending(s: &str) -> usize {
    let mut stdout = std::io::stdout();
    let _ = execute!(stdout, SetForegroundColor(Color::Yellow));
    print!("{s}");
    let _ = execute!(stdout, ResetColor);
    s.len()
}

fn check_available(name: &str) -> bool {
    let which_output = Command::new("which").arg(name).output().unwrap();
    // It's better to check the status code directly rather than asserting and then checking string output
    if !which_output.status.success() {
        return false;
    }
    // If `which` succeeds, it means the command is found.
    // The original check for "which: no {} in" is specific to some `which` implementations
    // and might not be robust. A successful exit code is a more reliable indicator.
    // However, to keep behavior closer to original if that specific string check was important:
    // !String::from_utf8_lossy(&which_output.stdout).starts_with(&format!("which: no {} in", name))
    // For a more robust check of command availability:
    true // If status.success() is true, the command is generally considered available.
}
