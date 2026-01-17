fn main() {
    println!("cargo:rerun-if-changed=resources/redlotoo_dead-internet.mp4");
    println!("cargo:rerun-if-changed=resources/redlotoo_dead-internet-atlas.png");
    println!("cargo:rerun-if-changed=resources/redlotoo_dead-internet.png");
}
