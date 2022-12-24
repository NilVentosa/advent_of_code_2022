use clap::{Arg, Command};
use reqwest::blocking::Client;
use std::env;
use std::error::Error;
use std::fs;
use std::io::prelude::*;
use std::path::Path;
use std::process::Command as ProcessCommand;

fn main() {
    let matches = Command::new("AOC")
        .author("Nil Ventosa")
        .version("0.1.0")
        .arg(
            Arg::new("day")
                .short('d')
                .long("day")
                .required(true)
                .help("the day to generate"),
        )
        .get_matches();

    let day_number = matches.get_one::<String>("day").unwrap().to_string();
    let day_folder = format!("day{}", day_number);

    if let Err(e) = create_cargo_project(&day_folder) {
        eprintln!("Error: {e}");
        return;
    }
    if let Err(e) = create_input_files(&day_number) {
        eprintln!("Error: {e}");
        return;
    }
    if let Err(e) = copy_main_file(&day_folder) {
        eprintln!("Error: {e}");
        return;
    }
}

fn create_input_files(day_number: &str) -> Result<(), Box<dyn Error>> {
    let cookie = env::var("AOC_COOKIE")?;

    println!("Downloading input file");
    let response = Client::new()
        .get(format!(
            "https://adventofcode.com/2022/day/{}/input",
            &day_number
        ))
        .header("Cookie", cookie)
        .send()?
        .text()?;

    fs::File::create(format!("day{}/test_input.txt", &day_number))?;

    fs::File::create(format!("day{}/input.txt", &day_number))
        .unwrap()
        .write_all(response.as_bytes())?;

    Ok(())
}

fn create_cargo_project(day_folder: &str) -> Result<String, String> {
    println!("Creating cargo project");
    if Path::new(&day_folder).exists() {
        return Err(format!("{} already exists.", day_folder));
    }

    println!("- Creating {}", &day_folder);

    match ProcessCommand::new("cargo")
        .arg("new")
        .arg(day_folder)
        .output()
    {
        Ok(_output) => Ok(format!("- {} created", &day_folder)),
        Err(error) => Err(format!("- Error creating {}: {}", &day_folder, error)),
    }
}

fn copy_main_file(day_folder: &str) -> Result<(), String> {
    println!("Copying main.rs file");
    match fs::copy("resources/day.rs", format!("{}/src/main.rs", &day_folder)) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Error copying main file: {}", e)),
    }
}
