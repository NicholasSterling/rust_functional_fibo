#![feature(test)]
// #![feature(type_alias_impl_trait)]

use std::iter::repeat_with;

extern crate test;

type Int = u64;

fn a_few(iter: impl Iterator<Item=Int>) -> Vec<Int> {
    iter.take(7).collect()
}

pub fn main() {

    dbg!(a_few(  tuple_n_ns()));
    dbg!(a_few( byhand_n_ns()));
    dbg!(a_few(iterize_n_ns()));
    dbg!(a_few(iterate_n_ns()));

    dbg!(a_few(  tuple_fib()));
    dbg!(a_few( byhand_fib()));
    dbg!(a_few(iterize_fib()));
    dbg!(a_few(iterate_fib()));
}

pub fn tuple_fib() -> impl Iterator<Item=Int> {
    let mut state = (0u64, 1u64);
    repeat_with(move || {
        state = (state.1, state.0 + state.1);
        state.0
    })
}

pub fn tuple_n_ns() -> impl Iterator<Item=Int> {
    let mut state = (1u64, 1u64);
    repeat_with(move || {
        if state.1 == 0 {
            let n = state.0;
            state = (n+1, n);
        } else {
            state = (state.0, state.1-1);
        }
        state.0
    })
}

////////////

pub fn byhand_fib() -> impl Iterator<Item=Int> {
    let mut a = 1u64;
    let mut b = 1u64;
    repeat_with(move || {
        let tmp = a;
        a = b;
        b += tmp;
        tmp
    })
}

pub fn byhand_n_ns() -> impl Iterator<Item=Int> {
    let mut n = 1u64;
    let mut r = 1u64;
    repeat_with(move || {
        if r == 0 {
            r = n;
            n += 1;
        } else {
            r -= 1;
        }
        n
    })
}

////////////

pub fn old_iterize_fib() -> impl Iterator<Item=Int> {
    old_iterize((1u64,1u64), |(a,b)| (b, a+b))
}

pub fn old_iterize_n_ns() -> impl Iterator<Item=Int> {
    old_iterize((1u64,0u64), |(n,r)|
        if r == 0 {
            (n+1, n)
        } else {
            (n, r-1)
        }
    )
}

pub fn iterize_fib() -> impl Iterator<Item=Int> {
    iterize((1u64,1u64), |(a,b)| (b, a+b))
}

pub fn iterize_n_ns() -> impl Iterator<Item=Int> {
    iterize((1u64,0u64), |(n,r)|
        if r == 0 {
            (n+1, n)
        } else {
            (n, r-1)
        }
    )
}

// Produces an Iterator by induction.
// Given an initial state of type (R,S) and a function that produces
// the next state from an existing state, we return an Iterator for the Rs.
// So in (R,S), R is the part that gets (R)eturned by the Iterator,
// and S is any additional (S)tate used internally.
pub fn old_iterize<R: Copy, S: Copy>(s0: (R,S), f: impl Fn((R,S)) -> (R,S)) -> impl Iterator<Item = R> {
    let mut state = s0;
    repeat_with(
        move || { state.swap(f(state)).0 }
    )
}

pub fn iterize<R: Copy, S: Copy, F>(s0: (R,S), f: F) -> impl Iterator<Item = R>
where F: Fn((R,S)) -> (R,S)
{
    let mut state = s0;
    repeat_with(
        move || { state.swap(f(state)).0 }
    )
}

// a.swap(b) sets a to b and returns the old value of a.
pub trait Swap: Sized {
    fn swap(&mut self, value: Self) -> Self;
}
impl<T> Swap for T {
    fn swap(&mut self, new: Self) -> Self {
        std::mem::replace(self, new)
    }
}

////////////

pub fn iterate_fib() -> impl Iterator<Item=Int> {
    itertools::iterate((1u64,1u64), |&(a,b)| (b, a+b))
        .map(|p| p.0)
}

pub fn iterate_n_ns() -> impl Iterator<Item=Int> {
    itertools::iterate((1u64,0u64), |&(n,r)|
        if r == 0 {
            (n+1, n)
        } else {
            (n, r-1)
        }
    ).map(|p| p.0)
}

#[cfg(test)]
mod tests {

    use super::*;
    use test::Bencher;

    const FIB: Int = 12200160415121876737;
    const N_NS: Int = 29820;

    pub fn fib_sum(iter: impl Iterator<Item=Int>) -> Int {
        iter.take(91).sum()
    }

    pub fn n_ns_sum(iter: impl Iterator<Item=Int>) -> Int {
        iter.take(1000).sum()
    }

    /////

   #[test]
    fn tuple_fib_t() {
        assert_eq!(FIB, fib_sum(tuple_fib()));
    }

   #[test]
    fn tuple_n_ns_t() {
        assert_eq!(N_NS, n_ns_sum(tuple_n_ns()));
    }

   #[test]
    fn byhand_fib_t() {
        assert_eq!(FIB, fib_sum(byhand_fib()));
    }

   #[test]
    fn byhand_n_ns_t() {
        assert_eq!(N_NS, n_ns_sum(byhand_n_ns()));
    }

    #[test]
    fn old_iterize_fib_t() {
        assert_eq!(FIB, fib_sum(old_iterize_fib()));
    }

    #[test]
    fn old_iterize_n_ns_t() {
        assert_eq!(N_NS, n_ns_sum(old_iterize_n_ns()));
    }

    #[test]
    fn iterize_fib_t() {
        assert_eq!(FIB, fib_sum(iterize_fib()));
    }

    #[test]
    fn iterize_n_ns_t() {
        assert_eq!(N_NS, n_ns_sum(iterize_n_ns()));
    }

    #[test]
    fn iterate_fib_t() {
        assert_eq!(FIB, fib_sum(iterate_fib()));
    }

    #[test]
    fn iterate_n_ns_t() {
        assert_eq!(N_NS, n_ns_sum(iterate_n_ns()));
    }

    /////

    #[bench]
    fn tuple_fib_b(b: &mut Bencher) {
        b.iter(|| { fib_sum(tuple_fib()) });
    }

    #[bench]
    fn tuple_n_ns_b(b: &mut Bencher) {
        b.iter(|| { n_ns_sum(tuple_n_ns()) });
    }

    #[bench]
    fn byhand_fib_b(b: &mut Bencher) {
        b.iter(|| { fib_sum(byhand_fib()) });
    }

    #[bench]
    fn byhand_n_ns_b(b: &mut Bencher) {
        b.iter(|| { n_ns_sum(byhand_n_ns()) });
    }

    #[bench]
    fn old_iterize_fib_b(b: &mut Bencher) {
        b.iter(|| { fib_sum(old_iterize_fib()) });
    }

    #[bench]
    fn old_iterize_n_ns_b(b: &mut Bencher) {
        b.iter(|| { n_ns_sum(old_iterize_n_ns()) });
    }

    #[bench]
    fn iterize_fib_b(b: &mut Bencher) {
        b.iter(|| { fib_sum(iterize_fib()) });
    }

    #[bench]
    fn iterize_n_ns_b(b: &mut Bencher) {
        b.iter(|| { n_ns_sum(iterize_n_ns()) });
    }

    #[bench]
    fn iterate_fib_b(b: &mut Bencher) {
        b.iter(|| { fib_sum(iterate_fib()) });
    }

    #[bench]
    fn iterate_n_ns_b(b: &mut Bencher) {
        b.iter(|| { n_ns_sum(iterate_n_ns()) });
    }
}
