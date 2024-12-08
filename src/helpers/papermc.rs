use reqwest::blocking::get;
use std::fs::File;
use std::io::Write;

pub fn download_paper() -> Result<(), Box<dyn std::error::Error>> {
    println!("Downloading paper... Version - 1.21.3");
    let url = "https://api.papermc.io/v2/projects/paper/versions/1.21.3/builds/81/downloads/paper-1.21.3-81.jar";
    let response = get(url)?;
    if response.status().is_success() {
        let mut file = File::create("papermc.jar")?;
        let content = response.bytes()?;
        file.write_all(&content)?;

        println!("PaperMC downloaded successfully!");

        Ok(())
    } else {
        Err(format!("Error corrupted: {}", response.status()).into())
    }
}

pub fn eula() -> bool {
    println!("Eula - https://www.minecraft.net/en-en/eula");
    print!("Do you accept eula? (y/n) ");
    let mut input = String::new();
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut input).unwrap();
    match input.to_lowercase().trim() {
        "y" => true,
        "n" => false,
        _ => false,
    }
}

pub fn eula_write(option: bool) {
    match option {
        true => {
            let mut file = File::create("eula.txt").unwrap();
            let content = "eula=true";
            file.write_all((&content).as_ref()).unwrap();
            println!("Eula written!");
        }
        false => {
            println!("You didn't accept eula!");
        }
    }
}