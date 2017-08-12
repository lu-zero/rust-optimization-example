
trait Clip {
    fn clip8(self) -> u8;
}

impl Clip for i16 {
    #[inline]
    fn clip8(self) -> u8 {
        if self < 0 {
            0
        } else if self > 255 {
            255
        } else {
            self as u8
        }
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
            dst[oidx0 + x * 2 + 0] = o0.wrapping_shr(2).wrapping_add(128).clip8();
            dst[oidx0 + x * 2 + 1] = o1.wrapping_shr(2).wrapping_add(128).clip8();
            dst[oidx1 + x * 2 + 0] = o2.wrapping_shr(2).wrapping_add(128).clip8();
            dst[oidx1 + x * 2 + 1] = o3.wrapping_shr(2).wrapping_add(128).clip8();
        }
        idx0 += sstride;
        idx1 += sstride;
        idx2 += sstride;
        idx3 += sstride;
        oidx0 += dstride * 2;
        oidx1 += dstride * 2;
    }
}

pub fn recombine_plane_unsafe(
    src: &[i16],
    sstride: usize,
    dst: &mut [u8],
    dstride: usize,
    w: usize,
    h: usize,
) {
    unsafe {
        let hw = (w / 2) as isize;
        let mut band0 = src.as_ptr();
        let mut band1 = band0.offset(hw);
        let mut band2 = band0.offset(((h / 2) * sstride) as isize);
        let mut band3 = band2.offset(hw);
        let mut dst0 = dst.as_mut_ptr();
        let mut dst1 = dst0.offset(dstride as isize);
        let hh = (h / 2) as isize;
        for _ in 0..hh {
            let mut b0_ptr = band0;
            let mut b1_ptr = band1;
            let mut b2_ptr = band2;
            let mut b3_ptr = band3;
            let mut d0_ptr = dst0;
            let mut d1_ptr = dst1;
            for _ in 0..hw {
                let p0 = *b0_ptr;
                let p1 = *b1_ptr;
                let p2 = *b2_ptr;
                let p3 = *b3_ptr;
                let s0 = p0.wrapping_add(p2);
                let s1 = p1.wrapping_add(p3);
                let d0 = p0.wrapping_sub(p2);
                let d1 = p1.wrapping_sub(p3);
                let o0 = s0.wrapping_add(s1).wrapping_add(2);
                let o1 = d0.wrapping_add(d1).wrapping_add(2);
                let o2 = s0.wrapping_sub(s1).wrapping_add(2);
                let o3 = d0.wrapping_sub(d1).wrapping_add(2);
                *d0_ptr.offset(0) = (o0 >> 2).wrapping_add(128).clip8();
                *d0_ptr.offset(1) = (o1 >> 2).wrapping_add(128).clip8();
                *d1_ptr.offset(0) = (o2 >> 2).wrapping_add(128).clip8();
                *d1_ptr.offset(1) = (o3 >> 2).wrapping_add(128).clip8();
                b0_ptr = b0_ptr.offset(1);
                b1_ptr = b1_ptr.offset(1);
                b2_ptr = b2_ptr.offset(1);
                b3_ptr = b3_ptr.offset(1);
                d0_ptr = d0_ptr.offset(2);
                d1_ptr = d1_ptr.offset(2);
            }
            band0 = band0.offset(sstride as isize);
            band1 = band1.offset(sstride as isize);
            band2 = band2.offset(sstride as isize);
            band3 = band3.offset(sstride as isize);
            dst0 = dst0.offset((dstride * 2) as isize);
            dst1 = dst1.offset((dstride * 2) as isize);
        }
    }
}
