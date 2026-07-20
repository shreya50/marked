use marked_rs::{parse, EditError, IncrementalParser, Options, StreamParser};

#[test]
fn streaming_output_matches_a_complete_parse_across_chunk_boundaries() {
    let source = "# Chunked heading\n\nA paragraph.\n\n- one\n- two";
    let mut stream = StreamParser::new();
    let mut html = stream.push("# Chunk");
    html.push_str(&stream.push("ed heading\n\nA par"));
    html.push_str(&stream.push("agraph.\n\n- one\n"));
    html.push_str(&stream.push("- two"));
    html.push_str(&stream.finish());
    assert_eq!(html, parse(source));
}

#[test]
fn streaming_waits_for_a_fenced_block_to_close() {
    let mut stream = StreamParser::new();
    assert_eq!(stream.push("```rust\nlet x = 1;\n\n"), "");
    assert_eq!(
        stream.push("```\n\n"),
        "<pre><code class=\"language-rust\">let x = 1;\n\n</code></pre>\n"
    );
}

#[test]
fn incremental_edits_refresh_html_and_keep_options() {
    let mut document =
        IncrementalParser::with_options("# Title\n\nThis is ~~old~~.", Options { gfm: true });
    let start = document.source().find("old").unwrap();
    assert_eq!(
        document.replace(start..start + 3, "new").unwrap(),
        "<h1>Title</h1>\n<p>This is <del>new</del>.</p>\n"
    );
}

#[test]
fn streaming_preserves_utf8_split_across_chunks_and_gfm_options() {
    let source = "# Café\n\nThis is ~~ready~~.\n\n| Name | Value |\n| --- | ---: |\n| parser | 1 |";
    let mut stream = StreamParser::with_options(Options { gfm: true });
    let mut html = stream.push("# Caf");
    html.push_str(&stream.push("é\n\nThis is ~~ready~~.\n\n| Name | Value |\n"));
    html.push_str(&stream.push("| --- | ---: |\n| parser | 1 |"));
    html.push_str(&stream.finish());
    assert_eq!(
        html,
        marked_rs::parse_with_options(source, Options { gfm: true })
    );
}

#[test]
fn incremental_edits_reject_invalid_ranges_without_changing_html() {
    let mut document = IncrementalParser::new("# Café");
    let original = document.html().to_owned();
    assert_eq!(document.replace(3..99, "x"), Err(EditError::OutOfBounds));
    assert_eq!(
        document.replace(5..6, "x"),
        Err(EditError::InvalidUtf8Boundary)
    );
    assert_eq!(document.html(), original);
}
