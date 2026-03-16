#[cfg(all(unix, not(target_os = "macos")))]
fn main() {
    println!("cargo:rustc-link-lib=hunspell-1.7");
    println!("cargo:rustc-link-lib=ncursesw");
    println!("cargo:rustc-link-lib=pcre2-8");
}

#[cfg(target_os = "macos")]
fn main() {
    // add macos dependencies below
    // println!("cargo:rustc-flags=-l edit");
}
