// sched.rs
// Simple process scheduler
// Stephen Marz
// 27 Dec 2019

use crate::process::{ProcessState, PROCESS_LIST};
use crate::cpu::get_mtime;

pub fn schedule() -> usize {
	let mut frame_addr: usize = 0x1111;
	unsafe {
		if let Some(mut pl) = PROCESS_LIST.take() {
			// Rust allows us to label loops so that break statements can be
			// targeted.
			'procfindloop: loop {
				pl.rotate_left(1);
				if let Some(prc) = pl.front_mut() {
					match prc.get_state() {
						ProcessState::Running => {
							frame_addr =
								prc.get_frame_address();
							break 'procfindloop;
						},
						ProcessState::Sleeping => {
							// Awaken sleeping processes whose sleep until is in
							// the past.
							if prc.get_sleep_until() <= get_mtime() {
								prc.set_state(ProcessState::Running);
								frame_addr = prc.get_frame_address();
								break 'procfindloop;
							}
						},
						_ => {},
					}
				}
			}
			PROCESS_LIST.replace(pl);
		}
		else {
			println!("could not take process list");
		}
	}
	frame_addr
}
