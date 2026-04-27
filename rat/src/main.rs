use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut num_mod = 0;
    let mut files: Vec<String> = vec![];

    if args.len() > 1 {
        for arg in args.iter().skip(1) {
            match arg.trim() {
                "-n" => {
                    if num_mod != 2 {
                        num_mod = 1;
                    }
                }
                "-b" => num_mod = 2,
                _ => files.push(arg.to_string()),
            }
        }

        if files.len() < 1 {
            println!("Please specity a file!")
        } else {
            for file in files {
                println!("");
                println!("{}", file);
                println!("");

                let mut num = 1;
                let n_file = File::open(file);
                match n_file {
                    Ok(n_file) => {
                        let reader = BufReader::new(n_file);
                        for line in reader.lines() {
                            match line {
                                Ok(line) => match num_mod {
                                    0 => println!("{}", line),
                                    1 => {
                                        println!("{} {}", num, line);
                                        num += 1;
                                    }
                                    2 => match line.trim().is_empty() {
                                        true => {
                                            println!("{}", line)
                                        }
                                        false => {
                                            println!("{} {}", num, line);
                                            num += 1;
                                        }
                                    },
                                    _ => {}
                                },
                                Err(e) => {
                                    eprintln!("{}", e)
                                }
                            }
                        }
                    }
                    Err(e) => {
                        eprint!("{}", e);
                    }
                }
            }
        }
    } else {
        println!("Please specify a file!")
    }
}
