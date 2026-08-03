#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

use std::env;
use std::io::Write;
use std::net::{ TcpStream};
use std::process::Command;
use std::time::Duration;

const PORT: u16 = 39643;

const PLAYSTAR_BIN: &str = if cfg!(target_os = "windows") {
    "playstar.exe"
} else {
    "PlayStar"
};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    try_connect(&args);
}

// Tries to connect to existing playstar session or spawns a new one
fn try_connect(args: &[String]){
    match TcpStream::connect(("127.0.0.1", PORT)){
        Ok(_) => {
            forward_args(args);
        },
        Err(_) => {
            println!("[playstar-runner] Error connecting to stream");
            launch_playstar(args);
        },
    }
}

// Launches playstar with given args
fn launch_playstar(args: &[String]){
    let exe_dir = env::current_exe()
        .expect("Could not get executable directory")
        .parent()
        .expect("No parent directory")
        .to_path_buf();

    let playstar_path = exe_dir.join(PLAYSTAR_BIN);

    let mut cmd = Command::new(&playstar_path);
    cmd.args(args);
    cmd.spawn().expect("Fail starting PlayStar");
}

// Forwards args to current playstar session
fn forward_args(args: &[String]){
    match TcpStream::connect(("127.0.0.1", PORT)){
        Ok(mut stream) => {
            let payload = args.join("\n");
            stream.set_write_timeout(Some(Duration::from_secs(2))).ok();
            stream.write_all(payload.as_bytes()).ok();
        }
        Err(e) => eprintln!("[playstar-runner]: Connection error while connecting to current instance; Error: {e}"),
    }
}