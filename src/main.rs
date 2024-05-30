use clap::Parser;
use std::fs::File;
use std::io::prelude::*;
use std::fs;
use std::fs::metadata;
use sysinfo::{
    Components, Disks, Networks, System,
};
use console::style;

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
    } 

    if args.action == "rmfile" {
        let _ = del_file();
    }

    if args.action == "makedir" {
        let _ = make_dir();
    }

    if args.action == "rmdir" {
        let _ = del_dir();
    }

    if args.action == "infofile" {
        let _ = file_properties();
    }

    if args.action == "sysfetch" {
        let _ = fetch_device_info();
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

    let dir_metadata = fs::metadata(args.name)?;
    println!("File Size: {:?} KB ", dir_metadata.len());
    println!("File Type: {:?} ", dir_metadata.file_type());

    // Modify into more readable format
    if let Ok(time) = dir_metadata.created() {
        println!("Created At: {time:?}");
    }
    Ok(())

}

fn fetch_device_info() {
    let mut sys = System::new_all();

    sys.refresh_all();
    println!("{}", style("\nSystem Information\n").green());

    println!("{}: {} GB", style("total memory").green(), sys.total_memory()/u64::pow(10, 9));
    println!("{}: {} GB", style("used memory").green(), sys.used_memory()/u64::pow(10, 9));
    println!("{}: {} GB", style("total swap").green(), sys.total_swap()/u64::pow(10, 9));
    println!("{}: {} GB\n", style("used swap").green(), sys.used_swap()/u64::pow(10, 9));   

    println!("{}:             {:?}", style("System name").green(), System::name());
    println!("{}:   {:?}", style("System Kernel Version").green(), System::kernel_version());
    println!("{}:       {:?}", style("System OS Version").green(), System::os_version());
    println!("{}:        {:?}", style("System host name").green(), System::host_name());

    println!("NB CPUs: {}", sys.cpus().len());

}

// fn dir_search() {

// }
