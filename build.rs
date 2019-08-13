use std::path::PathBuf;
use std::{env, fs};

fn main() {
    let target = env::var("TARGET").unwrap();
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let name = env::var("CARGO_PKG_NAME").unwrap();

    let feature_interrupts_qregs = env::var("CARGO_FEATURE_INTERRUPTS_QREGS").is_ok();
    let feature_inline_asm = env::var("CARGO_FEATURE_INLINE_ASM").is_ok();

    if target.starts_with("riscv") && !feature_inline_asm {
        let target_flags_regex = regex::Regex::new("riscv32([a-z]+)-unknown-none-elf").unwrap();
        let unimportant_flags = regex::Regex::new("[ma]").unwrap();
        let target_with_important_flags = unimportant_flags.replace_all(
            target_flags_regex
                .captures(&target)
                .expect(
                    "RISC-V target doesn't match the pattern 'riscv32([a-z]+)-unknown-none-elf'",
                )
                .get(1)
                .unwrap()
                .as_str(),
            "",
        );

        let target_with_important_flags =
            format!("riscv32{}-unknown-none-elf", target_with_important_flags);

        let cpu_features = if feature_interrupts_qregs {
            "RV32RT_INTERRUPTS_QREGS"
        } else {
            "RV32RT_BARE"
        };

        let lib_name = format!("{}_{}", target_with_important_flags, cpu_features);

        fs::copy(
            format!("bin/{}.a", lib_name),
            out_dir.join(format!("lib{}.a", name)),
        ).unwrap();

        println!("cargo:rustc-link-lib=static={}", name);
        println!("cargo:rustc-link-search={}", out_dir.display());
    }

    if target.starts_with("riscv32") {
        println!("cargo:rustc-cfg=riscv");
        println!("cargo:rustc-cfg=riscv32");
    }
}
