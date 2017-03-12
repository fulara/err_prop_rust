use std::ops::*;

extern crate cgmath;

use cgmath::num_traits::*;
use cgmath::*;

use std::num::FpCategory;


#[derive(Copy, Clone, PartialEq, Debug)]
pub struct F64Err {
    val: f64,
    err: f64,
}

impl F64Err {
    #[inline]
    pub fn new_errorfree(val: f64) -> F64Err {
        F64Err {
            val: val,
            err: 0.
        }
    }

    #[inline]
    pub fn new(val: f64) -> F64Err {
        F64Err {
            val: val,
            err: val
        }
    }

    #[inline]
    pub fn new_exact(val: f64, err: f64) -> F64Err {
        F64Err {
            val: val,
            err: err
        }
    }

    #[inline]
    pub fn val(&self) -> f64 {
        self.val
    }

    #[inline]
    pub fn err(&self) -> f64 {
        self.err
    }

    #[inline]
    pub fn err_times_eps(&self) -> f64 {
        self.err * ::std::f64::EPSILON
    }
}

impl Mul for F64Err {
    type Output = F64Err;
    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        F64Err {
            val: self.val * rhs.val,
            err: self.val.abs() * rhs.err + rhs.val.abs() * self.err
        }
    }
}

impl MulAssign for F64Err {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs
    }
}

impl Div for F64Err {
    type Output = F64Err;
    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        F64Err {
            val: self.val / rhs.val,
            err: self.err / rhs.val.abs() + rhs.err * self.val / (rhs.val * rhs.val)
        }
    }
}

impl DivAssign for F64Err {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs
    }
}

impl Add for F64Err {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        F64Err {
            val: self.val + rhs.val,
            err: self.val.abs().max(rhs.val.abs()) + self.err + rhs.err
        }
    }
}

impl AddAssign for F64Err {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl Sub for F64Err {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        F64Err {
            val: self.val - rhs.val,
            err: self.val.abs().max(rhs.val.abs()) + self.err + rhs.err
        }
    }
}

impl SubAssign for F64Err {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}

impl Rem for F64Err {
    type Output = Self;
    #[inline]
    fn rem(self, rhs: Self) -> Self::Output {
        unimplemented!();
    }
}

impl RemAssign for F64Err {
    #[inline]
    fn rem_assign(&mut self, rhs: Self) {
        *self = *self % rhs
    }
}

impl PartialOrd for F64Err {
    #[inline]
    fn partial_min(self, other: Self) -> Self {
        if self.val < other.val {
            self
        } else {
            other
        }
    }
    #[inline]
    fn partial_max(self, other: Self) -> Self {
        if self.val > other.val {
            self
        } else {
            other
        }
    }
}

impl ::std::cmp::PartialOrd for F64Err {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<::std::cmp::Ordering> {
        self.val.partial_cmp(&other.val)
    }
}

impl Zero for F64Err {
    #[inline]
    fn zero() -> Self {
        Self::new_errorfree(0.)
    }

    #[inline]
    fn is_zero(&self) -> bool {
        self.val.is_zero()
    }
}

impl One for F64Err {
    #[inline]
    fn one() -> Self {
        Self::new_errorfree(1.)
    }
}

impl ToPrimitive for F64Err {
    /// Converts the value of `self` to an `isize`.
    #[inline]
    fn to_isize(&self) -> Option<isize> {
        self.val.to_isize()
    }

    /// Converts the value of `self` to an `i8`.
    #[inline]
    fn to_i8(&self) -> Option<i8> {
        self.val.to_i8()
    }

    /// Converts the value of `self` to an `i16`.
    #[inline]
    fn to_i16(&self) -> Option<i16> {
        self.val.to_i16()
    }

    /// Converts the value of `self` to an `i32`.
    #[inline]
    fn to_i32(&self) -> Option<i32> {
        self.val.to_i32()
    }

    #[inline]
    fn to_i64(&self) -> Option<i64> {
        self.val.to_i64()
    }

    /// Converts the value of `self` to a `usize`.
    #[inline]
    fn to_usize(&self) -> Option<usize> {
        self.val.to_usize()
    }

    /// Converts the value of `self` to an `u8`.
    #[inline]
    fn to_u8(&self) -> Option<u8> {
        self.val.to_u8()
    }

