use espers::plugin::Plugin;
use espers::records::{Group, Record};

use std::fs::File;

use anyhow::Error;
use clap::Parser;

/// Dump contents of `*.es[mp]` files
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Path to plugin file
    path: String,

    /// Print only malformed records
    #[clap(long)]
    errors: bool,

    /// Panic at the first malformed record
    #[clap(long)]
    panic: bool,
}

pub fn dump(group: &Group, indent: usize, args: &Args) {
    if !args.errors {
        println!("{:indent$}{}", "", group);
    }

    for record in &group.records {
        match record {
            Ok(Record::Group(g)) => dump(g, indent + 1, args),
            Ok(record) => {
                let i = indent + 1;
                if !args.errors {
                    println!("{:i$}{}", "", record);
                }
            }
            Err(err) => {
                let i = indent + 1;

                if args.panic {
                    panic!("{:i$}Error reading file: {}", "", err);
                } else {
                    println!("{:i$}Error reading file: {}", "", err);
                }
            }
        }
    }
}

pub fn main() -> Result<(), Error> {
    let args = Args::parse();
    let mut f = File::open(&args.path)?;

    let plugin = Plugin::parse(&mut f)?;

    for record in plugin.records {
        if let Ok(Record::Group(g)) = record {
            dump(&g, 0, &args);
        }
    }

    Ok(())
}
