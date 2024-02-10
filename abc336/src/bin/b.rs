use proconio::input;

fn main() {
    input! {
        mut n: usize,
    }

    let mut i = 0;
    while n % 2 == 0 {
        n /= 2;
        i += 1;
    }

    println!("{}", i);
}
