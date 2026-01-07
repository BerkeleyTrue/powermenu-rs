use std::process::Command;

fn main() {
    // Compile GResource for video/media assets
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let status = Command::new("glib-compile-resources")
        .arg("--sourcedir=resources")
        .arg(&format!("--target={}/resources.gresource", out_dir))
        .arg("resources/resources.gresource.xml")
        .status()
        .expect("Failed to run glib-compile-resources");

    if !status.success() {
        panic!("glib-compile-resources failed");
    }

    println!("cargo:rerun-if-changed=resources/resources.gresource.xml");
    println!("cargo:rerun-if-changed=resources/redlotoo_dead-internet.mp4");
    println!("cargo:rerun-if-changed=resources/style.css");

    relm4_icons_build::bundle_icons(
        // Name of the file that will be generated at `OUT_DIR`
        "icon_names.rs",
        // Optional app ID
        Some("com.bt.powermenu-rs"),
        // Custom base resource path:
        // * defaults to `/com/example/myapp` in this case if not specified explicitly
        // * or `/org/relm4` if app ID was not specified either
        None::<&str>,
        // Directory with custom icons (if any)
        None::<&str>,
        // List of icons to include
        [
            "turn-off",
            "rotation-lock",
            "moon-outline",
            "arrow-circular-small-bottom-right",
            "arrow-into-box",
        ],
    );
}
