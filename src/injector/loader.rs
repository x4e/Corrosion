mod libinjector;

use sysinfo::{System, RefreshKind, SystemExt, ProcessExt};
use std::io::{stdout, Write, stdin};
use crate::libinjector::*;
use std::mem::zeroed;
use std::ffi::{CStr, CString};
use std::os::raw::c_void;

const CLIENT_NAME: &str = "target/debug/libcorrosion_client.so";

fn main() {
	let system = System::new_with_specifics(RefreshKind::new().with_processes());
	
	println!("Available processes:");
	for (pid, process) in system.get_processes() {
		if process.name() == "java" {
			println!("\t{}: {}", pid, process.name());
			println!("\t\t\t{:?}", process.cmd());
		}
	}
	
	print!("Please select a pid: ");
	let _ = stdout().flush();
	let input = get_input().parse::<i32>().expect("Please provide a valid 32bit integer");
	
	println!("Selected {}, injecting {}", input, CLIENT_NAME);
	
	let mut injector: *mut injector;
	let mut lib_handle: *mut c_void;
	unsafe {
		injector = zeroed();
		if injector_attach(&mut injector, input) != INJERR_SUCCESS {
			eprintln!("Attach error: try checking https://www.jetbrains.com/help/clion/attaching-to-local-process.html#prereq-ubuntu");
			panic!("Attach Error: {:?}", CStr::from_ptr(injector_error()));
		}
		
		lib_handle = zeroed();
		let lib_path = CString::new(CLIENT_NAME).unwrap();
		if injector_inject(injector, lib_path.as_ptr(), &mut lib_handle) != INJERR_SUCCESS {
			panic!("Inject Error: {:?}", CStr::from_ptr(injector_error()));
		}
	}
	
	println!("Done");
	/*println!("Library injected, press press enter to uninject...");
	get_input();
	
	unsafe {
		if injector_uninject(injector, lib_handle) != INJERR_SUCCESS {
			panic!("Uninject Error: {:?}", CStr::from_ptr(injector_error()));
		}
		
		injector_detach(injector);
	}
	
	println!("Uninjected");*/
}


fn get_input() -> String {
	let mut input = String::new();
	match stdin().read_line(&mut input) {
		Ok(_goes_into_input_above) => {},
		Err(_no_updates_is_fine) => {},
	}
	input.trim().to_string()
}
