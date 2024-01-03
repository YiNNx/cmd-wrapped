use colored::*;
use std::{
    io::{stdout, Write},
    thread::sleep,
    time::Duration,
};

lazy_static::lazy_static! {}

pub struct View {}

impl View {
    pub fn display_title(year: i32) {
        let title = r#"

        
         ██████╗ ███╗   ███╗ ██████╗ 
        ██╔════╝ ████╗ ████║ ██╔══██╗
        ██║      ██╔████╔██║ ██║  ██║
        ╚██████╗ ██║╚██╔╝██║ ██████╔╝
         ╚═════╝ ╚═╝ ╚═╝ ╚═╝ ╚═════╝
        ██╗     ██╗ ███████╗  ██████╗  ███████╗ ███████╗ ████████╗███████╗ 
        ██║     ██║ ██╔═══██╗██╔═══██╗ ██╔═══██╗██╔═══██╗██╔═════╝██╔═══██╗
        ██║ ██╗ ██║ ███████╔╝████████║ ███████╔╝███████╔╝██████╗  ██║   ██║
        ██║████╗██║ ██╔═══██╗██╔═══██║ ██╔════╝ ██╔════╝ ██╔═══╝  ██║   ██║
        ╚███╔═███╔╝ ██║   ██║██║   ██║ ██║      ██║      ████████╗███████╔╝
         ╚══╝ ╚══╝  ╚═╝   ╚═╝╚═╝   ╚═╝ ╚═╝      ╚═╝      ╚═══════╝╚══════╝ 


"#;

        let hi: String = String::new()
            + &r#"                      



                                       ――――――――――――――――――――――――――――――――
                                    /   "#
                .cyan()
                .to_string()
            + &format!("Find what your {}", year)
            + &r#"             \
                                   |    "#
                .cyan()
                .to_string()
            + "looks like in command-line!"
            + &r#"      |
                                    \   "#
                .cyan()
                .to_string()
            + "Press [Enter] to page through."
            + &r#"  /   
                                       ――――――――――――――――――――――――――――――――
                                                       \
                                                        \"#
            .cyan()
            .to_string();

        let ferris = r#"
                                                           __~^~^~__
                                                      \) /           \ (/
                                                        '_   0 ◡ 0   _'
                                                       \\   ¯¯¯¯¯¯¯   //"#;

        View::clear();
        let mut res = String::new();
        for c in title.chars() {
            if c == '█' {
                res += &c.to_string();
            } else {
                res += &c.to_string().cyan().to_string();
            }
        }
        println!("{res}");
        Self::line_break();
        print!("{}", &hi);
        print!("{}", &ferris.to_string().red().bold());
        Self::wait();
    }

    pub fn sub_title(str: &str) {
        Self::line_break();
        Self::cyan_println(str);
        Self::line_break();
    }

    pub fn content(str: &str) {
        Self::typewriter(str)
    }

    pub fn wait() {
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        Self::clear();
    }

    pub fn clear() {
        print!("{esc}[2J{esc}[1;1H\n", esc = 27 as char);
    }

    pub fn padding() {
        print!("    ");
    }

    pub fn line_break() {
        println!();
    }

    pub fn cyan_println(str: &str) {
        Self::padding();
        println!("{}", str.cyan().bold());
    }

    pub fn typewriter(s: &str) {
        Self::padding();
        for c in s.chars() {
            print!("{c}");
            let _ = stdout().flush();
            if c == '#' {
                sleep(Duration::from_millis(8));
            } else if c != ' ' {
                sleep(Duration::from_millis(25));
            }
        }
        println!("");
    }

    pub fn typewriter_for_line(s: &str) {
        for c in s.lines() {
            sleep(Duration::from_millis(150));
            Self::padding();
            println!("{c}");
            let _ = stdout().flush();
        }
    }

    pub fn graph(graph_list: &Vec<usize>) -> String {
        let mut res = format!(" {}\n", "―".repeat(110)).to_string()
        +&format!("│  Jan       Feb     Mar     Apr       May     Jun     Jul       Aug     Sep     Oct       Nov     Dec         │\n").dimmed();
        for i in 0..=6 {
            res += &format!("│ ");
            for j in 0..=52 {
                let ordinal = i + j * 7;
                if ordinal >= 365 {
                    res += "  "
                } else {
                    res += &format!(
                        "{:>2}",
                        match graph_list[ordinal] {
                            0 => "  ".normal(),
                            1..=30 => "■".cyan().dimmed(),
                            31..=50 => "■".cyan(),
                            _ => "■".bright_cyan().bold(),
                        }
                    )
                }
            }
            res += &format!("   │\n");
        }
        res += &format!(" {}", "_".repeat(110));
        res
    }
}
