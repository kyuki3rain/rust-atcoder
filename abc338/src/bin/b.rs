use proconio::input;
use std::collections::BTreeMap;

fn main() {
    input! {
        s: String,
    }

    let mut map: BTreeMap<char, i32> = BTreeMap::new();
    for (_, c) in s.chars().enumerate() {
        let entry = map.entry(c);
        entry.and_modify(|v| *v += 1).or_insert(1);
    }

    let mux_char = map.iter().rev().max_by_key(|&(i, v)| v).unwrap().0;
    println!("{}", mux_char);
}
