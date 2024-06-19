#[cfg(all(unix, not(target_os = "macos")))]
fn main() {
    // add unix dependencies below
    println!("cargo:rustc-flags=-l pcre2-8");
}

#[cfg(target_os = "macos")]
fn main() {
    // add macos dependencies below
    println!("cargo:rustc-flags=-l pcre2-8");
}
