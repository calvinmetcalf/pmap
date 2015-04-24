#![feature(scoped)]
#![cfg_attr(test, feature(test))]
use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::atomic::{AtomicUsize, Ordering};

pub fn pmap<A, B, F>(array:  &[A], out:&mut [B], max: usize, f: F)
where F: Send+Sync, F: Fn(A)->B,  A: Send+Sync+Copy,  B: Send
  {
    assert!(out.len() >= array.len());
    let idx = &AtomicUsize::new(0);
    let len = array.len();
    let array = Arc::new(array);
    let data = Arc::new(Mutex::new(out));
    let f = Arc::new(f);
    {
        let mut threads: Vec<thread::JoinGuard<bool>> = Vec::new();
        for _ in 0..max {
            let f = f.clone();
            let data = data.clone();
            let array = array.clone();
            let trd = thread::scoped(move|| {
                loop {
                    let i = idx.fetch_add(1, Ordering::Relaxed);
                    if i >= len {
                        return true;
                    }
                    let in_data = array[i];
                    let out_data = f(in_data);
                    let mut data = data.lock().unwrap();
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
        let mut thing = [0usize;5];
        pmap(&behind, &mut thing, 2, inc);
        let out = [3usize, 4,5,6,7];
        assert_eq!(thing, out);
    }
    fn inc (x: usize) -> usize {
       x + 2
    }
}
