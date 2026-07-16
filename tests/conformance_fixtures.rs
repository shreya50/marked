use std::{fs, path::Path};

use marked_rs::parse;

#[test]
fn renders_core_commonmark_fixtures() {
    let fixtures = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/commonmark-core");
    let mut sources: Vec<_> = fs::read_dir(&fixtures)
        .expect("read fixture directory")
        .map(Result::unwrap)
        .filter(|entry| entry.path().extension().is_some_and(|extension| extension == "md"))
        .collect();
    sources.sort_by_key(|entry| entry.file_name());

    assert!(!sources.is_empty(), "fixture directory must not be empty");
    for source in sources {
        let path = source.path();
        let expected_path = path.with_extension("html");
        let markdown = fs::read_to_string(&path).expect("read Markdown fixture");
        let expected = fs::read_to_string(&expected_path).expect("read HTML fixture");
        assert_eq!(parse(&markdown), expected, "fixture {}", path.display());
    }
}
