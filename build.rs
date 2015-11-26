fn main() {
    println!("cargo:rustc-link-lib=static=x264");
    println!("cargo:rustc-link-lib=static=mp3lame");
    println!("cargo:rustc-link-lib=static=postproc");
}
