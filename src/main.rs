#[macro_use]
extern crate itertools;
extern crate time;
extern crate lib;

use time::PreciseTime;

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
    let count = 10000;
    let src = vec![42; 600 * 1024];
    let mut a = vec![0; 600 * 1024 * 8];
    let mut b = vec![0; 600 * 1024 * 8];

    benchme("reference", count, || lib::recombine_plane_reference(&src, 360, &mut a, 368, 360, 288));
    benchme("unsafe", count, || lib::recombine_plane_unsafe(&src, 360, &mut b, 368, 360, 288));

    cmp(&a, &b);

    benchme("reference 32bit", count, || lib::recombine_plane_reference_32(&src, 360, &mut a, 368, 360, 288));
    benchme("unsafe 32bit", count, || lib::recombine_plane_unsafe_32(&src, 360, &mut b, 368, 360, 288));

    cmp(&a, &b);

    benchme("chunks", count, || lib::recombine_plane_chunks(&src, 360, &mut b, 368, 360, 288));

    cmp(&a, &b);

    benchme("chunks 32bit", count, || lib::recombine_plane_chunks_32(&src, 360, &mut b, 368, 360, 288));

    cmp(&a, &b);
}
