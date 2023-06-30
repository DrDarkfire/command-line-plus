use std::{fs, ops::Deref};
#[allow(unused)]
use std::{path::Path, path::PathBuf, fs::File, process::Command, env};

#[allow(unused)]
pub fn ls(cwd: &PathBuf, flags: &Vec<&str>) -> Option<PathBuf>{
    // get all the files and directories in our path. this is something we have to do regardless of flags
    let paths = fs::read_dir(cwd.as_path());
    // if flags.contains(&"v") {
    //     return ls_verbose(cwd);
    // } else {
    //     for path in paths {
    //         let f = path?.file_name();
    //         println!("{}", f.to_string_lossy());
    //     }
    //     return 0;
    // }
    println!("Not yet implemented!");
    return None;
}

#[allow(unused)]
fn ls_verbose(cwd: &PathBuf) -> i8 {
    return 0
}

/// pub fn touch
/// built for abusive file creation currently
/// NO CHECKS SO BE SAFE PLOX
pub fn touch(cwd: &PathBuf, args: &Vec<&str>) -> Option<PathBuf>{
    for s in args {
        File::create(cwd.join(s)).unwrap();
    }
    
    return None;
}

#[allow(unused)]
/// pub fn cd
/// change directory
/// the checks are not robust enough yet so please be gentle
pub fn cd(cwd: &PathBuf, args: &Vec<&str>) -> Option<PathBuf>{
    // early quit if we have an inappropriate # of args
    if args.len() != 1 {
        return None
    }
    // get us a path we can work with and the arg
    let mut p: PathBuf = PathBuf::from(cwd);
    let arg = args.first().unwrap().deref();
    
    // if first arg is ".." and p is not a prefix only pop the pathbuf
    if args.first().unwrap().deref() == ".." && !p.is_prefix() {
        p.pop();
        return Some(p)
    }
    
    // if arg is a prefix try to change drives
    if arg.is_prefix() {        
        return Some(PathBuf::from(format!("{}\\", arg)));
    }
    // if arg exists in file system then push
    // create a temp path that's a clone
    let mut p2 = p.clone();
    p2.push(arg);
    if p2.exists() {
        return Some(p2);
    }
    return Some(p);
}

/// pub fn mv
/// move (cut and paste)
#[allow(unused)]
pub fn mv(cwd: &PathBuf, args: &Vec<&str>) {
    nyi()
}

/// pub fn cp
/// copy
#[allow(unused)]
pub fn cp(cwd: &PathBuf, args: &Vec<&str>) {
    nyi()
}

/// pub fn zip
/// zip
#[allow(unused)]
pub fn zip(cwd: &PathBuf, args: &Vec<&str>) {
    nyi()
}

/// pub fn unzip
/// unzip
#[allow(unused)]
pub fn unzip(cwd: &PathBuf, args: &Vec<&str>) {
    nyi()
}

/// pub fn df
/// disk/filesystem info
#[allow(unused)]
pub fn df(cwd: &PathBuf, args: &Vec<&str>) {
    nyi()
}

/// pub fn chmod
/// change permissions
#[allow(unused)]
pub fn chmod(cwd: &PathBuf, args: &Vec<&str>) {
    nyi()
}

/// download files from the internet into 
#[allow(unused)]
pub fn wget(cwd: &PathBuf, args: &Vec<&str>) {
    nyi()
}

/// remove files
#[allow(unused)]
pub fn rm(cwd: &PathBuf, args: &Vec<&str>) {
    nyi()
}

/// remove directories
#[allow(unused)]
pub fn rmdir(cwd: &PathBuf, args: &Vec<&str>) {
    nyi()
}

/// rename singular file currently
/// perhaps a matching system where the args get split in 2 and then processed accordingly
#[allow(unused)]
pub fn rename(cwd: &PathBuf, args: &Vec<&str>) {

}

/// open a file with the system's default text editor
/// usage: edit [filename] OR edit [path\to\file\filename]
/// notes:
/// - you should be within the directory of the file or at least the same disk currently
/// - use your system's directory separator ( '\' for windows )
#[allow(unused)]
pub fn edit(cwd: &PathBuf, args: &Vec<&str>) {
    // if not enough arguments we return early
    if nea_guard(2, args.len().try_into().unwrap()) {
        return;
    }
    // get the system default editor
    let editor = env::var("EDITOR").unwrap();

    let mut cmd = Command::new(editor);
    cmd.arg(args.first().unwrap());
}

fn nyi() {
    println!("Not yet implemented!");
}

fn nea_guard(expected_args: u8, given_args: u8) -> bool {
    if given_args < expected_args {
        return true
    }
    false
}


/// PREFIX TRAIT
/// meant for practice.
/// is it practical to do it this way? probably not
/// checks if the item is a drive prefix only

trait Prefix {
    fn is_prefix(&self) -> bool;
}

impl Prefix for PathBuf {
    fn is_prefix(&self) -> bool {
        self.is_absolute() && self.components().count() == 1
    }
}

impl Prefix for &str {
    fn is_prefix(&self) -> bool {
        self.len() == 2 && self.starts_with(r"[a-zA-Z]:")
    }
}