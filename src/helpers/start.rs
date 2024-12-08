use std::fs::File;
use std::io::Write;

pub fn start_server() {
    if cfg!(target_os = "windows") {
        let mut file = File::create("start.bat").unwrap();
        let content = "java -jar server.jar";
        file.write_all((&content).as_ref()).unwrap();
        println!("To start the server, just open start.bat");
    } else if cfg!(target_os = "linux") {
        let mut file = File::create("start.sh").unwrap();
        let content = "java -jar server.jar";
        file.write_all((&content).as_ref()).unwrap();
        println!("To start the server, just open start.sh");
    }
}