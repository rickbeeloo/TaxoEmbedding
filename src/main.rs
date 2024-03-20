
use std::fs::File;
use std::io::{BufReader, BufRead, BufWriter, Write};
use std::collections::HashSet;
use serde::{Deserialize, Serialize};
use serde_json::Deserializer;
use std::path::PathBuf;
use glob::glob;


fn read_taxo_dump(taxo_dump_path: &str) -> HashSet<String>{
    let file = File::open(taxo_dump_path).expect("Could not find file");
    let reader = BufReader::new(file);

    let mut taxo_names: HashSet<String> = HashSet::new();

    for line in reader.lines() {
        let line = line.expect("Failed to read line"); // Handle the Result
        let mut parts = line.split("|");
        if let Some(taxo_name) = parts.nth(1) {
            let taxo_name = taxo_name.trim().to_lowercase();
           // println!("Taxo name: {}", taxo_name);
            taxo_names.insert(taxo_name.to_string());
        } 
    }
    println!("Taxo names loaded: {}", taxo_names.len());
    taxo_names
}


#[derive(Deserialize, Debug)]
struct Obj {
    items: Vec<Article>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Article {
    pmid: u32, 
    year: u32, 
    abstract_text: String, 
    abstract_title: String, 
    abstract_authors_list: Vec<String>,
}

fn scan_pubmed(pubmed_json_file: &PathBuf, targets: &HashSet<String>, writer:&mut BufWriter<&mut File>) {
    let file = File::open(pubmed_json_file).expect("File not there");
    let reader = BufReader::new(file);

    let articles: Vec<Article> = serde_json::from_reader(reader).unwrap();

    let mut lines_found = 0;
    // Iterate over deserialized items
    for article in articles {

        // Check the title
        let title = article.abstract_title.to_lowercase();
        for word in title.split_whitespace() {
            if targets.contains(word) {
                lines_found += 1;
                writer.write_all(title.as_bytes()).expect("Failed to write!");
                writer.write_all(&[b'\n']).unwrap();
                break;
            }
        }

        // Check the abstract for lines
        for line in article.abstract_text.to_lowercase().split(". ") {
            for word in line.split_whitespace() {
                if targets.contains(word) {
                   // println!("{}", line);
                   lines_found += 1;
                    writer.write_all(line.as_bytes()).expect("Failed to write!");
                    writer.write_all(&[b'\n']).unwrap();
                    break;
                }
            }
        }
    }

    println!("Done with chunk: {} lines found", lines_found);

}

fn scan_pubmed_portions(json_folder: &str, target:HashSet<String>, output_path: &str) {

    let mut out_file = File::create(output_path).expect("Can;t open output file");
    let mut writer = BufWriter::new(&mut out_file);

    let pattern: String = json_folder.to_owned() + "/*.json";
    for entry in glob(&pattern).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                println!("Working on file: {:?}", path);
                scan_pubmed(&path, &target, &mut writer);
            }
            Err(e) => eprintln!("Error: {:?}", e),
        }
    }    

}


fn main() {
    let output = "matching_lines.txt";
    let taxo_path = "data/names.dmp".to_string();
    let json_path = "data/pubmed_portion_0.json".to_string();
    let targets = read_taxo_dump(&taxo_path);

    scan_pubmed_portions("data", targets, &output);
    //scan_pubmed(&json_path, &targets);
    
    //
}
