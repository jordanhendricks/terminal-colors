// ascii ESC: 033
// 30-37: fg
// 40-47: bg

enum AnsiBasic {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

impl AnsiBasic {
    fn colorized_msg(self, msg: &str) -> String {
        let mut s = String::new();
        s.push_str("\x1b[");

        let code = match self {
            AnsiBasic::Black => "30",
            AnsiBasic::Red => "31",
            AnsiBasic::Green => "32",
            AnsiBasic::Yellow => "33",
            AnsiBasic::Blue => "34",
            AnsiBasic::Magenta => "35",
            AnsiBasic::Cyan => "36",
            AnsiBasic::White => "37",
        };

        s.push_str(code);
        s.push_str("m");
        s.push_str(msg);

        // reset color
        s.push_str("\x1b[0m");

        s
    }

    fn colorized_bg(self, msg: &str) -> String {
        let mut s = String::new();
        s.push_str("\x1b[");

        let code = match self {
            AnsiBasic::Black => "40",
            AnsiBasic::Red => "41",
            AnsiBasic::Green => "42",
            AnsiBasic::Yellow => "43",
            AnsiBasic::Blue => "44",
            AnsiBasic::Magenta => "45",
            AnsiBasic::Cyan => "46",
            AnsiBasic::White => "47",
        };

        s.push_str(code);
        s.push_str("m");
        s.push_str(msg);

        // reset color
        s.push_str("\x1b[0m");

        s
    }

