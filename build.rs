extern crate build_probe_mpi;
extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    // Allow user to set PETSc paths from environment variables.
    let petsc_include_dir: PathBuf = [env::var("PETSC_DIR").unwrap(),
        String::from("include")].iter().collect();

    let petsc_arch_include_dir: PathBuf = [env::var("PETSC_DIR").unwrap(),
        env::var("PETSC_ARCH").unwrap(), String::from("include")].iter().collect();

    let petsc_lib_dir = PathBuf::from(env::var("PETSC_LIB").unwrap());

    // Allow user to set SLEPc paths from environment variables.
    let slepc_include_dir: PathBuf = [env::var("SLEPC_DIR").unwrap(),
        String::from("include")].iter().collect();

    let slepc_arch_include_dir: PathBuf = [env::var("SLEPC_DIR").unwrap(),
        env::var("PETSC_ARCH").unwrap(), String::from("include")].iter().collect();

    let slepc_lib_dir = PathBuf::from(env::var("SLEPC_LIB").unwrap());

    // Tell Cargo to link the PETSc, SLEPc, LAPACK, and BLAS libraries.
    println!("cargo:rustc-link-search={}", slepc_lib_dir.display());
    println!("cargo:rustc-link-lib=petsc");
    println!("cargo:rustc-link-lib=slepc");
    println!("cargo:rustc-link-lib=flapack");
    println!("cargo:rustc-link-lib=fblas");
    println!("cargo:rustc-link-lib=gfortran");

    // Find the system MPI library and headers,
    // in the same way as rsmpi/build.rs.
    let mpi_lib = match build_probe_mpi::probe() {
        Ok(mpi_lib) => mpi_lib,
        Err(errs) => {
            println!("Could not find MPI library for various reasons:\n");
            for (i, err) in errs.iter().enumerate() {
                println!("Reason #{}:\n{}\n", i, err);
            }
            panic!();
        }
    };

    // Tell Cargo to link the MPI libraries, as in rsmpi/build.rs.
    // TODO - some of the MPI library linking here and MPI library path-setting
    // below may be unnecessary.
    // TODO - make sure that linking MPI libraries here does not create incompatibilities
    // with rsmpi.
    for dir in &mpi_lib.lib_paths {
        println!("cargo:rustc-link-search=native={}", dir.display());
    }
    for lib in &mpi_lib.libs {
        println!("cargo:rustc-link-lib={}", lib);
    }

    // Set up builder with MPI, PETSc, and SLEPc library and include paths.
    let mut builder = bindgen::Builder::default();

    for lib in &mpi_lib.libs {
        builder = builder.link(lib.clone());
    }
    for dir in &mpi_lib.lib_paths {
        builder = builder.clang_arg(format!("-L{}", dir.display()));
    }
    for dir in &mpi_lib.include_paths {
        builder = builder.clang_arg(format!("-I{}", dir.display()));
    }

    builder = builder.link("petsc");
    builder = builder.clang_arg(format!("-L{}", petsc_lib_dir.display()));
    builder = builder.clang_arg(format!("-I{}", petsc_include_dir.display()));
    builder = builder.clang_arg(format!("-I{}", petsc_arch_include_dir.display()));
    builder = builder.link("slepc");
    builder = builder.clang_arg(format!("-L{}", slepc_lib_dir.display()));
    builder = builder.clang_arg(format!("-I{}", slepc_include_dir.display()));
    builder = builder.clang_arg(format!("-I{}", slepc_arch_include_dir.display()));

    // Generate SLEPc bindings.
    // Hide types which generate duplicate definitions:
    // https://stackoverflow.com/a/34379937
    let bindings = builder
        .header("wrapper.h")
        .blacklist_type("FP_NAN")
        .blacklist_type("FP_INFINITE")
        .blacklist_type("FP_ZERO")
        .blacklist_type("FP_SUBNORMAL")
        .blacklist_type("FP_NORMAL")
        .generate()
        .expect("Unable to generate SLEPc bindings");

    // Write out SLEPc bindings.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write SLEPc bindings");
}
