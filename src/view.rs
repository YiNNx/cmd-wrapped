use colored::*;
use std::{
    io::{stdout, Write},
    thread::sleep,
    time::Duration,
};

const TITLE: &'static str = r#"
         ██████╗███╗   ███╗██████╗ 
        ██╔════╝████╗ ████║██╔══██╗
        ██║     ██╔████╔██║██║  ██║
        ╚██████╗██║╚██╔╝██║██████╔╝
         ╚═════╝╚═╝ ╚═╝ ╚═╝╚═════╝

        ██╗    ██╗██████╗  █████╗ ██████╗ ██████╗ ███████╗██████╗ 
        ██║    ██║██╔══██╗██╔══██╗██╔══██╗██╔══██╗██╔════╝██╔══██╗
        ██║ █╗ ██║██████╔╝███████║██████╔╝██████╔╝█████╗  ██║  ██║
        ██║███╗██║██╔══██╗██╔══██║██╔═══╝ ██╔═══╝ ██╔══╝  ██║  ██║
        ╚███╔███╔╝██║  ██║██║  ██║██║     ██║     ███████╗██████╔╝
         ╚══╝╚══╝ ╚═╝  ╚═╝╚═╝  ╚═╝╚═╝     ╚═╝     ╚══════╝╚═════╝ 

"#;

lazy_static::lazy_static! {
    static ref HI: String = format!(
    r#"
                                      ____________________________
                                    /  {}         \
                                    \  {} /
                                      ----------------------------
                                                     \
                                                      \"#,
    "Find what your 2023".white(),"looks like in command-line!".white()
    );
}

const FERRIS: &'static str = r#"
                                                          __~^~^~__
                                                     \) /           \ (/
                                                       '_   ⌾ ◡ ⌾   _'
                                                      \\   ¯¯¯¯¯¯¯   //


"#;

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
        print!("{}", &HI.to_string().cyan().bold());
        print!("{}", &FERRIS.to_string().red().bold());
        Self::wait_title();
    }

    pub fn sub_title(str: &str) {
        Self::line_break();
        Self::cyan_println(str);
        Self::line_break();
    }

    pub fn content(str: &str) {
        Self::typewriter(str)
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

    pub fn hint_continue_title() {
        Self::line_break();
        Self::line_break();
        Self::padding();
        Self::padding();
        print!(
            "{}",
            "PRESS [ENTER] TO CONTINUE"
                .white()
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

    pub fn wait_title() {
        Self::hint_continue_title();
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        Self::clear();
    }

    pub fn clear() {
        print!("{esc}[2J{esc}[1;1H\n\n", esc = 27 as char);
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

    pub fn typewriter(s: &str) {
        Self::padding();
        for c in s.chars() {
            print!("{c}");
            let _ = stdout().flush();
            if c == '#' {
                sleep(Duration::from_millis(8));
            } else {
                sleep(Duration::from_millis(25));
            }
        }
        println!("");
    }

    pub fn typewriter_for_line(s: &str) {
        for c in s.lines() {
            Self::padding();
            println!("{c}");
            let _ = stdout().flush();
            sleep(Duration::from_millis(100));
        }
        println!("");
    }
}
