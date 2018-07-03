extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/svpng.c")
        .include("../")
        .compile("libsvpng.a");
}
