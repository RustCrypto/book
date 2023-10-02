fn main() {
    // Generate tests for all markdown files
    let mdbook_path = concat!(env!("CARGO_MANIFEST_DIR"), "/../src/");
    let readme_path = concat!(env!("CARGO_MANIFEST_DIR"), "/../README.md");
    let mut mdbook_files = skeptic::markdown_files_of_directory(mdbook_path);
    mdbook_files.push(readme_path.into());
    skeptic::generate_doc_tests(&mdbook_files);
}
