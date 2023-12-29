use colored::*;
use std::{
    io::{stdout, Write},
    thread::sleep,
    time::Duration,
};

const TITLE: &'static str = r#"
    ██████╗██╗     ██╗
   ██╔════╝██║     ██║
   ██║     ██║     ██║
   ╚██████╗███████╗██║
    ╚═════╝╚══════╝╚═╝
            
    ██╗    ██╗██████╗  █████╗ ██████╗ ██████╗ ███████╗██████╗ 
    ██║    ██║██╔══██╗██╔══██╗██╔══██╗██╔══██╗██╔════╝██╔══██╗
    ██║ █╗ ██║██████╔╝███████║██████╔╝██████╔╝█████╗  ██║  ██║
    ██║███╗██║██╔══██╗██╔══██║██╔═══╝ ██╔═══╝ ██╔══╝  ██║  ██║
    ╚███╔███╔╝██║  ██║██║  ██║██║     ██║     ███████╗██████╔╝
     ╚══╝╚══╝ ╚═╝  ╚═╝╚═╝  ╚═╝╚═╝     ╚═╝     ╚══════╝╚═════╝ "#;

pub struct View {}

impl View {
    pub fn display_title() {
        View::clear();
        let mut res = String::new();
        for c in TITLE.chars() {
            if c == '█' {
                res += &c.to_string();
            } else {
                res += &c.to_string().cyan().to_string();
            }
        }
        println!("{res}");
        Self::line_break();
        Self::sub_title(&"Find what's your 2023 looks like in command-line!".green().to_string());
        Self::wait();
    }

    pub fn sub_title(str: &str) {
        Self::cyan_println(str);
        Self::line_break();
    }

    pub fn content(str:&str){
        Self::scroll(str)
    }

    pub fn hint_continue() {
        Self::line_break();
        Self::line_break();
        Self::padding();
        println!(
            "{}",
            "PRESS [ENTER] TO CONTINUE"
                .bright_black()
                .bold()
                .to_string()
        );
    }

    pub fn wait() {
        Self::hint_continue();
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        Self::clear();
    }

    pub fn clear() {
        print!("{esc}[2J{esc}[1;1H\n\n\n", esc = 27 as char);
    }

    pub fn padding() {
        print!("    ");
    }

    pub fn line_break() {
        println!();
    }

    pub fn cyan_println(str: &str) {
        Self::padding();
        println!("{}", str.bright_cyan().bold());
    }

    pub fn scroll(s: &str) {
        Self::padding();
        for c in s.chars() {
            print!("{c}");
            stdout().flush().expect("Flushing to succeed");
            if c == '#' {
                sleep(Duration::from_millis(8));
            } else {
                sleep(Duration::from_millis(25));
            }
        }
        println!("");
    }
}
