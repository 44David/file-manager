// removes rust compiler warnings
#![allow(unused)]


use clap::Parser;
use std::fs::File;
use std::io::prelude::*;
use std::fs;
use std::fs::metadata;
use sysinfo::{
    Components, Disks, Networks, System,
};
use console::style;
use std::env;


use chrono::offset::Local;
use chrono::DateTime;
use std::time::SystemTime;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]

struct Args {
    #[arg(short, long)]
    action: String,
    name: String,
}

fn main() {
    let args = Args::parse();


    if args.action == "makefile" {
       let _ = make_file();
       println!("{}", style("File created successfully").cyan())
    } 

    if args.action == "rmfile" {
        let _ = del_file();
        println!("{}", style("File removed successfully").red())
    }

    if args.action == "makedir" {
        let _ = make_dir();
        println!("{}", style("Folder created successfully").cyan())
    }

    if args.action == "rmdir" {
        let _ = del_dir();
        println!("{}", style("Folder removed successfully").cyan())
    }

    if args.action == "infofile" {
        let _ = file_properties();
    }

    if args.action == "sysfetch" {
        let _ = fetch_device_info();
    }

    if args.action == "search" {
        let _ = dir_search();
    } 
}

fn make_file() -> std::io::Result<()> {
    let args = Args::parse();   

    let mut file = File::create(args.name)?;
    file.write_all(b"")?;
    Ok(())
}

fn del_file() -> std::io::Result<()> {  
    let args = Args::parse();

    fs::remove_file(args.name)?;
    Ok(())
}

fn make_dir() -> std::io::Result<()> {
    let args = Args::parse();

    fs::create_dir(args.name)?;
    Ok(())
}

fn del_dir() -> std::io::Result<()> {
    let args = Args::parse();

    fs::remove_dir(args.name);
    Ok(())
}

fn file_properties() -> std::io::Result<()> {
    let args = Args::parse();

    let file_metadata = fs::metadata(args.name)?;
    println!("{}", style("\nAbout this file").cyan());

    println!("File Size: {:?} KB ", file_metadata.len());

    if let Ok(time) = file_metadata.created() {
        let datetime: DateTime::<Local> = time.into();
        println!("Created At: {}", datetime.format("%d/%m/%Y %T"));

    };

    if let Ok(mod_time) = file_metadata.modified() {
        let last_modified: DateTime::<Local> = mod_time.into();
        println!("Modified At: {}\n", last_modified.format("%d/%m/%Y %T"));
    };
    Ok(())

}

fn dir_search() -> std::io::Result<()> {
    let args = Args::parse();
    let path = env::current_dir()?;
    let file_metadata = fs::metadata(&args.name)?;

    if file_metadata.is_file() {
        println!("{} found in {}", args.name, path.display() );
    } else {
        println!("{} was not found", args.name);
    };

    Ok(())
}



fn fetch_device_info() {
    let mut sys = System::new_all();

    sys.refresh_all();

    println!("{}", style("\nSystem Information\n").cyan());

    println!("{}: {} GB", style("total memory").cyan(), sys.total_memory()/u64::pow(10, 9));
    println!("{}: {} GB", style("used memory").cyan(), sys.used_memory()/u64::pow(10, 9));
    println!("{}: {} GB", style("total swap").cyan(), sys.total_swap()/u64::pow(10, 9));
    println!("{}: {} GB\n", style("used swap").cyan(), sys.used_swap()/u64::pow(10, 9));   

    println!("{}:             {:?}", style("System name").cyan(), System::name().unwrap());
    println!("{}:   {:?}", style("System Kernel Version").cyan(), System::kernel_version().unwrap());
    println!("{}:       {:?}", style("System OS Version").cyan(), System::os_version().unwrap());
    println!("{}:        {:?}", style("System host name").cyan(), System::host_name().unwrap());

    println!("{}:          {:?}", style("Number of CPU's").cyan(), sys.cpus().len());
}


