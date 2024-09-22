pub trait Gcd {
    fn gcd(&self, other: &Self) -> Self;
}

macro_rules! impl_integer_for_isize {
    ($T:ty) => {
        impl Gcd for $T {
            fn gcd(&self, other: &Self) -> Self {
                let mut m = *self;
                let mut n = *other;
                if m == 0 || n == 0 {
                    return (m | n).abs();
                }
                let shift = (m | n).trailing_zeros();
                if m == Self::MIN || n == Self::MIN {
                    return 1 << shift;
                }
                m = m.abs();
                n = n.abs();
                m >>= m.trailing_zeros();
                n >>= n.trailing_zeros();
                while m != n {
                    if m > n {
                        m -= n;
                        m >>= m.trailing_zeros();
                    } else {
                        n -= m;
                        n >>= n.trailing_zeros();
                    }
                }
                m << shift
            }
        }
    };
}

impl_integer_for_isize!(i8);
impl_integer_for_isize!(i16);
impl_integer_for_isize!(i32);
impl_integer_for_isize!(i64);
impl_integer_for_isize!(i128);
impl_integer_for_isize!(isize);
