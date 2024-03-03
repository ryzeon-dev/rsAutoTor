#![allow(non_snake_case, unused_must_use)]

use curl::easy::Easy;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;
use std::io::{stdout, Write};


const ASCII_ART: &str = "
\x1b[91m██████╗ ███████╗\x1b[96m █████╗ ██╗   ██╗████████╗ ██████╗ ████████╗ ██████╗ ██████╗ \x1b[00m
\x1b[91m██╔══██╗██╔════╝\x1b[96m██╔══██╗██║   ██║╚══██╔══╝██╔═══██╗╚══██╔══╝██╔═══██╗██╔══██╗\x1b[00m
\x1b[91m██████╔╝███████╗\x1b[96m███████║██║   ██║   ██║   ██║   ██║   ██║   ██║   ██║██████╔╝\x1b[00m
\x1b[91m██╔══██╗╚════██║\x1b[96m██╔══██║██║   ██║   ██║   ██║   ██║   ██║   ██║   ██║██╔══██╗\x1b[00m
\x1b[91m██║  ██║███████║\x1b[96m██║  ██║╚██████╔╝   ██║   ╚██████╔╝   ██║   ╚██████╔╝██║  ██║\x1b[00m
\x1b[91m╚═╝  ╚═╝╚══════╝\x1b[96m╚═╝  ╚═╝ ╚═════╝    ╚═╝    ╚═════╝    ╚═╝    ╚═════╝ ╚═╝  ╚═╝\x1b[00m
";

const AUTHOR_TAG: &str = "Code by \x1b[96mryzeon-dev\x1b[00m";
const GITHUB_LINK: &str = "\x1b[96mhttps://github.com/ryzeon-dev/\x1b[00m";

fn checkIp() -> String {
    let mut buffer = Vec::new();
    let mut handler = Easy::new();

    handler.proxy("socks5://127.0.0.1:9050");
    match handler.url("http://checkip.amazonaws.com/") {

        Err(why) => println!("Error: {}", why),
        Ok(_) => {
            let mut transfer = handler.transfer();
            
            transfer.write_function(|data| {
                buffer.extend_from_slice(data);
                Ok(data.len())
            }).unwrap();

            transfer.perform().unwrap();
        }
    }
    
    return String::from_utf8(buffer).unwrap().trim().to_string();
}

fn restartTor() {
    Command::new("/bin/sh").arg("-c").arg("sudo systemctl restart tor").output();
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut interval: u64 = 2000;

    if args.contains(&"--help".to_string()) || args.contains(&"-h".to_string()) {
        println!("rsAutoTor: TOR network automatic ip changer written in Rust");
        println!("usage: rsAutoTor [OPTIONS]");
        println!("\nOptions:");
        println!("    -i INTERVAL     Set wait interval before changing IP (default is 2 seconds)");
        println!("    -h | --help     Show this message and exit");
        std::process::exit(0);
    }

    if args.contains(&"-i".to_string()) {
        let index = args.iter().position(|arg| arg == "-i").unwrap();

        match args[index + 1].parse::<u64>() {
            Err(_) => {
                println!("Invalid argument");
                std::process::exit(1);
            },
            
            Ok(value) => {
                interval = value * 1000_u64;
            }
        }
    }

    println!("{}", ASCII_ART);
    println!("{} [{}]\n", AUTHOR_TAG, GITHUB_LINK);
    let mut stdout = stdout();

    loop {
        let ip = checkIp();
        
        stdout.write(format!("\rCurrent IP: \x1b[96m{ip:<width$}\x1b[00m", ip=ip, width=15).as_bytes());
        stdout.flush();
        
        restartTor();
        sleep(Duration::from_millis(interval));
    }
}