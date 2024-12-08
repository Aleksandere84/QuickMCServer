use std::process::{exit, Command};

pub fn check_java() -> bool {
    let output = Command::new("java")
        .arg("-version")
        .output();

    match output {
        Ok(output) => output.status.success(),
        Err(_) => false,
    }
}

pub fn install_java() {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .arg("/c")
            .arg("winget")
            .arg("install")
            .arg("EclipseAdoptium.Temurin.21.JDK")
            .spawn()
            .unwrap();
    } else if cfg!(target_os = "linux") {
        eprintln!("Install Java!");
    } else {
        eprintln!("Unsupported OS.");
        exit(1);
    }
}