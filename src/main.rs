use std::process::{Command, Stdio};
use chrono::prelude::*;
use log::{error, info, warn, SetLoggerError};

mod logger;
use logger::{Logger, Output};

fn main()  -> Result<(), SetLoggerError> {
    Logger::set(Output::File("/home/piotr/logs/tzcheck.log"), log::Level::Trace)?;

    let output = Command::new("docker").arg("logs").arg("-n1").arg("octez-node")
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

    //assert_eq!("January".parse::<Month>(), Ok(Month::January));
    //println!("{}", stdout);
    let now = Local::now();
    let now_str = now.to_rfc3339(); //1996-12-19T16:39:57-08:00
    let now_year = &now_str[..4];
    // add current year to the docker log output
    stdout.insert_str(0, now_year);
    // append timezone
    stdout.insert_str(19, " +0100");
    //println!("{}", stdout);
    //println!("{}", &stdout[..25]);
    let log_dt = DateTime::parse_from_str(&stdout[..25], "%Y%b %d %H:%M:%S %z").unwrap();

    let now_ts: i64 = now.timestamp();

    //println!("Current timestamp is {}", now_ts);
    let log_ts: i64 = log_dt.timestamp();
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
