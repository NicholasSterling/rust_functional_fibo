#![feature(test)]

extern crate test;

const N: usize = 90;
const EXPECTED: u64 = 2880067194370816120;

pub fn thor314(n: usize) -> u64 {
    let last: u64 = (1..)
        .scan((0u64, 1u64), |state, _| {
            let temp = state.0;
            state.0 = state.1 + state.0;
            state.1 = temp;
            Some(state.0)
        })
        .take(n)
        .last()
        .unwrap();
    last
}

pub fn jethrogb(n: usize) -> u64 {
    let last: u64 = std::iter::repeat_with({
        let mut state = (1u64, 1u64);
        move || {
            let next = (state.1, state.0 + state.1);
            std::mem::replace(&mut state, next).0
        }
    })  .take(n)
        .last()
        .unwrap();
    last
}

pub fn marcianx(n: usize) -> u64 {
    let last: u64 = std::iter::repeat_with({
        let mut state = (0u64, 1u64);
        move || {
            state = (state.1, state.0 + state.1);
            state.0
        }
    })  .take(n)
        .last()
        .unwrap();
    last
}

pub fn zicog1(n: usize) -> u64 {
    let mut state = (0u64, 1u64);
    let last: u64 = std::iter::repeat_with(|| {
        state = (state.1, state.0 + state.1);
        state.0
    })  .take(n)
        .last()
        .unwrap();
    last
}

pub fn fibonacci() -> impl Iterator<Item = u64> {
    let mut state = (0u64, 1u64);
    std::iter::repeat_with(move || {
        state = (state.1, state.0 + state.1);
        state.0
    })
}

pub fn burjui(n: usize) -> u64 {
    let last: u64 = fibonacci()
        .take(n)
        .last()
        .unwrap();
    last
}

////////////

pub fn amigonico(n: usize) -> u64 {
    // Returns an Iterator for the Fibonacci sequence: 1 1 2 3 5 8 ...
    fn fib() -> impl Iterator<Item = u64> {
        iterize((1u64,1u64), |p| (p.1, p.0 + p.1))
    }
    let last: u64 = fib()
        .take(n)
        .last()
        .unwrap();
    last
}

// Produces an Iterator by induction.
// Given an initial state of type (R,S) and a function that produces
// the next state from an existing state, we return an Iterator for the Rs.
// So in (R,S), R is the part that gets (R)eturned by the Iterator,
// and S is any additional (S)tate used internally.
pub fn iterize<R: Copy, S: Copy>(s0: (R,S), f: impl Fn((R,S)) -> (R,S)) -> impl Iterator<Item = R> {
    let mut state = s0;
    std::iter::repeat_with(
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

pub fn itertools(n: usize) -> u64 {
    let last: u64 = itertools::iterate((1u64,1u64), |&p| (p.1, p.0 + p.1))
        .map(|p| p.0)
        .take(n)
        .last()
        .unwrap();
    last
}

pub fn main() {
    let mut last;
    last = thor314(N);
    println!("thor314:  {}", last);
    last = jethrogb(N);
    println!("jethrogb: {}", last);
    last = marcianx(N);
    println!("marcianx: {}", last);
    last = zicog1(N);
    println!("zicog1:   {}", last);
    last = burjui(N);
    println!("burjui:   {}", last);
    last = amigonico(N);
    println!("amigonico:{}", last);
    last = itertools(N);
    println!("itertools:{}", last);
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn thor314_t() {
        assert_eq!(EXPECTED, thor314(N));
    }
    #[test]
    fn jethrogb_t() {
        assert_eq!(EXPECTED, jethrogb(N));
    }
    #[test]
    fn marcianx_t() {
        assert_eq!(EXPECTED, marcianx(N));
    }
    #[test]
    fn zicog1_t() {
        assert_eq!(EXPECTED, zicog1(N));
    }
    #[test]
    fn burjui_t() {
        assert_eq!(EXPECTED, burjui(N));
    }
    #[test]
    fn amigonico_t() {
        assert_eq!(EXPECTED, amigonico(N));
    }

    #[bench]
    fn thor314_b(b: &mut Bencher) {
        b.iter(|| {
            let n = test::black_box(N);
            thor314(n)
        });
    }
    #[bench]
    fn jethrogb_b(b: &mut Bencher) {
        b.iter(|| {
            let n = test::black_box(N);
            jethrogb(n)
        });
    }
    #[bench]
    fn marcianx_b(b: &mut Bencher) {
        b.iter(|| {
            let n = test::black_box(N);
            marcianx(n)
        });
    }
    #[bench]
    fn zicog1_b(b: &mut Bencher) {
        b.iter(|| {
            let n = test::black_box(N);
            zicog1(n)
        });
    }
    #[bench]
    fn burjui_b(b: &mut Bencher) {
        b.iter(|| {
            let n = test::black_box(N);
            burjui(n)
        });
    }
    #[bench]
    fn amigonico_b(b: &mut Bencher) {
        b.iter(|| {
            let n = test::black_box(N);
            amigonico(n)
        });
    }
    #[bench]
    fn itertools_b(b: &mut Bencher) {
        b.iter(|| {
            let n = test::black_box(N);
            itertools(n)
        });
    }
}
