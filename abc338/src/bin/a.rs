use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let mut ans = false;
    for (i, c) in s.chars().enumerate() {
        if i == 0 {
            ans |= c.is_uppercase();
        } else {
            ans &= c.is_lowercase();
        }
    }

    println!("{}", if ans { "Yes" } else { "No" });
}
