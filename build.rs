use std::ffi::OsStr;
use std::fs::{self, DirEntry, read_dir, remove_dir, remove_file};
use std::io::{Read, Write};
use std::os::unix::fs::MetadataExt;
use std::os::unix::process::CommandExt;
use std::process::{Command, exit}; use std::path::{self, Path};
fn check_program(program:&str) -> bool {
   Command::new("which")
       .arg(program)
       .status().unwrap().success() } fn find_protocol(protocol: &str) -> String { let output = Command::new("bash")
    .arg("-c")
    .arg(format!(r#"realpath $(find . -type f -name "{protocol}.xml")"#)) .output()
    .expect("failed to run");
    String::from_utf8(output.stdout).unwrap().trim().to_string()
}
fn gen_protocol(out_dir: &str,protocol: &str,outpaths:&mut Option<(String,String)>) -> bool {
    let protocol_name=protocol.replace("-","_");
    let outpath_h=format!("{out_dir}{protocol_name}.h"); let outpath_c=format!("{out_dir}{protocol_name}.c");
    *outpaths=Some((outpath_c.clone(),outpath_h.clone()));
    let protocol_xml:&str=&find_protocol(protocol);
    let protocol_h=Command::new("wayland-scanner")
    .args(&["client-header", protocol_xml])
    .arg(outpath_h)
    .output();
    let protocol_c =Command::new("wayland-scanner")
    .args(&["private-code", protocol_xml])
    .arg(outpath_c)
    .output();
    if let (Ok(c),Ok(h)) = (protocol_c,protocol_h) {
        if c.status.success() && h.status.success() {
            println!("{protocol}: compiled to {out_dir}");
            return true;
        }else {
            println!("cannot compile {protocol}");
        }
    }
    false
}
fn compile_c(compiler:&str,c_file:&str,obj_file:&str) -> bool {
    let status = Command::new(compiler)
        .arg("-c")
        .arg("-fPIC")
        .arg("-o")
        .arg(obj_file)
        .arg(c_file)
        .status()
        .expect("Failed to spawn C compiler");
    if !status.success() {
        eprintln!("C compilation failed for {}", c_file);
        return false;
    }
    true
}
fn link_c(linker:&str,files:&[&str],outfile:&str) -> bool {
    let status = Command::new(linker)
        .args(["rcs", outfile])
        .args(files)
        .status()
        .expect("Failed to spawn ar");
    if !status.success() {
        eprintln!("ar failed for {}", outfile);
        return false;
    }else {
        println!("compiled at {}", outfile);
    }
    true
}
fn execute(cmd:&str) -> bool {
    let commands:Vec<&str>=cmd.split("\n").collect();
    let program=commands[0];
    _=commands.iter().skip(1);
    Command::new(program)
        .args(commands.as_slice())
        .status()
        .expect(&format!("Error: cannot execute {cmd}"))
        .success()
}
fn execute_bash(cmd:&str) -> bool {
    execute(&format!("bash -c \"{cmd}\""))
}
fn convert_osstr_to_str(s:&OsStr) -> &str {
    s.to_str().unwrap()
}
fn build_static_lib(
    tmp_dir:&str,
    c_file: &str,
    output_lib: &str) -> bool {
     //let c_file   = &format!("./{tmp_dir}{c_file}");
    let output_lib = &format!("./{tmp_dir}lib{output_lib}.a");
    let stem = Path::new(c_file)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("out");
    let obj_c_path = format!("{tmp_dir}{}.o",stem);
    println!("cfile:{c_file}");
    let compiler="clang";
    if !compile_c(compiler,c_file, &obj_c_path) {
       return false; 
    }
    let files=[ obj_c_path.as_str() ];
    if !link_c("ar",&files,output_lib) {
        return false;
    }
    true
}
fn linker_options(out_dir: &str) {
    println!("cargo:rustc-link-search=native={out_dir}");
   println!("cargo:rustc-link-arg=-lwayland_native");
    // Tell the linker not to discard libs based on order / as-needed.
    println!("cargo:rustc-link-arg=-Wl,--no-as-needed");
    // Group: let lld re-scan until all symbols resolve.
   // println!("cargo:rustc-link-arg=-Wl,--start-group");
   // println!("cargo:rustc-link-arg=-l:libxdgshell_amalgamation.a");
   // println!("cargo:rustc-link-arg=-lwayland-client");
   // println!("cargo:rustc-link-arg=-lm");
   // println!("cargo:rustc-link-arg=-Wl,--end-group");
}
fn create_amalgamation(
    name:&str,
    out_dir: &str,
    file:(&str,&str)) -> bool {
    let c=file.0; let h=file.1;
    let amalgamation=&format!("{out_dir}{name}_amalgamation.c");
    let mut cf=fs::File::open(&c).expect(&format!("Cannot open {c}"));
    let mut hf=fs::File::open(&h).expect(&format!("Cannot open {h}"));
    let hf_meta=hf.metadata().unwrap();
    let cf_meta=cf.metadata().unwrap();
    let amalgamation_size=hf_meta.size()+cf_meta.size();
    let mut buffer=String::new();
    _=hf.read_to_string(&mut buffer);
    _=cf.read_to_string(&mut buffer);
    println!("is amalgamation? {}",
        amalgamation_size==buffer.len() as u64);
    let mut amalgamation=fs::File::create(amalgamation)
        .expect(&format!("Error creating {amalgamation}"));
    _=amalgamation.write_all(buffer.as_bytes());
    return amalgamation_size==buffer.len() as u64;
}
fn main() {
    let out_dir="src/backends/wayland/";
    let mut outpaths_xdgshell:Option<(String,String)>=None;
    if !gen_protocol(out_dir,"xdg-shell",&mut outpaths_xdgshell) {
        exit(1);
    }
    if let Some((c,h)) = outpaths_xdgshell {
        println!("outpaths.c:{c}");
        if !create_amalgamation("xdg_shell",out_dir,(&c,&h)) {
           exit(1); 
        }
        let c=format!("{out_dir}xdg_shell_amalgamation.c");
        if !build_static_lib(out_dir,&c,"xdg_shell_amalgamation") {
            exit(1);
        }
        if !check_program("c3c") {
           eprintln!("Erro: C3 compiler not add to $PATH or installed"); 
           exit(1);
        }
        let libfile = format!("{out_dir}wayland_native.c3");
        let outlib  = format!("{out_dir}libwayland_native");
        let c3_status = Command::new("c3c")
            .arg("dynamic-lib")
            .arg(&libfile)
            .arg("-o")
            .arg(&outlib)
            .args(["-L",out_dir])
            .args(["-l","wayland-client"])
            .args(["-l","xdg_shell_amalgamation"])
            .status().expect("Error: cannot compile ");
        if !c3_status.success() {
            eprintln!("Error: cannot compile {outlib}");
            exit(1);
        }
        if let Ok(entry) = read_dir(out_dir)  {
            for e in entry {
                if let Ok(e) = e {
                    let epath   = e.path();
                    let path_str= epath.clone().into_string().unwrap();
                    let spath=path_str.split_once(".");
                    if let Some(spath) = spath  {
                        if epath.is_file() && spath.1!="c3" {
                           _=remove_file(&epath);
                           println!("removed {}",spath.0);
                        }
                        if path_str=="headers" {
                           _=remove_dir(&epath);
                           println!("removed {}",spath.0);
                        }
                    }
                }else {
                    continue;
                }
            }
        } 
        exit(1);
        linker_options(out_dir);
     }
}
