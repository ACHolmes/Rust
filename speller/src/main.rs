use std::env;
use std::fs;
use std::collections::HashMap;
use std::time::{Duration, Instant};

fn main() {
    // Timing
    let prog_start = Instant::now();

    // Handling cmd line args
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 { println!("Usage: cargo run src/dictionaries/... src/texts/..."); return;}
    let config = parse_config(&args);

    // Reading input
    let dict = fs::read_to_string(config.dict_path)
        .expect("Should have been able to read the file");

    let contents: String = fs::read_to_string(config.text_path)
        .expect("Failed to read dictionary");

    // Creating dictionary
    let mut dict_size = 0;
    let hash_map = build_hash_map(&dict, &mut dict_size);

    // CORRECTNESS NOTE
    // Get slightly different results from the CS50 spec, they handle numbers in a different way (that is, in my opinion, worse)
    // Mine ignores any string with numbers, front or back (e.g. M25, 40's), CS50 spec only ignores the first, but would count 40's as misspelled
    // since it reaches the end of the number, then finds 's and counts that as a word.
    
    // Spell check counters
    let mut misspelled = 0;
    let mut correctly_spelled = 0;

    // Replacing unwanted punctuation with spaces to maintain separation of words
    let contents : String = contents.to_lowercase().chars()
    .map(|x| match x {
        'a'..='z' => x,
        '0'..='9' => x,
        '\'' => x,
        _ => ' '
    }).collect();

    // Splitting by whitespace, and ignoring any strings that contain numbers or no letters (e.g. some texts have '' or other random punctuation)
    for word in contents.split_whitespace(){
        if word.chars().any(|x| x.is_numeric()) { continue; }
        if !word.chars().all(|x| x.is_ascii_lowercase() || x == '\'') {continue;}
        if !hash_map.contains_key(word) {
            misspelled += 1;
            println!("{word}");
        }
        else { correctly_spelled += 1 }
    }
    // Timing
    let prog_end = Instant::now();
    let total_time = prog_end.duration_since(prog_start);

    // Printing results
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