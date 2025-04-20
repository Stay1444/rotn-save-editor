include!(concat!(env!("OUT_DIR"), "/build_info.rs"));

#[tokio::main]
async fn main() {
    println!(
        r#"HASH = "{}",
BUILD_DATE = "{}",
TARGET_OS = "{}",
RUSTC_VERSION = "{}",
PROFILE = "{}""#,
        BuildInfo::GIT_HASH,
        BuildInfo::BUILD_DATE,
        BuildInfo::TARGET_OS,
        BuildInfo::RUSTC_VERSION,
        BuildInfo::PROFILE
    );

    tracing_subscriber::fmt::init();

    tracing::info!("Starting");
}
