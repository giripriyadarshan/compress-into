use clap::{self, Parser};
use indicatif::HumanDuration;
use std::fs;
use std::io::prelude::*;
use std::path::Path;
use std::time::Instant;
use zip::ZipWriter;

#[derive(Parser, Default, Debug)]
#[command(author, version, about, long_about = None)] // Read from `Cargo.toml`
struct Arguments {
    #[clap(short, long)]
    path: Option<Vec<String>>,

    #[clap(short, long)]
    specific_path: Option<String>,

    #[clap(short, long, default_value = "0")]
    level: u8,
}

#[tokio::main]
async fn main() {
    let started = Instant::now();

    let args = Arguments::parse();

    println!("{:?}", args);

    let zip_path = Path::new("./output/test.zip");
    let file = fs::File::create(&zip_path).unwrap();
    let mut zip = ZipWriter::new(file);
    let options =
        zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Deflated);

    let folders = vec![
        Path::new("./test-input/folder1"),
        Path::new("./test-input/folder2"),
    ];

    for folder in folders {
        for entry in fs::read_dir(folder).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                zip.add_directory(&path.to_str().unwrap().to_string(), options)
                    .unwrap();
            } else {
                zip.start_file(
                    path.file_name().unwrap().to_str().unwrap().to_string(),
                    options,
                )
                .unwrap();
                let mut file = fs::File::open(&path).unwrap();
                let mut buffer = Vec::new();
                file.read_to_end(&mut buffer).unwrap();
                zip.write_all(&*buffer).unwrap();
            }
        }
    }
    zip.finish().unwrap();

    println!("Finished in {}", HumanDuration(started.elapsed()));
}
