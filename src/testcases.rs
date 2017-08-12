#[inline]
pub fn clip8(a: i16) -> u8 {
    if a < 0 {
        0
    } else if a > 255 {
        255
    } else {
        a as u8
    }
}

pub fn recombine_plane_reference(
    src: &[i16],
    sstride: usize,
    dst: &mut [u8],
    dstride: usize,
    w: usize,
    h: usize,
) {
    let mut idx0 = 0;
    let mut idx1 = w / 2;
    let mut idx2 = (h / 2) * sstride;
    let mut idx3 = idx2 + idx1;
    let mut oidx0 = 0;
    let mut oidx1 = dstride;

    for _ in 0..(h / 2) {
        for x in 0..(w / 2) {
            let p0 = src[idx0 + x];
            let p1 = src[idx1 + x];
            let p2 = src[idx2 + x];
            let p3 = src[idx3 + x];
            let s0 = p0.wrapping_add(p2);
            let d0 = p0.wrapping_sub(p2);
            let s1 = p1.wrapping_add(p3);
            let d1 = p1.wrapping_sub(p3);
            let o0 = s0.wrapping_add(s1).wrapping_add(2);
            let o1 = d0.wrapping_add(d1).wrapping_add(2);
            let o2 = s0.wrapping_sub(s1).wrapping_add(2);
            let o3 = d0.wrapping_sub(d1).wrapping_add(2);
            dst[oidx0 + x * 2 + 0] = clip8(o0.wrapping_shr(2).wrapping_add(128));
            dst[oidx0 + x * 2 + 1] = clip8(o1.wrapping_shr(2).wrapping_add(128));
            dst[oidx1 + x * 2 + 0] = clip8(o2.wrapping_shr(2).wrapping_add(128));
            dst[oidx1 + x * 2 + 1] = clip8(o3.wrapping_shr(2).wrapping_add(128));
        }
        idx0 += sstride;
        idx1 += sstride;
        idx2 += sstride;
        idx3 += sstride;
        oidx0 += dstride * 2;
        oidx1 += dstride * 2;
    }
}
