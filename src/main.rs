use genpdf::fonts::from_files;
use genpdf::Document;
use genpdf::SimplePageDecorator;
use std::fs::read_to_string;
use clap::{Command, Parser};

/// A simple program to convert a text file to a PDF document.


#[derive(Parser)]
#[command(author, version, long_about = None)]
struct Cli {
    /// The text file(s) to convert to PDF
    #[arg(short, long, required = true)]
    files: Vec<String>,
}

fn main() {
    let cli = Cli::parse();
    for file in &cli.files {
        convert_to_pdf(file);
    } 
}

fn convert_to_pdf(file: &str) {
    let font = from_files("/usr/share/fonts/liberation-fonts", "LiberationSans", None)
        .expect("Failed to load font");

    let mut doc = Document::new(font);
    doc.set_title("Test Text to PDF Document");
    let mut decorator = SimplePageDecorator::new();
    decorator.set_margins(10);
    doc.set_page_decorator(decorator);

    let contents = read_to_string(format!("{file}")).expect("Failed to read file");

    contents.lines().for_each(|line| {
        doc.push(genpdf::elements::Paragraph::new(line.to_string()));
    });
    let file = file.split('.').nth(0).unwrap();
    doc.render_to_file(&format!("{file}.pdf")).expect("Failed to write PDF file"); 
}
