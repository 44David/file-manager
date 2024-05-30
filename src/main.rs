use clap::Parser;
use std::fs::File;
use std::io::prelude::*;
// use std::io::Error;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]

struct Args {
    #[arg(short, long)]
    action: String,
    name: String,
}

fn main() {
    let args = Args::parse();

    if args.action == "make" {
       let _ = make_file();
    } 
}

fn make_file() -> std::io::Result<()> {
    let args = Args::parse();   

    let mut file = File::create(args.name)?;
    file.write_all(b"")?;
    Ok(())
}

fn del_file() -> std::io::Result<()> {

}

fn del_dir() -> std::io::Result<()> {

}

fn dir_properties() {

}
