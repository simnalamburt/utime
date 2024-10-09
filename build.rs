use version_check::is_min_version;

fn main() {
    println!("cargo::rustc-check-cfg=cfg(rustc_1_75_plus)");
    println!("cargo::rustc-check-cfg=cfg(rustc_1_10_plus)");
    if is_min_version("1.75.0").unwrap_or(true) {
        println!("cargo::rustc-cfg=rustc_1_75_plus");
    }
    if is_min_version("1.10.0").unwrap_or(true) {
        println!("cargo::rustc-cfg=rustc_1_10_plus");
    }
}
