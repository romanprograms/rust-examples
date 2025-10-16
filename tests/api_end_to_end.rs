#[test]
fn workspace_is_wired() {
    // Ensure we can call into core-utils from test context.
    assert_eq!(core_utils::greet("tester"), "Hello, tester!");
}