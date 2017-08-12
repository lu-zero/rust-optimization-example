#[macro_use]
extern crate itertools;
extern crate time;

use time::PreciseTime;
mod testcases;

use testcases::*;

fn cmp(a: &[u8], b: &[u8]) {
    let ia = a.iter();
    let ib = b.iter();

    for (av, bv) in izip!(ia, ib) {
        if av != bv {
            println!("{} != {}", av, bv);
        }
    }
}

fn benchme<F>(name: &str, n: usize, mut f: F)
    where F : FnMut() {
    let start = PreciseTime::now();
    for _ in 0..n {
        f();
    }
    let end = PreciseTime::now();
    println!("Runtime {} {}", name, start.to(end));
}

fn main() {
    let count = 1;
    let a = [0; 1];
    let b = [0; 1];

    benchme("reference", count, || hello_world());

    cmp(&a, &b);
}