    /// Converts the value of `self` to an `u16`.
    #[inline]
    fn to_u16(&self) -> Option<u16> {
        self.val.to_u16()
    }

    /// Converts the value of `self` to an `u32`.
    #[inline]
    fn to_u32(&self) -> Option<u32> {
        self.val.to_u32()
    }

    /// Converts the value of `self` to an `u32`.
    #[inline]
    fn to_u64(&self) -> Option<u64> {
        self.val.to_u64()
    }

    /// Converts the value of `self` to an `f32`.
    #[inline]
    fn to_f32(&self) -> Option<f32> {
        self.val.to_f32()
    }

    /// Converts the value of `self` to an `f64`.
    #[inline]
    fn to_f64(&self) -> Option<f64> {
        self.val.to_f64()
    }
}

impl NumCast for F64Err {
    #[inline]
    fn from<T>(t: T) -> Option<Self> {
        unimplemented!();
    }
}

impl Num for F64Err {
    type FromStrRadixErr = ParseFloatError;
    #[inline]
    fn from_str_radix(src: &str, radix: u32) -> Result<Self, ParseFloatError> {
        unimplemented!();
    }
}

impl BaseNum for F64Err {}

impl ApproxEq for F64Err {
    type Epsilon = Self;
    #[inline]
    fn default_epsilon() -> Self::Epsilon {
        unimplemented!()
    }

    #[inline]
    fn default_max_relative() -> Self::Epsilon {
        unimplemented!()
    }

    #[inline]
    fn default_max_ulps() -> u32 {
        unimplemented!()
    }

    #[inline]
    fn relative_eq(&self, other: &Self, epsilon: Self::Epsilon, max_relative: Self::Epsilon) -> bool {
        unimplemented!()
    }

    #[inline]
    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        unimplemented!()
    }
}

impl Neg for F64Err {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self::Output {
        unimplemented!()
    }
}

impl Float for F64Err {
    #[inline]
    fn nan() -> Self {
        unimplemented!()
    }

    #[inline]
    fn infinity() -> Self {
        unimplemented!()
    }

    #[inline]
    fn neg_infinity() -> Self {
        unimplemented!()
    }

    #[inline]
    fn neg_zero() -> Self {
        unimplemented!()
    }

    #[inline]
    fn min_value() -> Self {
        unimplemented!()
    }

    #[inline]
    fn min_positive_value() -> Self {
        unimplemented!()
    }

    #[inline]
    fn max_value() -> Self {
        unimplemented!()
    }

    #[inline]
    fn is_nan(self) -> bool {
        unimplemented!()
    }

    #[inline]
    fn is_infinite(self) -> bool {
        unimplemented!()
    }

    #[inline]
    fn is_finite(self) -> bool {
        unimplemented!()
    }

    #[inline]
    fn is_normal(self) -> bool {
        unimplemented!()
    }

    #[inline]
    fn classify(self) -> FpCategory {
        unimplemented!()
    }

    #[inline]
    fn floor(self) -> Self {
        unimplemented!()
    }

    #[inline]
    fn ceil(self) -> Self {
        unimplemented!()
    }

    #[inline]
    fn round(self) -> Self {
        unimplemented!()
    }

    #[inline]
    fn trunc(self) -> Self {
        unimplemented!()
    }

    #[inline]
    fn fract(self) -> Self {
        unimplemented!()
    }

    #[inline]
    fn abs(self) -> Self {
        unimplemented!()
    }

    #[inline]
    fn signum(self) -> Self {
        unimplemented!()
    }

    #[inline]
    fn is_sign_positive(self) -> bool {
        unimplemented!()
    }

    #[inline]
    fn is_sign_negative(self) -> bool {
        unimplemented!()
    }

    #[inline]
    fn mul_add(self, a: Self, b: Self) -> Self {
        unimplemented!()
    }

    #[inline]
    fn recip(self) -> Self {
        unimplemented!()
    }

    #[inline]
    fn powi(self, n: i32) -> Self {
        unimplemented!()
    }

    #[inline]
    fn powf(self, n: Self) -> Self {
        unimplemented!()
    }

    #[inline]
    fn sqrt(self) -> Self {
        unimplemented!()
    }

