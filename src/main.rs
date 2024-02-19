use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

mod engine;

fn match_file(expr: &str, file_path: &str) -> Result<(), Box<dyn Error>> {
    let f = File::open(file_path)?;
    let reader = BufReader::new(f);
    for line in reader.lines() {
        let line = line?;
        for (i, _) in line.char_indices() {
            if engine::do_matching(expr, &line[i..])? {
                println!("{line}");
            }
        }
    }
    Ok(())
}
fn main() -> Result<(), Box<dyn Error>> {
    let args = std::env::args().collect::<Vec<String>>();
    println!("{}", args.len());
    match args.len() {
        3 => match_file(&args[1], &args[2]),
        _ => {
            eprintln!("usage: {} regex file", args[0]);
            Err("invalid arguments".into())
        }
    }
}
