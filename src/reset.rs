use clap::Args;
use system_shutdown::reboot_with_message;

type WinCMD = std::process::Command;

#[derive(Args, Debug)]
pub struct Reset {
    #[clap(short, long, value_parser)]
    ttr: u16, //*TTR means TimeTillRestart */
}

pub fn restart_pc(options: &Reset) {
    WinCMD::new("cmd")
        .args(&["/c", "ipconfig /release"])
        .spawn()
        .expect("cls command failed to start");
    WinCMD::new("cmd")
        .args(&["/c", "ipconfig /flushdns"])
        .spawn()
        .expect("cls command failed to start");
    WinCMD::new("cmd")
        .args(&["/c", "ipconfig /renew"])
        .spawn()
        .expect("cls command failed to start");
    WinCMD::new("cmd")
        .args(&["/c", "netsh ", "int", "ip", "reset"])
        .spawn()
        .expect("cls command failed to start");

    match reboot_with_message("Rebooting your device...", options.ttr.into(), true) {
        Ok(_) => println!("Shutting down, bye!"),
        Err(error) => eprintln!("Failed to shut down: {}", error),
    }
}
