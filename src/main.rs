extern crate term;
extern crate rustyline;

use rustyline::error::ReadlineError;
use rustyline::Editor;

fn main () {
    let mut rl = Editor::<()>::new();

    loop {
        let name = "Josh";
        let mut who = String::from(name);
        who.push_str(": ");
        let readline = rl.readline(who.as_str());
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                let mut t = term::stdout().unwrap();
                t.fg(term::color::RED).unwrap();
                write!(t, "{}: ", name).unwrap();
                t.fg(term::color::GREEN).unwrap();
                write!(t, "{}", line).unwrap();

                t.reset().unwrap();
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
}