    fn colorized_fg_bg(fg: AnsiBasic, bg: AnsiBasic, msg: &str) -> String {
        let mut s = String::new();

        // foreground string
        s.push_str("\x1b[");
        let code = match fg {
            AnsiBasic::Black => "30",
            AnsiBasic::Red => "31",
            AnsiBasic::Green => "32",
            AnsiBasic::Yellow => "33",
            AnsiBasic::Blue => "34",
            AnsiBasic::Magenta => "35",
            AnsiBasic::Cyan => "36",
            AnsiBasic::White => "37",
        };
        s.push_str(code);
        s.push_str("m");

        // background string
        s.push_str("\x1b[");
        let code = match bg {
            AnsiBasic::Black => "40",
            AnsiBasic::Red => "41",
            AnsiBasic::Green => "42",
            AnsiBasic::Yellow => "43",
            AnsiBasic::Blue => "44",
            AnsiBasic::Magenta => "45",
            AnsiBasic::Cyan => "46",
            AnsiBasic::White => "47",
        };
        s.push_str(code);
        s.push_str("m");

        s.push_str(msg);

        // reset color
        s.push_str("\x1b[0m");

        s
    }
}

fn main() {
    println!("----------------------------");
    println!("VISUALIZING TERMINAL COLORS");
    println!("----------------------------");

    println!("ANSI COLORS (FOREGROUND)");
    println!("{}", AnsiBasic::colorized_msg(AnsiBasic::Black, "BLACK"));
    println!("{}", AnsiBasic::colorized_msg(AnsiBasic::Red, "RED"));
    println!("{}", AnsiBasic::colorized_msg(AnsiBasic::Green, "GREEN"));
    println!("{}", AnsiBasic::colorized_msg(AnsiBasic::Yellow, "YELLOW"));
    println!("{}", AnsiBasic::colorized_msg(AnsiBasic::Blue, "BLUE"));
    println!("{}", AnsiBasic::colorized_msg(AnsiBasic::Magenta, "MAGENTA"));
    println!("{}", AnsiBasic::colorized_msg(AnsiBasic::Cyan, "CYAN"));
    println!("{}", AnsiBasic::colorized_msg(AnsiBasic::White, "WHITE"));

    println!("");
    println!("ANSI COLORS (BACKGROUND)");
    println!("{}", AnsiBasic::colorized_bg(AnsiBasic::Black, "BLACK"));
    println!("{}", AnsiBasic::colorized_bg(AnsiBasic::Red, "RED"));
    println!("{}", AnsiBasic::colorized_bg(AnsiBasic::Green, "GREEN"));
    println!("{}", AnsiBasic::colorized_bg(AnsiBasic::Yellow, "YELLOW"));
    println!("{}", AnsiBasic::colorized_bg(AnsiBasic::Blue, "BLUE"));
    println!("{}", AnsiBasic::colorized_bg(AnsiBasic::Magenta, "MAGENTA"));
    println!("{}", AnsiBasic::colorized_bg(AnsiBasic::Cyan, "CYAN"));
    println!("{}", AnsiBasic::colorized_bg(AnsiBasic::White, "WHITE"));

    println!("");
    println!("ANSI COLORS (FG: BLACK)");
    println!("{}", AnsiBasic::colorized_fg_bg(AnsiBasic::Black, AnsiBasic::Red, "BLACK ON RED"));
    println!("{}", AnsiBasic::colorized_fg_bg(AnsiBasic::Black, AnsiBasic::Green, "BLACK ON GREEN"));
    println!("{}", AnsiBasic::colorized_fg_bg(AnsiBasic::Black, AnsiBasic::Yellow, "BLACK ON YELLOW"));
    println!("{}", AnsiBasic::colorized_fg_bg(AnsiBasic::Black, AnsiBasic::Blue, "BLACK ON BLUE"));
    println!("{}", AnsiBasic::colorized_fg_bg(AnsiBasic::Black, AnsiBasic::Magenta, "BLACK ON MAGENTA"));
    println!("{}", AnsiBasic::colorized_fg_bg(AnsiBasic::Black, AnsiBasic::Cyan, "BLACK ON CYAN"));
    println!("{}", AnsiBasic::colorized_fg_bg(AnsiBasic::Black, AnsiBasic::White, "BLACK ON WHITE"));

    println!("");
    println!("ANSI COLORS (FG: RED)");
    println!("{}", AnsiBasic::colorized_fg_bg(AnsiBasic::Red, AnsiBasic::Green, "RED ON GREEN"));
    println!("{}", AnsiBasic::colorized_fg_bg(AnsiBasic::Red, AnsiBasic::Yellow, "RED ON YELLOW"));
    println!("{}", AnsiBasic::colorized_fg_bg(AnsiBasic::Red, AnsiBasic::Blue, "RED ON BLUE"));
    println!("{}", AnsiBasic::colorized_fg_bg(AnsiBasic::Red, AnsiBasic::Magenta, "RED ON MAGENTA"));
    println!("{}", AnsiBasic::colorized_fg_bg(AnsiBasic::Red, AnsiBasic::Cyan, "RED ON CYAN"));
    println!("{}", AnsiBasic::colorized_fg_bg(AnsiBasic::Red, AnsiBasic::White, "RED ON WHITE"));

    println!("");
    println!("ANSI COLORS (FG: GREEN)");
    println!("{}", AnsiBasic::colorized_fg_bg(AnsiBasic::Green, AnsiBasic::Yellow, "GREEN ON YELLOW"));
    println!("{}", AnsiBasic::colorized_fg_bg(AnsiBasic::Green, AnsiBasic::Blue, "GREEN ON BLUE"));
    println!("{}", AnsiBasic::colorized_fg_bg(AnsiBasic::Green, AnsiBasic::Magenta, "GREEN ON MAGENTA"));
    println!("{}", AnsiBasic::colorized_fg_bg(AnsiBasic::Green, AnsiBasic::Cyan, "GREEN ON CYAN"));
    println!("{}", AnsiBasic::colorized_fg_bg(AnsiBasic::Green, AnsiBasic::White, "GREEN ON WHITE"));

    println!("");
    println!("ANSI COLORS (FG: YELLOW)");
    println!("{}", AnsiBasic::colorized_fg_bg(AnsiBasic::Yellow, AnsiBasic::Blue, "YELLOW ON BLUE"));
    println!("{}", AnsiBasic::colorized_fg_bg(AnsiBasic::Yellow, AnsiBasic::Magenta, "YELLOW ON MAGENTA"));
    println!("{}", AnsiBasic::colorized_fg_bg(AnsiBasic::Yellow, AnsiBasic::Cyan, "YELLOW ON CYAN"));
    println!("{}", AnsiBasic::colorized_fg_bg(AnsiBasic::Yellow, AnsiBasic::White, "YELLOW ON WHITE"));

    println!("");
    println!("ANSI COLORS (FG: MAGENTA)");
    println!("{}", AnsiBasic::colorized_fg_bg(AnsiBasic::Magenta, AnsiBasic::Cyan, "MAGENTA ON CYAN"));
    println!("{}", AnsiBasic::colorized_fg_bg(AnsiBasic::Magenta, AnsiBasic::White, "MAGENTA ON WHITE"));

    println!("");
    println!("ANSI COLORS (FG: CYAN)");
    println!("{}", AnsiBasic::colorized_fg_bg(AnsiBasic::Cyan, AnsiBasic::White, "CYAN ON WHITE"));
}
