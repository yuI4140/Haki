use std::env; 
use std::process::{Command, Stdio};
use std::str;
fn check_programs() -> bool {
    let pacman_output = Command::new("pacman").arg("-Qe").output();
    if let Ok(s) = pacman_output {
        if !s.status.success() {
            return false;
        }
        let grep_result = Command::new("grep")
            .arg("-E")
            .arg("(slurp)|(grim)")
            .stdin(Stdio::piped())
            .output();
        return grep_result
            .expect("Err: making grep command")
            .status
            .success();
    }
    false
}
fn get_res_and_uuid() -> (Vec<u8>, Vec<u8>) {
    let bres = Command::new("slurp")
        .output()
        .expect("failed to execute process")
        .stdout;
    let buuid = Command::new("uuidgen")
        .output()
        .expect("failed to execute process")
        .stdout;
    (bres, buuid)
}
fn main() {
    let args:Vec<String>=env::args().collect(); 
    let mut xhome:String=String::new();
    if let Ok(home) = env::var("HOME") {
       xhome=home; 
    }
    if args.len()==2 && args[1]=="check" {
    if !check_programs() {
        eprintln!("Warning: programs don't exists!");
        eprintln!("INFO: Proceeding to install them");
        let installation = Command::new("sudo")
            .arg("pacman")
            .arg("-S")
            .arg("grim")
            .arg("slurp")
            .arg("--noconfirm")
            .output();
        if let Ok(s) = installation {
            if !s.status.success() {
                eprintln!(
                    "Err: cannot install require pakages:{}",
                    str::from_utf8(&s.stderr)
                        .expect("Err: cannot convert Vec<u8> -> &str")
                        .replace("error:", "")
                );
                std::process::exit(-1);
            }
            eprintln!("INFO: installed require pakages");
        } else {
            eprintln!("Err: cannot install require pakages");
            std::process::exit(-1);
        }
    }
    }
    let (bres, buuid) = get_res_and_uuid();
    let res = str::from_utf8(&bres)
        .expect("Err: conv Vec<u8> -> &str")
        .trim();
    let uuid = str::from_utf8(&buuid)
        .expect("Err: conv Vec<u8> -> &str")
        .trim();
    _ = Command::new("mkdir")
        .arg("-p")
        .arg(&format!("{}/Pictures",xhome))
        .output()
        .expect("Err: cannot make dir");
    let output_image: String = format!("~/Pictures/screenshot_{}.png", uuid);
    let output_image=output_image.replace("~",xhome.as_str());
    let ximage = Command::new("grim")
        .arg("-g")
        .arg(res)
        .arg(output_image)
        .output();
    println!("Image Status:");
    if let Ok(s) = ximage {
        if !s.status.success() {
            println!( "Err:{}",
              str::from_utf8(&s.stderr)
              .expect("Err: cannot convert Vec<u8> -> &str")
            );
        }else {
            println!("Ok");
        }
    }
}
