use rand::prelude::SliceRandom;
use rand::thread_rng;

// 0 - 9, a - z, A - Z
const POSSIBLE_CHARS: [u8; 62] = [
    48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78,
    79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106,
    107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122,
];

pub fn random_string(size: usize) -> String {
    let mut rng = thread_rng();
    let mut arr: Vec<u8> = vec![0; size];

    for i in 0..size {
        arr[i] = *POSSIBLE_CHARS.choose(&mut rng).unwrap();
    }

    String::from_utf8(arr).unwrap()
}

pub fn create_short_url(unique_id: &str) -> String {
    format!("https://su.at/{}", unique_id)
}
