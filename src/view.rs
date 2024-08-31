use colored::*;
use std::{
    io::{stdout, Write},
    thread::sleep,
    time::Duration,
};

pub const STR_WEEKDAY: [&str; 7] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
pub const STR_MONTH: [&str; 12] = [
    "Jan  ", "Feb ", "Mar  ", "Apr ", "May ", "Jun  ", "Jul ", "Aug ", "Sep  ", "Oct ", "Nov ",
    "Dec  ",
];

pub struct View;

impl View {
    pub fn display_cover(year: i32) {
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
            + &r"             \
                                   |    "
                .cyan()
                .to_string()
            + "looks like in command-line!"
            + &r"      |
                                    \   "
                .cyan()
                .to_string()
            + "Press [Enter] to page through."
            + &r"  /   
                                       ――――――――――――――――――――――――――――――――
                                                       \
                                                        \"
            .cyan()
            .to_string();

        let ferris = r"
                                                           __~^~^~__
                                                      \) /           \ (/
                                                        '_   0 ◡ 0   _'
                                                       \\   ¯¯¯¯¯¯¯   //";

        Self::clear();
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

    pub fn sub_title_with_keyword<T: ToString>(sub_title: &str, keyword: T) {
        Self::sub_title(
            &(sub_title.to_string()
                + " - "
                + &keyword.to_string().italic().underline().to_string()),
        );
    }

    pub fn content(str: &str) {
        Self::typewriter(str)
    }

    pub fn style_keyword<T: ToString>(keyword: T) -> ColoredString {
        keyword.to_string().cyan().bold()
    }

    pub fn wait() {
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        Self::clear();
    }

    // position the cursor at row 1, column 1
    pub fn clear() {
        println!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    }

    pub fn padding() {
        print!("    ");
    }

    pub fn line_break() {
        println!();
    }

    pub fn cyan_println(str: &str) {
        Self::typewriter(&str.cyan().bold().to_string());
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
        Self::line_break();
    }

    pub fn typewriter_for_line(s: &str) {
        for c in s.lines() {
            sleep(Duration::from_millis(150));
            Self::padding();
            println!("{c}");
            let _ = stdout().flush();
        }
    }

    pub fn display(s: &str) {
        for c in s.lines() {
            println!("    {c}")
        }
    }

    pub fn graph(graph_list: &[usize]) -> String {
        let mut res = format!(" {}\n", "―".repeat(109))
        +&("│  Jan       Feb     Mar     Apr       May     Jun     Jul       Aug     Sep     Oct       Nov     Dec         │\n").dimmed();
        for i in 0..=6 {
            res += "│ ";
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
            res += "   │\n";
        }
        res += &format!(" {}", "_".repeat(109));
        res
    }

    pub fn graph2(graph_list: &[usize]) -> String {
        let mut res = String::new();
        for i in 0..12 {
            res += STR_MONTH[i];
        }
        res = res + "\n";
        for i in 0..=6 {
            for j in 0..=52 {
                let ordinal = i + j * 7;
                if ordinal >= 365 {
                    res += " "
                } else {
                    res += &format!(
                        "{}",
                        match graph_list[ordinal] {
                            0 => " ".normal(),
                            1..=30 => "●".cyan().dimmed(),
                            31..=50 => "●".cyan(),
                            _ => "●".bright_cyan().bold(),
                        }
                    )
                }
            }
            res += "\n";
        }
        res
    }

    pub fn histogram<T: ToString>(index: T, count: usize, max: usize) {
        Self::typewriter_for_line(&format!(
            "{:<3} {}| {}",
            index.to_string().bold(),
            "#".repeat(count / (max / 90 + 1)).dimmed().bold(),
            if count == max {
                count.to_string()
            } else {
                count.to_string()
            },
        ));
    }

    pub fn histogram_command<T: ToString>(
        index: T,
        count: usize,
        len: usize,
        len_max: usize,
    ) -> String {
        format!(
            "{:<len$}  {}| {}",
            index.to_string(),
            "#".repeat((count as f64 * ((41 - len_max) as f64 / len as f64)) as usize),
            count,
            len = len_max,
        )
    }

    pub fn histogram_with_total<T: ToString>(index: T, count: usize, total: usize, max: usize) {
        if count == 0 {
            return;
        }
        Self::typewriter_for_line(&format!(
            "{:<125}{}",
            &format!(
                "{} {}| {:<5}",
                index.to_string().bold(),
                "#".repeat(count / (max / 80 + 1)).dimmed().bold(),
                count.to_string().bold()
            ),
            format!("[{:<4} total]", total).bright_black()
        ));
    }

    pub fn display_count_and_total(item: &String, count: usize, total: usize) {
        View::typewriter_for_line(&format!(
            "- {:<50} {:<6}{}",
            item.green().bold(),
            count,
            format!("[{:<4} total]", total,).bright_black()
        ));
    }

    pub fn hint_finish(year: i32) {
        Self::sub_title(&format!("All {} command line stats wrapped!", year));
        Self::typewriter_for_line(&format!(
            "Specify other years with arguments, such as `./cmd-wrapped {}`\n\n",
            year - 1
        ));
    }
}

pub struct Window {
    width: usize,
    padding: usize,
    display: fn(&str),
}

impl Window {
    pub fn new(width: usize, padding: usize, fn_display: fn(&str)) -> Window {
        View::line_break();
        Window {
            width,
            padding,
            display: fn_display,
        }
    }

    pub fn padding(&mut self, padding: usize) {
        self.padding = padding
    }

    pub fn edge(&self) {
        (self.display)(&format!(" {} \n", "―".repeat(self.width - 1)))
    }

    pub fn break_line(&self) {
        (self.display)(&format!("│{}│\n", " ".repeat(self.width)))
    }

    pub fn content(&self, s: &str) {
        for c in s.lines() {
            (self.display)(&format!(
                "│{}{:<width$}{}│\n",
                " ".repeat(self.padding),
                c,
                " ".repeat(self.padding),
                width = self.width - self.padding * 2,
            ))
        }
    }
}
