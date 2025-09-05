use std::process::Command;

fn main() {
    // Get the rustc version
    let output = Command::new("rustc")
        .arg("--version")
        .output()
        .expect("Failed to execute rustc --version");

    let version = String::from_utf8(output.stdout)
        .expect("Failed to parse rustc version output")
        .trim()
        .to_string();

    // making it available as an environment variable at compile time
    println!("cargo:rustc-env=RUSTC_VERSION={}", version);
}
