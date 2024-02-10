use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut s = String::from("L");
    for _ in 0..n {
        s += "o";
    }
    s += "ng";

    println!("{}", s);
}
