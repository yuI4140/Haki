use std::{process::{Command, exit}};
fn find_protocol(protocol: &str) -> String {
    let output = Command::new("bash")
    .arg("-c")
    .arg(format!(r#"realpath $(find . -type f -name "{protocol}.xml")"#))
    .output()
    .expect("failed to run");
    String::from_utf8(output.stdout).unwrap().trim().to_string()
}
fn gen_protocol(out_dir: &str,protocol: &str) -> bool {
    let protocol_xml:&str=&find_protocol(protocol);
    let protocol_h=Command::new("wayland-scanner")
    .args(&["client-header", protocol_xml])
    .arg(format!("{out_dir}/{protocol}.h"))
    .output();
    let protocol_c =Command::new("wayland-scanner")
    .args(&["private-code", protocol_xml])
    .arg(format!("{out_dir}/{protocol}.c"))
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
fn main() {
    let out_dir="src/backends/wayland/";
    if !gen_protocol(out_dir,"xdg-shell") {
        exit(1);
    }
    cc::Build::new()
        .file("src/backends/wayland/wayland-native.c")
         .flag("-O2")
        .flag("-Wall")
        .flag("-Wextra")
        .flag("-Werror")
        .flag("-lwayland-client")
        .flag("-lm")
        .compile("wayland-native");
     println!("cargo:rerun-if-changed=src/backends/wayland/");
}
