use regex::Regex;
use std::process::{Command, Stdio};
use std::thread;

const URL: &str = "https://paijose.tk"; // Here you put the url

fn threads() {
    const NTHREADS: u32 = 1; // number of threads
    let mut children = vec![];

    for _i in 0..NTHREADS {
        children.push(thread::spawn(move || {
            let ou = Command::new("curl")
                .arg(URL)
                .stdout(Stdio::piped())
                .output()
                .expect("Failed to execute command");

            let re = Regex::new(r"(http(s?):)([/|.|\w|\s|-])*\.(?:jpg|gif|jpeg|png)").unwrap();
            let output = String::from_utf8(ou.stdout).unwrap();
            let formated = re.find_iter(&output).collect::<Vec<_>>();
            if formated.is_empty() {
                println!("No matches from: {}", URL);
            } else {
                for i in formated {
                    println!("{}", i.as_str())
                }
            }
        }));
    }

    for child in children {
        let _ = child.join();
    }
}

fn main() {
    println!("scraping: {}", URL);
    threads();
}
