fn main() {
    println!("cargo:rustc-link-lib=SPCommon");
    println!(
        "cargo:rustc-link-search=native={}",
        std::env::current_dir().unwrap().display()
    );
}
