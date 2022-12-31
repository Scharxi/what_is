use owo_colors::{OwoColorize, colors::Red};

pub struct CLI;

impl CLI {
    pub fn get_input() -> String {
        let args = std::env::args().skip(1).collect::<Vec<String>>();

        if args.len() == 1 {
            eprintln!("{}: Incorecct syntax of command '{}'\n\t{}", "SYNTAX ERROR".fg::<Red>().bold(), "whatis".green(), "Usage: whatis <word>".green().bold());
            std::process::exit(1);
        }

        let mut word = String::new(); 
        
        if args[0] == "whatis" {
            word.push_str(args[1..].to_vec().join(" ").trim()); 
        } else {
            eprintln!("{}: Incorecct syntax of command '{}'\n\t{}", "SYNTAX ERROR".fg::<Red>().bold(), "whatis".green(), "Usage: whatis <word>".green().bold());
            std::process::exit(1);
        }

        return word.clone();
    }
    
}
