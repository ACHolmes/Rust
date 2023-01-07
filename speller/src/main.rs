use std::env;
use std::fs;
use std::collections::HashMap;
use std::time::{Duration, Instant};

fn main() {
    let prog_start = Instant::now();

    let args: Vec<String> = env::args().collect();

    if args.len() != 3 { println!("Usage: cargo run src/dictionaries/... src/texts/..."); return;}

    let config = parse_config(&args);

    println!("Using dictionary: {}", config.dict_path);
    println!("Using text: {}", config.text_path);

    let dict = fs::read_to_string(config.dict_path)
        .expect("Should have been able to read the file");

    let contents: String = fs::read_to_string(config.text_path)
        .expect("Failed to read dictionary");

    // println!("With dictionary:\n{dict} \nand dictionary: \n{contents}");

    let mut dict_size = 0;
    let hash_map = build_hash_map(&dict, &mut dict_size);

    let mut misspelled = 0;
    let mut correctly_spelled = 0;

    // Currently has issuse with texts containing '' or other punctuation garbage
    let contents : String = contents.to_lowercase().chars()
    .map(|x| match x {
        '\'' => x,
        'a'..='z' => x,
        _ => ' '
    }).collect();

    for word in contents.split_whitespace(){
        if !hash_map.contains_key(word) {
            misspelled += 1;
            println!("{word}");
        }
        else { correctly_spelled += 1 }
    }
    let prog_end = Instant::now();
    let total_time = prog_end.duration_since(prog_start);
    let content_size = misspelled + correctly_spelled;

    print_results(misspelled, dict_size, content_size, total_time);    
}

struct Config {
    dict_path: String,
    text_path: String,
}

fn parse_config(args: &[String]) -> Config {
    let dict_path = args[1].clone();
    let text_path = args[2].clone();

    Config { dict_path, text_path }
}

fn build_hash_map<'a> (dict: &'a str, dict_size: &'a mut u32) -> HashMap<&'a str, bool>
{
    let mut hash_map: HashMap<&str, bool> = HashMap::with_capacity(dict.len());
    for word in dict.split_whitespace(){
        hash_map.insert(word, true);
        *dict_size += 1;
    }
    return hash_map;
}

fn print_results(misspelled: u32, dict_size: u32, content_size: u32, total_time: Duration) -> () {
    println!("WORDS MISSPELLED:     {misspelled}");
    println!("WORDS IN DICTIONARY:  {dict_size}");
    println!("WORDS IN TEXT:        {content_size}");
    println!("TIME IN TOTAL:        {:?}", total_time);
}