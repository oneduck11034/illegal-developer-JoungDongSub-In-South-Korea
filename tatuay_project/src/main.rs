use std::process::Command;
 

fn main() {
    if cfg!(target_os = "windows") {
        println!("Running on Windows!");
        // Add Windows-specific code here
        // https://github.com/zxcV32/unlimited-folders-on-windows-desktop/blob/master/virus.bat
        
        // need to echo Hello From echotest.bat >> ./init.bat
        // FTP or SSH file transfer
        let _ = Command::new("cmd")
            .args(&["/C", "./init.bat"])
            .output()
            .expect("failed to execute process");

    } else if cfg!(target_os = "linux") {
        println!("Running on Linux!");
        // Add Linux-specific code here
        // https://stackoverflow.com/questions/40613898/how-to-force-immediate-writes-to-a-disk-from-our-own-driver-on-linux


    } else {
        println!("Running on an unsupported OS!");
    }
}

