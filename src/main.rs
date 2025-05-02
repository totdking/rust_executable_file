use std::error::Error;
use std::fs:: metadata;
use std::path::Path;
use std::process::Command;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    //This is for debug mode
    // let file_path = "/home/konquest/home/konquest/rust/Rust-for-practice/floating_point/target/debug/floating_point";

    //This is for release mode
    let file_path = "/home/konquest/home/konquest/rust/Rust-for-practice/floating_point/target/release/floating_point";
    check_permission(file_path)?;

    let duration = Duration::from_secs(3);
    thread::sleep(duration);

    run_command(&file_path)?;
    Ok(())
    

}

fn check_permission(filepath: &str) -> std::io::Result<()> {
    let path = Path::new(filepath);
    match metadata(&path) {
        Ok(metadata) => {
            let permission = metadata.permissions();
            let readonly = permission.readonly() == false;
            let writable = !permission.readonly();

            println!("File: {}", filepath);
            println!("Readable: {}", readonly);
            println!("Writable: {}", writable);
            Ok(())
        }
        Err(e) => {
            eprintln!(
                "could not get metadata for file path {:?} {:?}",
                filepath, e
            );
            panic!("FILE NOT EXECUTABLE THEREFORE EXECUTABLE FUNCTION PANICS HERE");
            // Err(e)
        }
    }
}

fn run_command(filepath: &str) -> Result<(), Box<dyn Error>> {
    let mut command = Command::new(&filepath);
    let status = command.status()?;
    if status.success() {
        println!("the file has been successfully executed at {:?}", filepath);
        Ok(())
    } else {
        eprintln!("failed to execute {:?}", filepath);
        eprintln!("status code: {:?}", status.code());
        Err("Execution failed".into())
    }
}
