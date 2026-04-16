use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let target = env::var("TARGET").unwrap();
    if target.contains("windows") {
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let out_dir = env::var("OUT_DIR").unwrap();
        let rc_path = PathBuf::from(&manifest_dir).join("prompt.rc");
        let res_path = PathBuf::from(&out_dir).join("prompt.res");

        // Find rc.exe
        let rc = r#"C:\Program Files (x86)\Windows Kits\10\bin\10.0.19041.0\x64\rc.exe"#;

        let status = Command::new(rc)
            .arg("/fo")
            .arg(&res_path)
            .arg(&rc_path)
            .status()
            .expect("failed to run rc.exe");

        if !status.success() {
            panic!("rc.exe failed");
        }

        println!("cargo:rustc-link-arg-bin=prompt={}", res_path.display());
    }
}
