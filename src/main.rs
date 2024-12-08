mod helpers;

use std::process::exit;
use crate::helpers::java::{check_java, install_java};
use crate::helpers::papermc::{download_paper, eula, eula_write};
use crate::helpers::start::start_server;

fn java_not_install() {
    eprintln!("Java is not installed!");
    install_java();
}

fn main() {
    match check_java(){
        true => println!("Java is Installed!"),
        false => java_not_install(),
    }

    match download_paper() {
        Ok(_) => {
            println!("Downloading papermc completed!");
        }
        Err(e) => {
            eprintln!("Error! {}", e);
            exit(1);
        }
    }

    match eula() {
        true => eula_write(true),
        false => eula_write(false),
    }

    start_server();
}
