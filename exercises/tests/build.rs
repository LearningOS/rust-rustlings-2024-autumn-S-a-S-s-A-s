//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

fn main() {
    // For tests7: Set up an environment variable called `TEST_FOO` to the current UNIX timestamp
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs(); 

    // Command to set the environment variable TEST_FOO
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);

    // For tests8: Enable the "pass" feature
    println!("cargo:rustc-cfg=feature=\"pass\"");
}
