#[cfg(not(feature = "nightly"))]
#[test]
fn invalid_fsm_builders() {
    use std::path::Path;
    use trybuild::TestCases;
    use walkdir::WalkDir;

    let tester = TestCases::new();
    let test_cases = WalkDir::new("src/tests/invalid")
        .into_iter()
        // filtering out directories here stops the walker
        .filter_entry(|entry| entry.file_type().is_dir() ||
            entry.file_name().to_str().map_or(false, |entry| Path::new(entry).extension().map_or(false, |ext| ext.eq_ignore_ascii_case("rs"))))
        // keep only files
        .filter(|entry| entry.as_ref().map_or(false, |entry| entry.file_type().is_file()))
        .map(|entry| entry.unwrap().into_path());

    for test in test_cases {
        tester.compile_fail(test);
    }
}
