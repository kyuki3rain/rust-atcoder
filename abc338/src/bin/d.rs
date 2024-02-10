use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        x: [u32; m],
    }

    let mut d = 0;
    let mut costs = vec![0; n];
    for i in 0..m - 1 {
        let min_x = x[i].min(x[i + 1]) as i32;
        let max_x = x[i].max(x[i + 1]) as i32;
        let l1: i32 = max_x - min_x;
        let l2: i32 = (n as i32 - l1).abs();
        let cost = (l2 - l1).abs();

        d += l1.min(l2) as u32;

        match l1.cmp(&l2) {
            std::cmp::Ordering::Less => {
                for j in (min_x - 1)..(max_x - 1) {
                    costs[j as usize] += cost as u32;
                }
            }
            std::cmp::Ordering::Greater => {
                for j in (max_x - 1)..(min_x - 1 + n as i32) {
                    costs[(j % n as i32) as usize] += cost as u32;
                }
            }
            std::cmp::Ordering::Equal => {}
        }
    }

    println!("{}", d + costs.iter().min().unwrap());
}
