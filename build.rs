use std::env;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    
    // Informa ao linker para procurar scripts no diretório atual
    println!("cargo:rustc-link-search=.");
    
    // Força recompilação se o linker script mudar
    println!("cargo:rerun-if-changed=linker.ld");
    
    // Força recompilação se arquivos de assembly mudarem
    println!("cargo:rerun-if-changed=src/arch/exceptions.S");
    println!("cargo:rerun-if-changed=src/boot.rs");
    
    // Informações do target para o código (usando env vars do cargo)
    println!("cargo:rustc-env=TARGET_ARCH=aarch64");
    println!("cargo:rustc-env=TARGET_CPU=cortex-a53");
    println!("cargo:rustc-env=BUILD_TARGET=raspberry-pi-3b-plus");
    
    // Configurações específicas do kernel
    println!("cargo:rustc-link-arg=-Map=kernel.map");  // Gera mapa de memória
    
    // Verifica se o linker script existe
    if !std::path::Path::new("linker.ld").exists() {
        panic!("linker.ld não encontrado! O script do linker é necessário para o build.");
    }
    
    // Salva informações de build
    std::fs::write(
        out_dir.join("build_info.rs"),
        format!(
            r#"
pub const BUILD_TIME: &str = "Compiled with Rust";
pub const TARGET_ARCH: &str = "{}";
pub const TARGET_CPU: &str = "{}";
pub const KERNEL_VERSION: &str = "{}";
"#,
            env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_else(|_| "aarch64".to_string()),
            "cortex-a53",
            env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| "0.1.0".to_string())
        ),
    ).unwrap();
}