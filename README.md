# KBuild kmods utility

This utility will print which of the kernel modules are loaded by the running kernel so that you can filter unnecessary modules from the kernel configuration to build the kernel faster from source.

## Building & running
    $ cargo build --release
    $ target/release/kbuild_kmods_util $(uname -r)
    
## Output
    Loaded modules (150):
        /lib/modules/5.13.0-rc3-202105300039+/kernel/arch/x86/crypto/blake2s-x86_64.ko: blake2s_x86_64
        /lib/modules/5.13.0-rc3-202105300039+/kernel/arch/x86/crypto/chacha-x86_64.ko: chacha_x86_64
        /lib/modules/5.13.0-rc3-202105300039+/kernel/arch/x86/crypto/crc32-pclmul.ko: crc32_pclmul
        /lib/modules/5.13.0-rc3-202105300039+/kernel/arch/x86/crypto/crc32c-intel.ko: crc32c_intel
        ...
    Not loaded modules (1746):
        /lib/modules/5.13.0-rc3-202105300039+/kernel/arch/x86/crypto/aegis128-aesni.ko: aegis128_aesni
        /lib/modules/5.13.0-rc3-202105300039+/kernel/arch/x86/crypto/blowfish-x86_64.ko: blowfish_x86_64
        /lib/modules/5.13.0-rc3-202105300039+/kernel/arch/x86/crypto/camellia-aesni-avx-x86_64.ko: camellia_aesni_avx_x86_64
        /lib/modules/5.13.0-rc3-202105300039+/kernel/arch/x86/crypto/camellia-aesni-avx2.ko: camellia_aesni_avx2
        ...
    Loaded but not on disk (0):
        ...

