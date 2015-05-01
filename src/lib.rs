#![feature(scoped)]
#![cfg_attr(test, feature(test))]
use std::sync::{Arc};
use std::thread;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::mem;
pub fn pmap<A, B, F>(array:  &[A], out:&mut [B], max: usize, f: F)
where F: Send+Sync, F: Fn(A)->B,  A: Send+Sync+Copy,  B: Send+Sync
  {
    assert!(out.len() >= array.len());
    let idx = &AtomicUsize::new(0);
    let len = array.len();
    let array = Arc::new(array);
    let f = Arc::new(f);
    {
        let mut threads: Vec<thread::JoinGuard<bool>> = Vec::new();
        for _ in 0..max {
            let data: &mut [B] = unsafe{ mem::transmute_copy(&out) };
            let f = f.clone();
            let array = array.clone();
            let trd = thread::scoped(move|| {
                loop {
                    let i = idx.fetch_add(1, Ordering::Relaxed);
                    if i >= len {
                        return true;
                    }
                    let in_data = array[i];
                    let out_data = f(in_data);
                    data[i] = out_data;
                    //println!("inside {} {} {:?}", i, j, data.deref());
                }
            });
            threads.push(trd);
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate test;
    use super::pmap;


    #[test]
    fn it_works() {
        let behind = [1usize, 2, 3, 4, 5];
        let mut thing = [0usize; 5];
        pmap(&behind, &mut thing, 2, inc);
        let out = [1usize, 2, 6,24,120];
        assert_eq!(thing, out);
    }
    fn inc (x: usize) -> usize {
      inc2(1, x)
    }
    fn inc2 (acc: usize, x: usize) -> usize {
        match x {
          1=> acc,
          _=> inc2(acc * x, x - 1)
      }
    }
}


#[cfg(test)]
mod bench {
    extern crate test;
    extern crate rand;
    use super::pmap;
    use self::test::Bencher;
    use self::rand::Rng;
    fn fill(array:  &mut [u8]) {
      let mut rng = rand::chacha::ChaChaRng::new_unseeded();
      let len = array.len();
      for i in 0..len {
        array[i] = rng.gen::<u8>();
      }
    }
    #[bench]
    fn one(bh: & mut Bencher) {
        let mut behind = [0u8; 8000];
        let mut thing = [0u8;8000];
        fill(&mut behind);
        bh.iter( || {
            pmap(&behind, &mut thing, 1, inc);
        });
        bh.bytes = behind.len() as u64;
    }
    #[bench]
    fn two(bh: & mut Bencher) {
        let mut behind = [0u8; 8000];
        let mut thing = [0u8;8000];
        fill(&mut behind);
        bh.iter( || {
            pmap(&behind, &mut thing, 2, inc);
        });
        bh.bytes = behind.len() as u64;
    }
    #[bench]
    fn three(bh: & mut Bencher) {
        let mut behind = [0u8; 8000];
        let mut thing = [0u8; 8000];
        fill(&mut behind);
        bh.iter( || {
            pmap(&behind, &mut thing, 3, inc);
        });
        bh.bytes = behind.len() as u64;
    }
    #[bench]
    fn four(bh: & mut Bencher) {
        let mut behind = [0u8; 8000];
        let mut thing = [0u8;8000];
        fill(&mut behind);
        bh.iter( || {
            pmap(&behind, &mut thing, 4, inc);
        });
        bh.bytes = behind.len() as u64;
    }
    #[bench]
    fn six(bh: & mut Bencher) {
        let mut behind = [0u8; 8000];
        let mut thing = [0u8;8000];
        fill(&mut behind);
        bh.iter( || {
            pmap(&behind, &mut thing, 6, inc);
        });
        bh.bytes = behind.len() as u64;
    }
    #[bench]
    fn eight(bh: & mut Bencher) {
        let mut behind = [0u8; 8000];
        let mut thing = [0u8;8000];
        fill(&mut behind);
        bh.iter( || {
            pmap(&behind, &mut thing, 8, inc);
        });
        bh.bytes = behind.len() as u64;
    }
    fn inc (x: u8) -> u8 {
      inc2(1, x)
    }
    fn inc2 (acc: u8, x: u8) -> u8 {
        match x {
          1=> acc,
          _=> inc2(acc * x, x - 1)
      }
    }
}
