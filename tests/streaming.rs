use marked_rs::{parse, IncrementalParser, Options, StreamParser};

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
    assert_eq!(stream.push("```\n\n"), "<pre><code class=\"language-rust\">let x = 1;\n\n</code></pre>\n");
}

#[test]
fn incremental_edits_refresh_html_and_keep_options() {
    let mut document = IncrementalParser::with_options("# Title\n\nThis is ~~old~~.", Options { gfm: true });
    let start = document.source().find("old").unwrap();
    assert_eq!(document.replace(start..start + 3, "new").unwrap(), "<h1>Title</h1>\n<p>This is <del>new</del>.</p>\n");
}
