// tests8.rs
//
// This execrise shares `build.rs` with the previous exercise.
// You need to add some code to `build.rs` to make both this exercise and
// the previous one work.
//
// Execute `rustlings hint tests8` or use the `hint` watch subcommand for a
// hint.



fn main() {
    // In tests7, we should set up an environment variable
    // called `TEST_FOO`. Print in the standard output to let
    // Cargo do it.
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs(); // What's the use of this timestamp here?
    let your_command = format!(
        "Your command here with {}, please checkout exercises/tests/build.rs",
        timestamp
    );
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);
    println!("cargo:rustc-cfg=feature=\"pass\"");

    // In tests8, we should enable "pass" feature to make the
    // testcase return early. Fill in the command to tell
    // Cargo about that.
    let your_command = "Your command here, please checkout exercises/tests/build.rs";
    println!("cargo:rustc-cfg=pass");
}
