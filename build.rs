fn main() {
    println!("cargo:rustc-link-arg=-T./.cargo/linker.ld"); // 遵循链接脚本
    println!("cargo:rustc-link-arg=-r"); // 导出可再分配的 ELF 文件
    println!("cargo:rerun-if-changed=./.cargo/linker.ld"); // 每次更改链接脚本时需要更新
}
