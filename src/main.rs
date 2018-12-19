extern crate docopt;
extern crate serde_derive;
extern crate serde;

use serde_derive::{Deserialize};

mod error;
mod ips;

const USAGE: &'static str = r#"
ips-patch: IPS patch tool

Applies patch to data read from stdin, writes output to stdout.

Usage:
  ips-patch <patch>
  ips-patch --help
"#;

#[derive(Debug, Deserialize)]
struct Args {
    arg_patch: String,
}

fn main() {
    let args: Args = docopt::Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    match ips::patch(&args.arg_patch) {
        Ok(_) => (),
        Err(e) => {
            use std::io::Write;
            let stderr = std::io::stderr();
            writeln!(&mut stderr.lock(), "{}", e).unwrap();
            std::process::exit(1);
        }
    }
}
