# Dependencies

bindgen requires LLVM/Clang:

    sudo apt install llvm-3.9-dev libclang-3.9-dev clang-3.9 autoconf texinfo

Build and test PETSc (complex scalars, debug and release mode):

    cd ~
    git clone -b maint https://bitbucket.org/petsc/petsc petsc
    ./configure PETSC_ARCH=arch-linux2-complex-debug --with-cc=mpicc --with-cxx=mpicxx --with-fc=mpif90 --download-fblaslapack --with-scalar-type=complex --with-shared-libraries=0
    make PETSC_DIR=$HOME/petsc PETSC_ARCH=arch-linux2-complex-debug all
    make PETSC_DIR=$HOME/petsc PETSC_ARCH=arch-linux2-complex-debug test

Build and test SLEPc. SLEPc reads the options which were used to compile PETSc, so these do not need to be specified again.

    cd ~
    wget http://slepc.upv.es/download/distrib/slepc-3.8.1.tar.gz
    tar -xvzf slepc-3.8.1.tar.gz
    cd slepc-3.8.1
    SLEPC_DIR=$PWD PETSC_DIR=$HOME/petsc PETSC_ARCH=arch-linux2-complex-debug ./configure
    make SLEPC_DIR=$PWD PETSC_DIR=$HOME/petsc PETSC_ARCH=arch-linux2-complex-debug
    SLEPC_DIR=$PWD PETSC_DIR=$HOME/petsc PETSC_ARCH=arch-linux2-complex-debug make test

# TODO

Add PETSc and SLEPc build instructions for release mode.

Add PETSc and SLEPc build instructions for real scalars.

Host `petsc-sys` publicly and update `Cargo.toml` accordingly.

Similar issues regarding choosing which version (debug/release, scalar type) to use
as those discussed in `petsc-sys/README.md`.
