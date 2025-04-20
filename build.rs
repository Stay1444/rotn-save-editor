use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    let git_hash = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
        .unwrap_or_else(|_| "unknown".to_string());

    let build_date = chrono::Utc::now().to_rfc3339(); // or `Local` for local time

    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_else(|_| "unknown".into());
    let rustc_version = rustc_version::version()
        .map(|v| v.to_string())
        .unwrap_or_else(|_| "unknown".into());
    let profile = env::var("PROFILE").unwrap_or_else(|_| "unknown".into());

    let build_info = format!(
        r#"
/// Auto-generated build information
pub struct BuildInfo;

impl BuildInfo {{
    pub const GIT_HASH: &'static str = "{git_hash}";
    pub const BUILD_DATE: &'static str = "{build_date}";
    pub const TARGET_OS: &'static str = "{target_os}";
    pub const RUSTC_VERSION: &'static str = "{rustc_version}";
    pub const PROFILE: &'static str = "{profile}";
}}
"#
    );

    fs::write(out_dir.join("build_info.rs"), build_info).expect("Unable to write build_info.rs");

    println!("cargo:rerun-if-changed=.git/HEAD");
    println!("cargo:rerun-if-env-changed=PROFILE");
}
