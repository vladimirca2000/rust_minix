fn main() {
    // Informa ao linker para procurar scripts no diretório atual
    println!("cargo:rustc-link-search=.");
    
    // Força recompilação se o linker script mudar
    println!("cargo:rerun-if-changed=linker.ld");
}