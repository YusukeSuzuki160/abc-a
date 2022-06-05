// -*- coding:utf-8-unix -*-

use proconio::input;

// ABC086C - Traveling
// https://atcoder.jp/contests/abs/tasks/arc089_a

fn main() {
    input! {
        n: usize,
    }
    let m = n % 100;
    if m < 10 {
        println!("0{}", m);
    }
    else {
        println!("{}", m);
    }
}
