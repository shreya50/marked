use std::{fs, path::Path};

use marked_rs::{parse, parse_with_options, Options};

#[test]
fn renders_gfm_fixtures_when_enabled() {
    let fixtures = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/gfm");
    let mut sources: Vec<_> = fs::read_dir(&fixtures)
        .expect("read fixture directory")
        .map(Result::unwrap)
        .filter(|entry| entry.path().extension().is_some_and(|extension| extension == "md"))
        .collect();
    sources.sort_by_key(|entry| entry.file_name());

    for source in sources {
        let path = source.path();
        let expected_path = path.with_extension("html");
        let markdown = fs::read_to_string(&path).expect("read Markdown fixture");
        let expected = fs::read_to_string(&expected_path).expect("read HTML fixture");
        assert_eq!(parse_with_options(&markdown, Options { gfm: true }), expected, "fixture {}", path.display());
    }
}

#[test]
fn gfm_extensions_are_disabled_by_default() {
    assert_eq!(parse("This is ~~literal~~."), "<p>This is ~~literal~~.</p>\n");
}
