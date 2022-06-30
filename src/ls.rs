extern crate chrono;
extern crate libc;
extern crate structopt;
extern crate colored; // not needed in Rust 2018


use std::fs;
use std::path::PathBuf;
use std::error::Error;
use std::process;
use std::os::unix::fs::PermissionsExt;
use std::os::linux::fs::MetadataExt;
use libc::{S_IRGRP, S_IROTH, S_IRUSR, S_IWGRP, S_IWOTH, S_IWUSR, S_IXGRP, S_IXOTH, S_IXUSR};
use colored::*;
use ansi_term::Style;

use structopt::StructOpt;
use chrono::{DateTime, Local};

#[derive(StructOpt, Debug)]
struct Opt 
{
    /// Output file
    #[structopt(default_value = ".", parse(from_os_str))]
    path: PathBuf,
}

fn parse_permissions(mode: u32) -> String 
{
    let user = triplet(mode, S_IRUSR, S_IWUSR, S_IXUSR);
    let group = triplet(mode, S_IRGRP, S_IWGRP, S_IXGRP);
    let other = triplet(mode, S_IROTH, S_IWOTH, S_IXOTH);
    [user, group, other].join("")
}

fn triplet(mode: u32, read: u32, write: u32, execute: u32) -> String 
{
    // o = 0, d = - = dash, r = r, w =w, x =x
    let ooo: &str = &format!("{}", "---".white()).to_string();
    let doo: &str = &format!("{}{}", "r".yellow(), "--").to_string();
    let owo: &str = &format!("{}{}{}", "-".white(), "w".red(), "-".white()).to_string();
    let ddx: &str = &format!("{}{}", "--".white(), "x".green()).to_string();
    let rdx: &str = &format!("{}{}{}", "r".yellow(), "-".white(), "x".green()).to_string();
    let rwd: &str = &format!("{}{}{}", "r".yellow(), "w".red(), "d".blue()).to_string();
    let dwx: &str = &format!("{}{}{}", "d".blue(), "w".red(), "x".green()).to_string();
    let rwx: &str = &format!("{}{}{}", "r".yellow(), "w".red(), "x".green()).to_string();
    match (mode & read, mode & write, mode & execute) 
    {
        (0, 0, 0) => ooo,
        (_, 0, 0) => doo,
        (0, _, 0) => owo,
        (0, 0, _) => ddx,
        (_, 0, _) => rdx,
        (_, _, 0) => rwd,
        (0, _, _) => dwx,
        (_, _, _) => rwx,
    }.to_string()
}

pub fn main(directory: String) 
{
    let mut path = PathBuf::new();
    path.push(directory);
    if let Err(ref e) = run(&path) 
    {
        println!("{}", e);
        process::exit(1);
    }
}

fn run(dir: &PathBuf) -> Result<(), Box<dyn Error>> 
{
    if dir.is_dir() 
    {
        println!("{}{} {} {} {} {}",
            "  ",
            Style::new().underline().paint("Inode").to_string(),
            Style::new().underline().paint("Permissions").to_string(),
            Style::new().underline().paint("Size").to_string(),
            Style::new().underline().paint("Date Modified").to_string(),
            Style::new().underline().paint("Name").to_string()
        );

        for entry in fs::read_dir(dir)? 
        {
            let entry = entry?;

            let metadata = entry.metadata()?;

            let file_name = entry.file_name().into_string().or_else(|f| Err(format!("Invalid entry: {:?}", f)))?;

            let size = metadata.len();

            let modified: DateTime<Local> = DateTime::from(metadata.modified()?);

            let mode = metadata.permissions().mode();

            let ino = metadata.st_ino();

            let uid = metadata.st_uid(); // user id
            
            let gid = metadata.st_gid(); // groud id

            println!("{} {} {:>5} {} {}", 
                    ino.to_string().magenta(),
                    parse_permissions(mode as u32), 
                    size.to_string().green(), 
                    modified.format("%_d %b %H:%M").to_string().red(), 
                    file_name.to_string().blue()
            );
        }
    }
    Ok(())
}

