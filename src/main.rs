// Main function

use std::{thread::sleep, time::{Duration, SystemTime, UNIX_EPOCH}};

fn main() {
    let start = SystemTime::now();
    let time_since_epoch= start.duration_since(UNIX_EPOCH).expect("Time went backwards");
    println!("{:?}", time_since_epoch);
    let start = SystemTime::now();
    let time_since_epoch= start.duration_since(UNIX_EPOCH).expect("Time went backwards");
    sleep(Duration::from_secs(1));
    println!("{:?}", time_since_epoch);
    let start = SystemTime::now();
    let time_since_epoch= start.duration_since(UNIX_EPOCH).expect("Time went backwards");
    println!("{:?}", time_since_epoch);
    let start = SystemTime::now();
    let time_since_epoch= start.duration_since(UNIX_EPOCH).expect("Time went backwards");
    println!("{:?}", time_since_epoch);
    let start = SystemTime::now();
    let time_since_epoch= start.duration_since(UNIX_EPOCH).expect("Time went backwards");
    println!("{:?}", time_since_epoch);
}