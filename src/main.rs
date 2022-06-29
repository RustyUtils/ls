// compiler settings
#![allow(non_snake_case)]
#[warn(unused_variables)]
#[warn(unused_imports)]
// imports
use std::env;
// file imports
#[path = "./ls.rs"] mod ls;
// crate imports
#[macro_use] extern crate chrono;
#[macro_use] extern crate libc;
#[macro_use] extern crate structopt;
#[macro_use] extern crate colored; // not needed in Rust 2018

pub fn main() 
{
    // get command line arguments
    let args: Vec<String> = env::args().skip(1).collect(); // skip ./biglet
    let _args_modifer: Vec<String> = env::args().skip(2).collect(); // skip ./biglet and -_modifier
    let arg_length: u8 = args.len().try_into().unwrap();
    let arg_string: String; // a added up of command line args
    let directory: String; // directory
    // argument checking
    if !(arg_length > 0) 
    {
        directory = "./".to_string();
        ls::main(directory);
    }
    // argument checking starts here
    else 
    {
        let modifier: &str = &args[0]; // so dont have to call on args[0] multiple times

        if modifier.eq("-h") || modifier.eq("--help") // if arg1 is equal to -h
        {
            println!("USAGE: \n\tls or ls [args] or ls [options(optional)] [args]");
            println!("ARGS: \n\t<args>... \n\t <args> contain a path for ex: /usr/bin");
            println!("OPTIONS: ");
            println!("\t-h or --help: Help sheet");
            println!("EXAMPLES: ");
            println!("\tls \n\tls /usr/bin");
            println!("-----------------------");
            println!("Program made by Thamognya Kodi AGPL3.0-or-later");
            println!("Source Code: ");
        } 
        else
        {
            // more than one arg case like ls /bin /usr/bin
            if arg_length > 1
            {
                // spilt the arg_string with each space to each directory i.e ls /usr/bin /bin => ls /usr/bin first then ls /bin
                arg_string = args[0..].join("");
                println!("{}", arg_string);
                println!("Hell0");
            }
            // only one arg case like ls /bin
            else
            {
                // convert &str modifier to String arg_string
                arg_string = modifier.to_string();
                directory = arg_string;
                ls::main(directory);
            }
        }
    }
}