    #[inline]
    fn exp(self) -> Self {
        unimplemented!()
    }

    #[inline]
    fn exp2(self) -> Self {
        unimplemented!()
    }

    #[inline]
    fn ln(self) -> Self {
        unimplemented!()
    }

    #[inline]
    fn log(self, base: Self) -> Self {
        unimplemented!()
    }

    #[inline]
    fn log2(self) -> Self {
        unimplemented!()
    }

    #[inline]
    fn log10(self) -> Self {
        unimplemented!()
    }

    #[inline]
    fn max(self, other: Self) -> Self {
        unimplemented!()
    }

    #[inline]
    fn min(self, other: Self) -> Self {
        unimplemented!()
    }

    #[inline]
    fn abs_sub(self, other: Self) -> Self {
        unimplemented!()
    }

    #[inline]
    fn cbrt(self) -> Self {
        unimplemented!()
    }

    #[inline]
    fn hypot(self, other: Self) -> Self {
        unimplemented!()
    }

    #[inline]
    fn sin(self) -> Self {
        unimplemented!()
    }

    #[inline]
    fn cos(self) -> Self {
        unimplemented!()
    }

    #[inline]
    fn tan(self) -> Self {
        unimplemented!()
    }

    #[inline]
    fn asin(self) -> Self {
        unimplemented!()
    }

    #[inline]
    fn acos(self) -> Self {
        unimplemented!()
    }

    #[inline]
    fn atan(self) -> Self {
        unimplemented!()
    }

    #[inline]
    fn atan2(self, other: Self) -> Self {
        unimplemented!()
    }

    #[inline]
    fn sin_cos(self) -> (Self, Self) {
        unimplemented!()
    }

    #[inline]
    fn exp_m1(self) -> Self {
        unimplemented!()
    }

    #[inline]
    fn ln_1p(self) -> Self {
        unimplemented!()
    }

    #[inline]
    fn sinh(self) -> Self {
        unimplemented!()
    }

    #[inline]
    fn cosh(self) -> Self {
        unimplemented!()
    }

    #[inline]
    fn tanh(self) -> Self {
        unimplemented!()
    }

    #[inline]
    fn asinh(self) -> Self {
        unimplemented!()
    }

    #[inline]
    fn acosh(self) -> Self {
        unimplemented!()
    }

    #[inline]
    fn atanh(self) -> Self {
        unimplemented!()
    }

    #[inline]
    fn integer_decode(self) -> (u64, i16, i8) {
        unimplemented!()
    }
}

impl BaseFloat for F64Err {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiplication_error() {
        let left = F64Err::new_errorfree(2.);
        let right = F64Err::new_errorfree(2.);

        let res = left * right;

        assert_eq!(4., res.val());
        assert_eq!(0., res.err());
    }

    #[test]
    fn addition_error() {
        let left = F64Err::new_errorfree(2.);
        let right = F64Err::new_errorfree(3.);

        let res = left + right;

        assert_eq!(5., res.val());
        assert_eq!(3., res.err());
    }

    #[test]
    fn addition_then_multipl() {
        let left = F64Err::new_errorfree(2.);
        let right = F64Err::new_errorfree(3.);

        let res_add = left + right;
        let res_mul = res_add * left;

        assert_eq!(10., res_mul.val());
        assert_eq!(3. * 2., res_mul.err());

        let res_mul = res_mul * res_mul;

        assert_eq!(100., res_mul.val());
        assert_eq!((3. * 2.) * 10. + (3. * 2.) * 10., res_mul.err());
    }

    #[test]
    fn subtract_error() {
        let left = F64Err::new_errorfree(2.);
        let right = F64Err::new_errorfree(3.);

        let res = left - right;

        assert_eq!(-1., res.val());
        assert_eq!(3., res.err());

        let res = res - right;

        assert_eq!(-4., res.val());
        assert_eq!(6., res.err());
    }

    #[test]
    fn can_use_cgmath_stuff() {
        type Point2 = ::cgmath::Point2<F64Err>;

        let a = Point2::new(F64Err::new(1.), F64Err::new(0.));
        let b = Point2::new(F64Err::new(1.), F64Err::new(0.));

        let det = ::cgmath::Matrix2::new(a.x, a.y,
                                         b.x, b.y).determinant();
    }
}
