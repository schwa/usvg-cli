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
    let xmltree = usvg::roxmltree::Document::parse(&contents)
        .expect("Should have been able to parse the file");
    let svgtree = usvg::Tree::from_xmltree(&xmltree, &usvg::Options::default())
        .expect("Should have been able to create a tree");
    let processed = svgtree.to_string(&usvg::XmlOptions::default());
    fs::write(args.output, &processed);
}
