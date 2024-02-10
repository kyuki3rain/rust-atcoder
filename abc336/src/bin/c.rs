use proconio::input;

fn main() {
    input! {
        mut n: u64,
    }

    let mut h = vec![];
    n -= 1;

    while n != 0 {
        h.push(n % 5);
        n /= 5;
    }

    let mut ans: u128 = 0;
    let mut keta: u128 = 1;
    for i in 0..h.len() {
        ans += h[i] as u128 * keta;
        keta *= 10;
    }

    println!("{}", ans * 2);
}
