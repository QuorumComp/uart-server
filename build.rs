#[cfg(not(target_os = "windows"))]
fn main () {
    println!("cargo::rustc-link-arg=-lncursesw");
}

#[cfg(target_os = "windows")]
fn main () {
}
