use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [u32; n],
    }

    let mut dp = vec![false; n];
    let mut dpn = vec![false; n];

    for i in 0..n {
        dp[i] = true;
    }

    let mut ans = n / 2 + 1;
    for l in 1..(n / 2) {
        let mut f = false;

        for i in 1..n - 1 {
            if i + 1 + l >= n {
                break;
            }

            dpn[i] = dp[i - 1] && dp[i + 1] && (a[i + 1 + l] > l as u32 + 1);
            f |= dpn[i];
        }
        dp = dpn.clone();

        if !f {
            ans = l + 1;
            break;
        }
    }

    println!("{}", ans);
}
