extern crate libc;
extern crate petsc_sys;
extern crate slepc_sys;

use libc::{c_char, c_int};
use std::ffi::CString;
use petsc_sys::{PetscPrintf, PETSC_COMM_WORLD};
use slepc_sys::{SlepcInitialize, SlepcFinalize};

fn main() {
    let argv = std::env::args().collect::<Vec<String>>();
    let argc = argv.len();

    let mut c_argc = argc as c_int;
    let mut c_argv = argv.into_iter().map(|arg| CString::new(arg).unwrap().into_raw()).collect::<Vec<*mut c_char>>();
    let mut c_argv_ptr = c_argv.as_mut_ptr();

    unsafe {
        let ierr = SlepcInitialize(&mut c_argc, &mut c_argv_ptr, std::ptr::null(), std::ptr::null());
        if ierr != 0 {
            println!("error code {} from SlepcInitialize", ierr);
        }

        let msg = CString::new("Hello from SLEPc and PETSc\n").unwrap();

        let ierr = PetscPrintf(PETSC_COMM_WORLD, msg.as_ptr());
        if ierr != 0 {
            println!("error code {} from PetscPrintf", ierr);
        }

        let ierr = SlepcFinalize();
        if ierr != 0 {
            println!("error code {} from SlepcFinalize", ierr);
        }
    };
}
