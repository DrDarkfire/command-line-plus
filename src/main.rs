use std::{env, path::PathBuf, path::Path, ops::Deref, io::{self, Write}, collections::HashMap};
mod functions;
//use colored::*;
use dirs;

// the list of available commands. would love to know how to dynamically implement this in safe rust
static COMMANDS: [&str; 3] = [
    "touch", 
    "ls",
    "cd"
    ];

/// FIRST ITERATION OF COMMANDLINEPLUS
/// The idea is that you can either:
/// - run a one off function like ls or touch
/// - maybe eventually have a rudimentary full terminal functionality on windows like you would linux. 
/// This 'product' is meant as a learning exercise in Rust.
/// It will need a full rewrite eventually to make the terminal 'engine' run better.
/// Currently flags aren't supporded
fn main() {
    // get our commandline args
    let args: Vec<String> = env::args().skip(1).collect();

    // check if args has anything in it. If so, end the program. Later we can use this as an alternative to clp run
    if args.is_empty() || String::from("run") == String::from(args.first().unwrap()) {
        // println!("{}", "Command-Line-Plus cannot be used without a function to run currently".red());
        env_process(None);
        return;
    }

    let path = String::from(args.first().unwrap());

    if is_path_guard(&path) {
        env_process(Some(&PathBuf::from(path)));
    }

    if let Ok(cwd) = env::current_dir() {
        process(&cwd, &args)
    }
}

#[allow(unused_variables)]
fn process(cwd: &PathBuf, args: &Vec<String>) {
    // call the hashmap maker

    // unknown function guard clause

    // check which function we're using. if it doe
}

#[allow(unused_mut)]
fn env_process(path: Option<&PathBuf>) {
    // declare cwd as a mutable PathBuf struct
    let mut cwd: PathBuf;

    let mut fns: HashMap<&str, fn(&PathBuf, &Vec<&str>) -> Option<PathBuf>> = HashMap::new();
    fns.insert("touch", functions::touch);
    fns.insert("ls", functions::ls);
    fns.insert("cd", functions::cd);

    // if we are given a path, we assign it. otherwise use the person's home directory
    if let Some(path) = path {
        cwd = PathBuf::from(path);
    } else {
        cwd = dirs::home_dir().expect("No home directory found.");
    }

    // initialize our input variable
    let mut input = String::new();

    loop {
        print!("{}>", cwd.to_string_lossy()); // print the current dir we're acting on
        io::stdout().flush().expect("that shouldn't have happened?");
        io::stdin().read_line(&mut input).unwrap();
        //println!("You entered: {}", input);
        // split the string based on whitespace, separate the command in prep for the other commands
        let mut input_vec: Vec<&str> = input.split_whitespace().collect();
        let mut cmd: String = input_vec.remove(0).to_string();

        // guard clauses for readability
        if quit_guard(&cmd) {
            std::process::exit(0);
        }

        // this is wrong and we should just add the error to an else to the main if statement
        if unknown_fn_guard(&cmd) {
            println!("Unknown function! Please check your spelling or consult the documentation or type \"help\"");
        }

        // we are now ok and can run the function
        // OLD VERSION
        // if let Some(func) = fns.get(&cmd.deref()) {
        //     func(&cwd, &input_vec);
        // }

        // NEW VERSION
        if let Some(func) = fns.get(&cmd.deref()) {
            let n_cwd: Option<PathBuf> = func(&cwd, &input_vec);
            // if n_cwd has a value. assign it to cwd
            if n_cwd.is_some() {
                cwd = n_cwd.unwrap();
            }
        }

        //println!("The command we are running is: {}", cmd);
        input = String::new();
    }
}

fn unknown_fn_guard(func: &String) -> bool {
    if COMMANDS.contains(&func.as_str()) {
        return false
    }
    true
}

fn is_path_guard(path: &String) -> bool {
    if Path::new(&path).is_absolute() {
        return true
    }
    false
}

/// fn quit_guard
/// returns true if a quit command is received
fn quit_guard(s: &String) -> bool {
    let str: String = String::from(s.deref());
    if str == "q" || str == "quit" {
        return true
    }
    false
}