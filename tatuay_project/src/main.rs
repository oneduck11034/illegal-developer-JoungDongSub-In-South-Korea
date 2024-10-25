fn main() {
    if cfg!(target_os = "windows") {
        println!("Running on Windows!");
        // Add Windows-specific code here
    } else if cfg!(target_os = "linux") {
        println!("Running on Linux!");
        // Add Linux-specific code here
    } else {
        println!("Running on an unsupported OS!");
    }
}

