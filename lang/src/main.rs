//! lang — run Lang.P programs: `lang run hello.lp`

use langc_cli::{run, CliFlavor};
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if let Err(e) = run(&args, CliFlavor::Lang) {
        eprintln!("{e}");
        process::exit(1);
    }
}
