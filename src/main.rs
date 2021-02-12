use std::process::Command;

fn main() {
    let mut good_ping = vec![];
    for i in 1..=254 {
        let address = format!("192.168.0.{}", i);
        let command = format!("ping -c1 {} -W1", address);
        let output = Command::new("sh")
            .arg("-c")
            .arg(command)
            .output().unwrap_or_else(|e| {
            panic!("failed to execute process: {}", e)
        });

        if output.status.success() {
            println!("Good Ping with address: {}", address);
            good_ping.push(address);

        } else {
            println!("Bad Ping with address {}", address);
        }
    }

    println!("Good pings: {:?}", &good_ping)
}