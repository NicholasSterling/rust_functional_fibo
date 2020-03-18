#![feature(test)]

extern crate test;

const N: u64 = 4000000000000000000;

pub fn thor314(n: u64) -> u64 {
    let sum: u64 = (1u64..)
        .scan((1u64, 1u64), |state, _| {
            let temp = state.0;
            state.0 = state.1 + state.0;
            state.1 = temp;
            Some(state.0)
        })
        .take_while(|&x| x < n)
        .filter(|x| x % 2 == 0)
        .sum();
    sum
}

pub fn jethrogb(n: u64) -> u64 {
    let sum: u64 = std::iter::repeat_with({
        let mut state = (1u64, 1u64);
        move || {
            let next = (state.1, state.0 + state.1);
            std::mem::replace(&mut state, next).0
        }
    })
    .take_while(|&x| x < n)
    .filter(|x| x % 2 == 0)
    .sum();
    sum
}

pub fn marcianx(n: u64) -> u64 {
    let sum: u64 = std::iter::repeat_with({
        let mut state = (1u64, 1u64);
        move || {
            state = (state.1, state.0 + state.1);
            state.0
        }
    })
    .take_while(|&x| x < n)
    .filter(|x| x % 2 == 0)
    .sum();
    sum
}

pub fn zicog1(n: u64) -> u64 {
    let mut state = (1u64, 1u64);
    let sum: u64 = std::iter::repeat_with(|| {
        state = (state.1, state.0 + state.1);
        state.0
    })
    .take_while(|&x| x < n)
    .filter(|x| x % 2 == 0)
    .sum();
    sum
}

pub fn zicog2(n: u64) -> u64 {
    let mut sum: u64 = 0;
    let mut state = (1u64, 1u64);
    loop {
        state = (state.1, state.0 + state.1);
        if state.0 >= n {
            break sum;
        }
        if state.0 % 2 == 0 {
            sum += state.0
        }
    }
}

pub fn exphp(n: u64) -> u64 {
    let sum = {
        let mut sum: u64 = 0;
        let mut state = (1u64, 1u64);
        loop {
            state = (state.1, state.0 + state.1);
            if state.0 >= n {
                break sum;
            }
            if state.0 % 2 == 0 {
                sum += state.0
            }
        }
    };
    sum
}

pub fn fibonacci() -> impl Iterator<Item = u64> {
    let mut state = (0u64, 1u64);
    std::iter::repeat_with(move || {
        state = (state.1, state.0 + state.1);
        state.0
    })
}

pub fn burjui(n: u64) -> u64 {
    let sum: u64 = fibonacci()
        .take_while(|&x| x < n)
        .filter(|x| x % 2 == 0)
        .sum();
    sum
}

////////////

pub fn amigonico(n: u64) -> u64 {
    // Returns an Iterator for the Fibonacci sequence: 1 1 2 3 5 8 ...
    fn fib() -> impl Iterator<Item = u64> {
        iterize((1u64,1u64), |p| (p.1, p.0 + p.1))
    }
    let sum: u64 = fib()
        .take_while(|&x| x < n)
        .filter(|x| x % 2 == 0)
        .sum();
    sum
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

pub fn itertools(n: u64) -> u64 {
    let sum: u64 = itertools::iterate((1u64,1u64), |&p| (p.1, p.0 + p.1))
        .map(|p| p.0)
        .take_while(|&x| x < n)
        .filter(|x| x % 2 == 0)
        .sum();
    sum
}

pub fn main() {
    let mut sum;
    sum = thor314(N);
    println!("thor314:  {}", sum);
    sum = jethrogb(N);
    println!("jethrogb: {}", sum);
    sum = marcianx(N);
    println!("marcianx: {}", sum);
    sum = zicog1(N);
    println!("zicog1:   {}", sum);
    sum = zicog2(N);
    println!("zicog2:   {}", sum);
    sum = exphp(N);
    println!("exphp:    {}", sum);
    sum = burjui(N);
    println!("burjui:   {}", sum);
    sum = amigonico(N);
    println!("amigonico:{}", sum);
    sum = itertools(N);
    println!("itertools:{}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    const EXPECTED: u64 = 3770056902373173214;

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
    fn zicog2_t() {
        assert_eq!(EXPECTED, zicog2(N));
    }
    #[test]
    fn exphp_t() {
        assert_eq!(EXPECTED, exphp(N));
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
    fn zicog2_b(b: &mut Bencher) {
        b.iter(|| {
            let n = test::black_box(N);
            zicog2(n)
        });
    }
    #[bench]
    fn exphp_b(b: &mut Bencher) {
        b.iter(|| {
            let n = test::black_box(N);
            exphp(n)
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
