use std::process::Command;

fn main() {
    // Capture build timestamp
    let now = chrono::Utc::now();
    println!(
        "cargo:rustc-env=BUILD_TIMESTAMP={}",
        now.format("%Y-%m-%d %H:%M:%S UTC")
    );

    // Try to capture git commit hash
    let git_hash = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .ok()
        .and_then(|output| {
            if output.status.success() {
                String::from_utf8(output.stdout).ok()
            } else {
                None
            }
        })
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    println!("cargo:rustc-env=GIT_COMMIT_HASH={}", git_hash);

    // Detect enabled features
    #[cfg(feature = "qdrant-backend")]
    println!("cargo:rustc-env=VECTOR_DB_BACKEND=Qdrant");

    #[cfg(not(feature = "qdrant-backend"))]
    println!("cargo:rustc-env=VECTOR_DB_BACKEND=LanceDB");

    // Rerun if git HEAD changes
    println!("cargo:rerun-if-changed=.git/HEAD");
    println!("cargo:rerun-if-changed=.git/refs/heads");

    // On Linux, the prebuilt onnxruntime binary (via fastembed → ort-sys) is
    // built against glibc 2.38+ and references __isoc23_strtol/ll/ull. Provide
    // shim symbols so the binary links on older glibc systems.
    #[cfg(target_os = "linux")]
    {
        println!("cargo:rerun-if-changed=build_shims/glibc_compat.c");
        cc::Build::new()
            .file("build_shims/glibc_compat.c")
            .compile("glibc_isoc23_compat");
    }
}
