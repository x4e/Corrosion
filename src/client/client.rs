mod bitfield;
mod misctypes;
mod jvm;
mod jni;
mod jvmti;
mod jmm;

mod utils;

use crate::jni::{JNI_GetCreatedJavaVMs, jsize, JavaVM, JNIEnv, JavaVMAttachArgs, JNI_VERSION_1_8, jclass, jmethodID, jint, JNI_OK};
use ctor::{ctor, dtor};
use std::ptr::null_mut;
use std::os::raw::c_void;
use std::ffi::CString;

#[ctor]
fn init() {
	unsafe {
		println!("Injected");
		
		// Retrieve the number of JVMs present in the current process
		// This should always be one, but none the less we will check anyway
		let mut num_vms: jsize = 0;
		check_jni!(JNI_GetCreatedJavaVMs(null_mut(), 0, &mut num_vms));
		assert_eq!(num_vms, 1, "Unexpected number of VMs");
		println!("2");
		
		let mut jvms: [*mut JavaVM; 1] = [null_mut()];
		check_jni!(JNI_GetCreatedJavaVMs(jvms.as_mut_ptr(), num_vms, &mut num_vms));
		assert_eq!(num_vms, 1, "Unexpected number of VMs");
		println!("3");
		
		let jvm: *mut JavaVM = jvms[0];
		assert!(!jvm.is_null());
		
		let mut attach_args = JavaVMAttachArgs {
			version: JNI_VERSION_1_8 as i32,
			name: cstr!("main"),
			group: null_mut()
		};
		
		println!("Getting env");
		
		let mut void_ptr: *mut c_void = null_mut() as *mut c_void;
		let penv_ptr: *mut *mut c_void = &mut void_ptr as *mut *mut c_void;
		let env: *mut JNIEnv = *penv_ptr as *mut JNIEnv;
		
		
		
		check_jni!((**jvm).AttachCurrentThreadAsDaemon.unwrap()(jvm, null_mut(), &mut attach_args as *mut JavaVMAttachArgs as *mut c_void));
		println!("env: {:p}", env);
		check_jni!((**jvm).GetEnv.unwrap()(jvm, penv_ptr, JNI_VERSION_1_8 as i32));
		println!("env2: {:p}", env);
		
		println!("Attached to thread");
		
		let system: jclass = (**env).FindClass.unwrap()(env, CString::new("java/lang/System").unwrap().into_raw());
		assert!(!system.is_null());
		
		let meth: jmethodID = (**env).GetStaticMethodID.unwrap()(env, system, cstr!("exit"), cstr!("(I)V"));
		(**env).CallStaticVoidMethod.unwrap()(env, system, meth, 0 as jint);
		
		println!("Exited");
		
		(**jvm).DetachCurrentThread.unwrap()(jvm);
	}
}

