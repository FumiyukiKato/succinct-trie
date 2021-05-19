extern crate fsa;

use clap::{AppSettings, Clap};

use std::{collections::{HashSet}, io::{BufRead, BufReader}};
use std::fs::File;

use fsa::config::K_NOT_FOUND;
use fsa::trie::Trie;

#[derive(Clap)]
#[clap(version = "0.1", author = "Fumiyuki K. <fumilemon79@gmail.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    /// Sets input file name. Server data
    #[clap(short, long, default_value = "input.csv")]
    server_input_file: String,

    /// Sets input file name. Client data
    #[clap(short, long, default_value = "input.csv")]
    client_input_file: String,
}

pub fn read_trajectory_hash_from_csv(filename: &str) -> Vec<Vec<u8>> {
    let file = File::open(filename).expect("file open error");
    let reader = BufReader::new(file);
    let mut hash_vec = Vec::new();
    for line in reader.lines().into_iter() {
        if let Ok(hash) = line {
            hash_vec.push(hash.as_bytes().to_vec());
        }
    }
    hash_vec
}

fn main() {
    let opts: Opts = Opts::parse();
    
    let mut server_data = read_trajectory_hash_from_csv(opts.server_input_file.as_str());

    server_data.sort();
    let fsa = Trie::new(&server_data);

    let mut hash_map = HashSet::<Vec<u8>>::with_capacity(server_data.len());
    for key in server_data {
        hash_map.insert(key);
    }

    let client_data = read_trajectory_hash_from_csv(opts.client_input_file.as_str());

    println!("[searching]");
    let mut h_not_found = 0;
    let mut h_found = 0;
    let mut not_found = 0;
    let mut found = 0;

    for key in client_data.iter() {
        if fsa.exact_search(key) != K_NOT_FOUND {
            found += 1;
        } else {
            not_found += 1;
        }

        if hash_map.contains(key) {
            h_found += 1;
        } else {
            h_not_found += 1;
        }

        if (fsa.exact_search(key) != K_NOT_FOUND) ^ hash_map.contains(key) {
            println!("different result! {:}", std::str::from_utf8(&key).unwrap());
        }
    }
    println!("Trie not found: {}, found: {}", not_found, found);
    println!("Hashmap not found: {}, found: {}", h_not_found, h_found);
    
    println!("ok.")
}