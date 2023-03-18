use clap::Args;
use indicatif::{ProgressBar, ProgressStyle};
use std::thread;
use std::time::Duration;
use system_shutdown::reboot_with_message;

type WinCMD = std::process::Command;

#[derive(Args, Debug)]
pub struct Reset {
    #[clap(short, long, value_parser)]
    ttr: u16, //*TTR means TimeTillRestart */

    #[clap(short, long, value_parser)]
    message: String,
}

pub fn restart_pc(options: &Reset) {
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(120));
    pb.set_style(
        ProgressStyle::with_template("{spinner:.blue} {msg}")
            .unwrap()
            // For more spinners check out the cli-spinners project:
            // https://github.com/sindresorhus/cli-spinners/blob/master/spinners.json
            .tick_strings(&[
                "▹▹▹▹▹",
                "▸▹▹▹▹",
                "▹▸▹▹▹",
                "▹▹▸▹▹",
                "▹▹▹▸▹",
                "▹▹▹▹▸",
                "▪▪▪▪▪",
            ]),
    );

    pb.set_message("Resetting...");
    thread::sleep(Duration::from_secs(1));
    WinCMD::new("cmd")
        .args(&["/c", "ipconfig /release"])
        .status()
        .expect("cls command failed to start");
    WinCMD::new("cmd")
        .args(&["/c", "ipconfig /flushdns"])
        .status()
        .expect("cls command failed to start");
    WinCMD::new("cmd")
        .args(&["/c", "ipconfig /renew"])
        .status()
        .expect("cls command failed to start");
    WinCMD::new("cmd")
        .args(&["/c", "netsh int ip reset"])
        .status()
        .expect("cls command failed to start");
    pb.finish_with_message("Done");
    match reboot_with_message(options.message.as_str(), options.ttr.into(), true) {
        Ok(_) => println!("Shutting down, bye!"),
        Err(error) => eprintln!("Failed to shut down: {}", error),
    }
}
