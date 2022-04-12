#![allow(unused_attributes)]
extern crate prusti_contracts;
use prusti_contracts::*;

#[path = "lib/matwrapper.rs"]
pub mod matwrapper;
use matwrapper::MatWrapper;

#[path = "lib/vecwrapper.rs"]
pub mod vecwrapper;
use vecwrapper::VecWrapper;

//#[lr::sig(fn (arr2: &RMat<f32>[m,n], m:usize{0 < m}, n: usize{ 0 < n}) -> bool)]
#[requires(0 < _m && 0 < n)]
#[requires(_m == arr2.rows() && n == arr2.cols())]
pub fn is_neg(arr2: &MatWrapper<f32>, _m:usize, n: usize) -> bool {
  let mut j = 1;
  while j < n - 1 {
    if arr2.get(0, j) < 0.0 {
      return true
    }
    j += 1;
  }
  false
}

//#[lr::sig(fn (m:usize{0 < m}, n:usize{0 < n}, arr2: &RMat<f32>[m, n]) -> bool)]
#[requires(0 < m && 0 < n)]
#[requires(m == arr2.rows() && n == arr2.cols())]
pub fn unb1(m:usize, n:usize, arr2: &MatWrapper<f32>) -> bool {
  let mut i = 0;
  let mut j = 1;

  // INV: 0 < i <= m, 0 <= j < n
  while j < n - 1 {
    if arr2.get(0, j) < 0.0 {
      i = i + 1;
      loop {
        if i < m {
          if arr2.get(i, j) < 0.0 {
            i = i + 1
          } else {
            i = 0;
            j = j + 1;
            break;
          }
        } else {
          return true
        }
      }
    } else {
      i = 0;
      j = j + 1;
    }
  }
  false
}

//#[lr::sig(fn (m:usize{0<m}, n:usize{2<n}, arr2: &RMat<f32>[m,n]) -> usize{v: 0<v && v+1<n})]
#[requires(0 < _m && 2 < n)]
#[requires(_m == arr2.rows() && n == arr2.cols())]
#[ensures(0 < result && result + 1 < n)]
pub fn enter_var(_m:usize, n:usize, arr2: &MatWrapper<f32>) -> usize {
  let mut c  = arr2.get(0, 1);
  let mut j  = 1;
  let mut j_ = 2;
  while j_ < n - 1 {
    body_invariant!(0 < j && j + 1 < n);
    // INV j+1 < n, j_ < n
    let c_ = arr2.get(0, j_);
	  if c_ < c {
      j = j_;
      c = c_;
    }
    j_ += 1
  }
  j
}

//#[lr::sig(fn(m:usize, n:usize, arr2: &RMat<f32>[m, n], j:usize{0 < j && j < n}, i0:usize{0 < i0 && i0 < m}, r0:f32) -> usize{v:0 < v && v < m})]
#[requires(0 < m && 0 < n
  && m == arr2.rows() && n == arr2.cols()
  && 0 < j && j < n
  && 0 < i0 && i0 < m)]
#[ensures(0 < result && result < m)]
pub fn depart_var(m:usize, n:usize, arr2: &MatWrapper<f32>, j:usize, i0:usize, r0:f32) -> usize {
  let mut i  = i0;
  let mut r  = r0;
  let mut i_ = i + 1;
  while i_ < m {
    body_invariant!(0 < i && i < m);

    let c_ = arr2.get(i_, j);
    if 0.0 < c_ {
        let r_ = arr2.get(i_, n-1) / c_;
        if r_ < r {
          i = i_;
          r = r_;
        }
        i_ += 1;
    } else {
      i_ += 1
    }
  }
  i
}

//#[warn(unconditional_recursion)]
//#[lr::assume]
//#[lr::sig(fn() -> usize{v:false})]
// #[trusted]
// #[ensures(false)]
// pub fn die() -> usize {
//   panic!("die")
// }

