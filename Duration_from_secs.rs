// GitHub Account
// https://github.com/softwareNuggets/Rust_shorts/
// Duration_from_secs.rs

use std::time::{Duration};
use std::thread;

fn main() {

	let duration = Duration::from_secs(3);
	thread::sleep(duration);

	dbg!(&duration);
	dbg!(&duration.as_micros());
	dbg!(&duration.as_millis());
	dbg!(&duration.as_nanos());
	dbg!(&duration.as_secs());
	dbg!(&duration.as_secs_f32());
	dbg!(&duration.as_secs_f64());
}
