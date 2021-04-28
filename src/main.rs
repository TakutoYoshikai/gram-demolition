use ofiles::opath;
use std::thread;
extern crate nix;

use nix::unistd::Pid;
use nix::sys::signal::{self, Signal};
use std::time;

fn run(file_path: &str, interval_ms: u64) {
    loop {
        let pids = opath(file_path).unwrap();
        for pid in pids {
            signal::kill(Pid::from_raw(u32::from(pid) as i32), Signal::SIGTERM).unwrap();
        }
        let interval = time::Duration::from_millis(interval_ms);
        thread::sleep(interval);
    }
}
fn main() {
}