#[trusted]
#[ensures(false)]
pub fn die () -> usize {
  unimplemented!();
}

//cd #[lr::sig(fn (m:usize{0 < m}, n:usize{0 < n}, arr2: &RMat<f32>[m, n], j: usize{0 < j && j < n}) -> usize{v:0 < v && v < m})]
#[requires(0 < m && 0 < _n)]
#[requires(m == arr2.rows() && _n == arr2.cols())]
#[requires(0 < j && j < _n)]
#[ensures(0 < result && result < m)]
pub fn init_ratio_i(m:usize, _n:usize, arr2: &MatWrapper<f32>, j: usize) -> usize {
  let mut i = 1;
  while i < m {
    body_invariant!(0 < i && i < m && j < _n);
    let c = arr2.get(i, j);
    if 0.0 < c {
      return i
    }
    i += 1;
  }

  die() // abort ("init_ratio: negative coefficients!")
}

//#[lr::sig(fn(m:usize{0 < m}, n:usize{0 < n}, arr2: &RMat<f32>[m, n],
//  j: usize{0 < j && j < n}, i:usize{0 < i && i < m}
// ) -> f32)]
#[requires(0 < _m && 0 < n)]
#[requires(arr2.rows() == _m && arr2.cols() == n)]
#[requires(0 < j && j < n)]
#[requires(0 < i && i < _m)]
pub fn init_ratio_c(_m:usize, n:usize, arr2: &MatWrapper<f32>, j: usize, i: usize) -> f32 {
    arr2.get(i, j) / arr2.get(i, n-1)
}

//#[lr::sig(fn (m:usize, n:usize, arr2:&mut RMat<f32>[m,n], i:usize{0 < i && i < m}, j:usize{0 < j && j < n}) -> i32)]
#[requires(arr2.rows() == m && arr2.cols() == n)]
#[requires(0 < j && j < n)]
#[requires(0 < i && i < m)]
#[ensures(arr2.rows() == m && arr2.cols() == n)]
fn row_op(m:usize, n:usize, arr2:&mut MatWrapper<f32>, i:usize, j:usize) {

  // norm(m, n, arr2, i, j);
  // RJ: rename `jj` to `j` to see an error!
  let c = arr2.get(i, j);
  let mut jj = 1;
  while jj < n {
    body_invariant!(jj < arr2.cols());
    body_invariant!(arr2.rows() == m && arr2.cols() == n);
    let tmp = arr2.get(i, jj);
    arr2.set(i, jj, tmp / c);
    jj += 1;
  }

  // ro_op_aux3(m, n, arr2, i, j, 0)
  let mut i_ = 0;
  while i_ < m {
    body_invariant!(arr2.rows() == m && arr2.cols() == n);
    if i_ != i {
      let c_ = arr2.get(i_, j);
      let mut j = 1;
      while j < n {
        body_invariant!(arr2.rows() == m && arr2.cols() == n);
        let cj  = arr2.get(i, j);
        let cj_ = arr2.get(i_, j);
        arr2.set(i_, j, cj_ - cj * c_);
        j += 1
      }
    }
    i_ += 1
  }
}

//#[lr::sig(fn (m:usize{1 < m}, n:usize{2 < n}, arr2:&mut RMat<f32>[m, n]) -> i32)]
#[requires(1 < m && 2 < n)]
#[requires(arr2.rows() == m && arr2.cols() == n)]
pub fn simplex(m:usize, n:usize, arr2:&mut MatWrapper<f32>) {
  while is_neg(arr2, m, n) {
    body_invariant!(arr2.rows() == m && arr2.cols() == n);
    if unb1(m, n, arr2) {
      die();
    } else {
      let j = enter_var(m, n, arr2);
      let i = init_ratio_i(m, n, arr2, j);
      let r = init_ratio_c(m, n, arr2, j, i);
      let i = depart_var(m, n, arr2, j, i, r);
      row_op(m, n, arr2, i, j);
    }
  }
}

pub fn main() {}