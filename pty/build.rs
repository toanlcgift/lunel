fn main() {
    #[cfg(target_os = "ios")]
    {
        println!("cargo:rustc-cfg=ios");
    }
}
