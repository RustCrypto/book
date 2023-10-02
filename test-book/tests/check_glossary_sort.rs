use std::fmt::Write;

#[test]
fn check_glossary_sort() {
    // Make sure glossary headings are in order
    let glossary_path = concat!(env!("CARGO_MANIFEST_DIR"), "/../src/glossary.md");
    let text = std::fs::read_to_string(glossary_path).unwrap();

    let mut linenums = vec![];
    let mut headings = vec![];
    text.lines()
        .enumerate()
        .filter(|(_, line)| line.starts_with("## "))
        .filter(|(_, line)| !line.contains("sort:ignore"))
        .for_each(|(idx, line)| {
            linenums.push(idx);
            headings.push(line.strip_prefix("## ").unwrap());
        });

    let unsorted = headings.clone();
    headings.sort_unstable();
    let longest = headings.iter().max_by_key(|h| h.len()).unwrap_or(&"").len();

    if !(unsorted == headings) {
        let mut msg = format!(
            "glossary headings are not currently sorted\n\n\
            line   {:<longest$}   sorted\n",
            "original"
        );
        for ((num, orig), expected) in linenums.iter().zip(unsorted.iter()).zip(headings.iter()) {
            writeln!(msg, "{num:<4}   {orig:<longest$}   {expected}").unwrap();
        }
        panic!("{msg}\n");
    }
}
