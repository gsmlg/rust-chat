extern crate term;
extern crate rustyline;
extern crate reqwest;

use std::thread;
use rustyline::error::ReadlineError;
use rustyline::Editor;

fn main () {
    let mut rl = Editor::<()>::new();

    thread::spawn(move || {
        let resp = reqwest::get("http://localhost:8088/room/1").unwrap();
        let mut t = term::stdout().unwrap();
        t.fg(term::color::BLUE).unwrap();
        write!(t, "{:?}", resp.status());
        if resp.status().is_success() {
            println!("success!");
        } else if resp.status().is_server_error() {
            println!("server error!");
        } else {
            println!("Something else happened. Status: {:?}", resp.status());
        }
        t.reset().unwrap();
    });

    loop {
        let name = "Josh";
        let mut who = String::from(name);
        who.push_str(": ");
        let readline = rl.readline(who.as_str());
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());

                let client = reqwest::Client::new();
                let _res = client.post("http://localhost:8088/room/1")
                    .body(line.clone())
                    .send()
                    .unwrap();

                let mut t = term::stdout().unwrap();
                t.fg(term::color::RED).unwrap();
                write!(t, "{}: ", name).unwrap();
                t.fg(term::color::GREEN).unwrap();
                write!(t, "{}\n", line).unwrap();
                // t.fg(term::color::BLUE).unwrap();
                // write!(t, "{}", res.text());
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
