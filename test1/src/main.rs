use std::fs;
use std::env;

fn main() {

    println!("curdir: {}, curexe: {}", env::current_dir().unwrap().display(), env::current_exe().unwrap().display());

    println!("cd to ../..\n");
    env::set_current_dir("../..").unwrap();
    println!("ls: {}", env::current_dir().unwrap().display());

    let paths = fs::read_dir("./").unwrap();
    for path in paths {
        let f = path.unwrap().path();
        println!("{} {}", if f.is_file() { "f" } else { "d" }, f.display());
    }

}
