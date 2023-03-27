#[cfg(not(feature = "nightly"))]
#[test]
fn invalid_fsm_builders() {
    use trybuild::TestCases;
    use walkdir::WalkDir;

    let tester = TestCases::new();
    let testes = WalkDir::new("src/tests/invalid")
        .into_iter()
        // filtering out directories here stops the walker
        .filter_entry(|entry| entry.file_type().is_dir() ||
            entry.file_name().to_str().map(|entry| entry.ends_with(".rs")).unwrap_or(false))
        // keep only files
        .filter(|entry| entry.as_ref().map(|entry| entry.file_type().is_file()).unwrap_or(false))
        .map(|entry| entry.unwrap().into_path());

    for test in testes {
        tester.compile_fail(test);
    }
}
