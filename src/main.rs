use std::process::{Command, Stdio};
use log::{error, info, warn, SetLoggerError};
use speedate::DateTime;

mod logger;
use logger::{Logger, Output};

fn main()  -> Result<(), SetLoggerError> {
    Logger::set(Output::File("/home/piotr/logs/tzcheck.log"), log::Level::Trace)?;
    //Logger::set(Output::STDOUT, log::Level::Trace)?;

    let output = Command::new("docker").arg("logs").arg("-n1").arg("-t").arg("octez-node")
        // Tell the OS to record the command's output
        .stdout(Stdio::piped())
        // execute the command, wait for it to complete, then capture the output
        .output()
        // Blow up if the OS was unable to start the program
        .unwrap();

    let status_code = output.status.code().unwrap();
    if status_code != 0 {
        error!("docker logs status code = {}", status_code);
        return Ok(());
    }
    //dbg!(&output);
    // extract the raw bytes that we captured and interpret them as a string
    let mut stdout = String::from_utf8(output.stderr).unwrap();

    // 2024-03-06T00:57:36.145446126Z Mar 06 01:57:36.145: operation op2A7QUS2B2jT99nkhrv3XWtQLiHBa5nwjoasV4sXazEiyhYf41 injected
    stdout.insert_str(19, "Z"); // reduce the number of miliseconds for parsing
    //println!("{}", &stdout[..20]);
    let mut log_dt = DateTime::parse_str(&stdout[..20]).unwrap();
    log_dt = log_dt.in_timezone(3600).unwrap(); // convert from docker timestamp log UTC time

    let now = DateTime::now(3600).unwrap();
    let now_ts = now.timestamp();
    //println!("Current timestamp is {}", now_ts);
    let log_ts = log_dt.timestamp();
    //println!("Log timestamp is {}", log_ts);
    //println!("Diff in seconds: {}", now_ts-log_ts);
    let diff_in_seconds = now_ts-log_ts;
    if diff_in_seconds < 60 {
        info!("Difference {} seconds", diff_in_seconds);
        return Ok(());
    }
    // restart tezos docker container bc its stuck
    let _output = Command::new("docker-compose").arg("restart")
        .current_dir("/home/piotr/tezos")
        .stdout(Stdio::piped())
        .output()
        .unwrap();
    warn!("Difference {} seconds: tezos container restarted", diff_in_seconds);
    Ok(())
}
