use std::path::Path;
use std::fs;
use std::io::{Cursor, Read};
use directories::UserDirs;
use downloader::{Download, Downloader};
use winreg::{enums::*, RegKey};

fn main() {
    println!("User NodeJS Installer v0.1.1\nMIT License created by Tymon Woźniak\n");
    let user_dirs = UserDirs::new().unwrap();
    let install_dir = user_dirs.home_dir().join(".node");
    println!("NodeJS path: {:?}", install_dir);
    println!();

    let is_installed = install_dir.exists();
    if is_installed {
        println!("NodeJS is currently installed @ {:?}", install_dir);
        return;
    }

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    // create_subkey opens with write permissions
    let (env, _) = hkcu.create_subkey("Environment").unwrap();

    let downloaded_file = Path::new("node-v21.7.1-win-x64.zip");
    if !downloaded_file.exists() {
        println!("Downloading...");
        let mut downloader = Downloader::builder().build().expect("Failed to create downloader instance");
        let downloads = [Download::new("https://nodejs.org/dist/v23.1.0/node-v23.1.0-win-x64.zip")];

        downloader.download(&downloads).unwrap();
        println!("Successfully downloaded binary archive.");
    }

    println!("Looking at environment Path variable..");
    let env_path: String = env.get_value("Path").unwrap();
    let paths = env_path.split(";");
    for path in paths {
        if path == install_dir.to_str().unwrap() {
            println!("NodeJS is currently installed!");
            return;
        }
    }

    println!("Loading archive..");
    let mut file = fs::File::open(downloaded_file).unwrap();
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).unwrap();
    println!("Loaded {} bytes.", bytes.len());

    println!("Unpacking archive..");
    zip_extract::extract(Cursor::new(bytes), &install_dir, true).unwrap();

    println!("Looking at environment Path variable..");
    let paths = env_path.split(";");
    for path in paths {
        if path == install_dir.to_str().unwrap() {
            println!("NodeJS is currently installed!");
            return;
        }
    }

    println!("Setting environment variable..");
    let new_path = format!("{};{}", env_path, install_dir.to_str().unwrap());
    env.set_value("Path", &new_path).expect("Failed to set Path variable.");

    println!("Cleaning up..");
    fs::remove_file(downloaded_file).expect("Failed to cleanup");

    println!("\nSuccessfully installed NodeJS");
    println!("Please restart terminal to use \"node\" or \"npm\"")
}
