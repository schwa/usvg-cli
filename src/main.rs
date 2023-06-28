use clap::Parser;
use std::fs;
use usvg::{TreeParsing, TreeWriting};

#[derive(Parser)]
struct Cli {
    input: std::path::PathBuf,
    output: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    let contents = fs::read_to_string(args.input).expect("Should have been able to read the file");
    let xml_tree = usvg::roxmltree::Document::parse(&contents)
        .expect("Should have been able to parse the file");
    let svg_tree = usvg::Tree::from_xmltree(&xml_tree, &usvg::Options::default())
        .expect("Should have been able to create a tree");
    let processed = svg_tree.to_string(&usvg::XmlOptions::default());
    fs::write(args.output, &processed).expect("Should have been able to write the file");
}
