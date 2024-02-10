use proconio::input;

fn main() {
    input! {
        n: usize,
        q: [u32; n],
        a: [u32; n],
        b: [u32; n],
    }

    let mut min_a = u32::MAX;

    for i in 0..n {
        if a[i] != 0 {
            min_a = min_a.min(q[i] / a[i]);
        }
    }

    let mut q_bar = vec![0; n];
    for i in 0..n {
        q_bar[i] = q[i] - min_a * a[i];
    }

    let mut min_b = u32::MAX;
    for i in 0..n {
        if b[i] != 0 {
            min_b = min_b.min(q_bar[i] / b[i]);
        }
    }

    let mut ans = min_a + min_b;
    for i in (0..min_a).rev() {
        let mut min_b = u32::MAX;
        for j in 0..n {
            q_bar[j] += a[j];
            if b[j] != 0 {
                min_b = min_b.min(q_bar[j] / b[j]);
            }
        }

        if ans <= i + min_b {
            ans = i + min_b;
        }
    }
    println!("{}", ans);
}
