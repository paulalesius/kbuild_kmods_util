# KBuild kmods utility

This utility will print which of the kernel modules are loaded by the running kernel so that you can filter unnecessary modules from the kernel configuration to build the kernel faster from source.

## Building & running
    $ cargo build --release
    $ target/release/kbuild_kmods_util --help 
    
    USAGE:
        kbuild_kmods_util --release <$(uname -r)>
    
    FLAGS:
        -h, --help       Prints help information
        -V, --version    Prints version information
    
    OPTIONS:
        -r, --release <$(uname -r)>     [default: 5.13.10-202108131625]

