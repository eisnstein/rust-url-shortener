use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use crate::Data;

pub fn load_urls() -> HashMap<String, Data> {
    let file = File::open("urls.txt").unwrap();
    let reader = BufReader::new(file);
    let mut map: HashMap<String, Data> = HashMap::new();

    for line in reader.lines() {
        let l = line.unwrap();
        let parts = l.split_once(':').unwrap();
        map.insert(
            parts.0.to_string(),
            Data {
                surl: parts.1.to_string(),
                is_synced: true,
            },
        );
    }

    map
}

pub fn store_urls(urls: &mut HashMap<String, Data>) {
    let mut file = File::options().append(true).open("urls.txt").unwrap();

    for url in urls.into_iter() {
        if url.1.is_synced {
            continue;
        }

        writeln!(&mut file, "{}:{}", url.0, url.1.surl).unwrap();

        url.1.is_synced = true;
    }
}
