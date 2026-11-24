use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn missing_release_fails_cleanly_in_cli_process() {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock must be after the Unix epoch")
        .as_nanos();
    let missing_release = std::env::temp_dir().join(format!(
        "jackrabbit-release-missing-{}-{nonce}",
        std::process::id()
    ));
    assert!(!missing_release.exists());

    let output = Command::new(env!("CARGO_BIN_EXE_jackrabbit-installer-cli"))
        .arg("install")
        .arg(&missing_release)
        .output()
        .expect("installer CLI must start");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(
        output.status.code(),
        Some(2),
        "installer must return its normal error status\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
    assert!(
        stderr.contains("JR-CLI-RELEASE-MISSING"),
        "installer must report the stable missing-release error\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
}
