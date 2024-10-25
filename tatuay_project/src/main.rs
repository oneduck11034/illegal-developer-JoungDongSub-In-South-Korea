fn main() {
    if cfg!(target_os = "windows") {
        println!("Running on Windows!");
        // Add Windows-specific code here
        
        // https://github.com/zxcV32/unlimited-folders-on-windows-desktop/blob/master/virus.bat
    } else if cfg!(target_os = "linux") {
        println!("Running on Linux!");
        // Add Linux-specific code here
        
        // https://stackoverflow.com/questions/40613898/how-to-force-immediate-writes-to-a-disk-from-our-own-driver-on-linux
    } else {
        println!("Running on an unsupported OS!");
    }
}

