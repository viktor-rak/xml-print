mod types;
use std::env::{self, Args};
use std::fs::File;
use quick_xml::Reader;
use quick_xml::events::Event;
use std::process;
use types::Node;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: xml-print <filename>");
        process::exit(1);
    }
    let file = &args[1];
    println!("Processing file {file}...");
    let mut reader = Reader::from_str(file);
    reader.config_mut().trim_text(true);
    let mut tree = Vec::<Node>::new();
    let mut root = Node::new("root".to_string());
    tree.push(root); //value gets moved here. TODO: fix this so it actually works lol
    loop {
        match reader.read_event() {
            Ok(Event::Start(ref e)) => {
                let name = e.name();
                let name = reader.decoder().decode(name.as_ref()).unwrap();
                let new_node = Node::new(name.to_string());
                if let Some(prev) = tree.last_mut() {
                    prev.add_child(new_node);
                }
            }
            Ok(Event::End(ref e)) => {
                if let Some(node) = tree.pop() {
                    if tree.is_empty() {
                        root.add_child(node);
                    }
                }
            }
            Ok(Event::Eof) => {
                break;
            }
            Err(e) => {
                eprintln!("Something went wrong with parsing: {e}");
                break;
            }
            _ => {
                println!("What the fuck even happened bruh");
            }
        }
    }
}

