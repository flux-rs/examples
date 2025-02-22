#![allow(unused_attributes)]
#![feature(register_tool)]
#![register_tool(lr)]

#[path = "lib/rvec.rs"]
pub mod rvec;
use rvec::RVec;

// CREDIT: https://shane-o.dev/blog/binary-search-rust

// ALSO WORKS

// SAFE: [lr::sig(fn(k: i32, items: &n@RVec<i32>{0 <= n}) -> usize{v: 0 <= v  && v <= n})]

#[lr::sig(fn(k: i32, items: &RVec<i32>) -> usize)]
pub fn binary_search(k: i32, items: &RVec<i32>) -> usize {
  let size = items.len();
  if size <= 0 {
    return size;
  }

  let mut low: usize = 0;
  let mut high: usize = size - 1;

  while low <= high {
      // SAFE   let middle = (high + low) / 2;
      // let middle = high + ((high - low) / 2);
      let middle = low + ((high - low) / 2);
      let current = items.get(middle);
      if *current == k {
        return middle;
      }
      if *current > k {
        if middle == 0 {
          return size;
        }
        high = middle - 1
      }
      if *current < k {
        low = middle + 1
      }
  }
  size
}
