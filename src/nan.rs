// use std::hash::Hash;
// use bevy_reflect::{Reflect, impl_reflect_value};
// use num_traits::float::FloatCore as Float;
// use ordered_float::FloatIsNan;

// #[derive(Debug, Serialize, Deserialize, Default, PartialOrd, PartialEq, Eq, Clone, Copy)]
// pub struct NotNan<T: Float + Reflect>(ordered_float::NotNan<T>);

// impl_reflect_value!(NotNan<T: Float + Reflect>);

// impl<T: Float + Reflect> NotNan<T> {
// 	#[inline]
// 	pub fn new(val: T) -> Result<Self, FloatIsNan> {
//         ordered_float::NotNan::new(val).map(|r| {
// 			Self(r)
// 		})
//     }

// 	#[inline]
//     pub const unsafe fn new_unchecked(val: T) -> Self {
//         Self(ordered_float::NotNan::new_unchecked(val))
//     }
// }

// impl<T: Float + Reflect> Hash for NotNan<T> {
//     fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
//         self.0.hash(state);
//     }
// }

// impl From<NotNan<f32>> for NotNan<f64> {
//     #[inline]
//     fn from(v: NotNan<f32>) -> NotNan<f64> {
//         unsafe { NotNan::new_unchecked(*(v.0) as f64) }
//     }
// }

// impl<T: Float> Deref for NotNan<T> {
//     type Target = T;

//     #[inline]
//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

// impl<T: Float> PartialEq<T> for NotNan<T> {
//     #[inline]
//     fn eq(&self, other: &T) -> bool {
//         self.0 == *other
//     }
// }

// /// Adds a float directly.
// ///
// /// Panics if the provided value is NaN or the computation results in NaN
// impl<T: Float> Add<T> for NotNan<T> {
//     type Output = Self;

//     #[inline]
//     fn add(self, other: T) -> Self {
//         NotNan::new(self.0 + other).expect("Addition resulted in NaN")
//     }
// }

// /// Adds a float directly.
// ///
// /// Panics if the provided value is NaN.
// impl<T: Float + Sum> Sum for NotNan<T> {
//     fn sum<I: Iterator<Item = NotNan<T>>>(iter: I) -> Self {
//         NotNan::new(iter.map(|v| v.0).sum()).expect("Sum resulted in NaN")
//     }
// }

// impl<'a, T: Float + Sum + 'a> Sum<&'a NotNan<T>> for NotNan<T> {
//     #[inline]
//     fn sum<I: Iterator<Item = &'a NotNan<T>>>(iter: I) -> Self {
//         iter.cloned().sum()
//     }
// }

// /// Subtracts a float directly.
// ///
// /// Panics if the provided value is NaN or the computation results in NaN
// impl<T: Float> Sub<T> for NotNan<T> {
//     type Output = Self;

//     #[inline]
//     fn sub(self, other: T) -> Self {
//         NotNan::new(self.0 - other).expect("Subtraction resulted in NaN")
//     }
// }

// /// Multiplies a float directly.
// ///
// /// Panics if the provided value is NaN or the computation results in NaN
// impl<T: Float> Mul<T> for NotNan<T> {
//     type Output = Self;

//     #[inline]
//     fn mul(self, other: T) -> Self {
//         NotNan::new(self.0 * other).expect("Multiplication resulted in NaN")
//     }
// }

// impl<T: Float + Product> Product for NotNan<T> {
//     fn product<I: Iterator<Item = NotNan<T>>>(iter: I) -> Self {
//         NotNan::new(iter.map(|v| v.0).product()).expect("Product resulted in NaN")
//     }
// }

// impl<'a, T: Float + Product + 'a> Product<&'a NotNan<T>> for NotNan<T> {
//     #[inline]
//     fn product<I: Iterator<Item = &'a NotNan<T>>>(iter: I) -> Self {
//         iter.cloned().product()
//     }
// }

// /// Divides a float directly.
// ///
// /// Panics if the provided value is NaN or the computation results in NaN
// impl<T: Float> Div<T> for NotNan<T> {
//     type Output = Self;

//     #[inline]
//     fn div(self, other: T) -> Self {
//         NotNan::new(self.0 / other).expect("Division resulted in NaN")
//     }
// }

// /// Calculates `%` with a float directly.
// ///
// /// Panics if the provided value is NaN or the computation results in NaN
// impl<T: Float> Rem<T> for NotNan<T> {
//     type Output = Self;

//     #[inline]
//     fn rem(self, other: T) -> Self {
//         NotNan::new(self.0 % other).expect("Rem resulted in NaN")
//     }
// }

// macro_rules! impl_not_nan_binop {
//     ($imp:ident, $method:ident, $assign_imp:ident, $assign_method:ident) => {
//         impl<T: Float> $imp for NotNan<T> {
//             type Output = Self;

//             #[inline]
//             fn $method(self, other: Self) -> Self {
//                 self.$method(other.0)
//             }
//         }

//         impl<T: Float> $imp<&T> for NotNan<T> {
//             type Output = NotNan<T>;

//             #[inline]
//             fn $method(self, other: &T) -> Self::Output {
//                 self.$method(*other)
//             }
//         }

//         impl<T: Float> $imp<&Self> for NotNan<T> {
//             type Output = NotNan<T>;

//             #[inline]
//             fn $method(self, other: &Self) -> Self::Output {
//                 self.$method(other.0)
//             }
//         }

//         impl<T: Float> $imp for &NotNan<T> {
//             type Output = NotNan<T>;

//             #[inline]
//             fn $method(self, other: Self) -> Self::Output {
//                 (*self).$method(other.0)
//             }
//         }

//         impl<T: Float> $imp<NotNan<T>> for &NotNan<T> {
//             type Output = NotNan<T>;

//             #[inline]
//             fn $method(self, other: NotNan<T>) -> Self::Output {
//                 (*self).$method(other.0)
//             }
//         }

//         impl<T: Float> $imp<T> for &NotNan<T> {
//             type Output = NotNan<T>;

//             #[inline]
//             fn $method(self, other: T) -> Self::Output {
//                 (*self).$method(other)
//             }
//         }

//         impl<T: Float> $imp<&T> for &NotNan<T> {
//             type Output = NotNan<T>;

//             #[inline]
//             fn $method(self, other: &T) -> Self::Output {
//                 (*self).$method(*other)
//             }
//         }

//         impl<T: Float + $assign_imp> $assign_imp<T> for NotNan<T> {
//             #[inline]
//             fn $assign_method(&mut self, other: T) {
//                 *self = (*self).$method(other);
//             }
//         }

//         impl<T: Float + $assign_imp> $assign_imp<&T> for NotNan<T> {
//             #[inline]
//             fn $assign_method(&mut self, other: &T) {
//                 *self = (*self).$method(*other);
//             }
//         }

//         impl<T: Float + $assign_imp> $assign_imp for NotNan<T> {
//             #[inline]
//             fn $assign_method(&mut self, other: Self) {
//                 (*self).$assign_method(other.0);
//             }
//         }

//         impl<T: Float + $assign_imp> $assign_imp<&Self> for NotNan<T> {
//             #[inline]
//             fn $assign_method(&mut self, other: &Self) {
//                 (*self).$assign_method(other.0);
//             }
//         }
//     };
// }

// impl_not_nan_binop! {Add, add, AddAssign, add_assign}
// impl_not_nan_binop! {Sub, sub, SubAssign, sub_assign}
// impl_not_nan_binop! {Mul, mul, MulAssign, mul_assign}
// impl_not_nan_binop! {Div, div, DivAssign, div_assign}
// impl_not_nan_binop! {Rem, rem, RemAssign, rem_assign}

// impl<T: Float> Neg for NotNan<T> {
//     type Output = Self;

//     #[inline]
//     fn neg(self) -> Self {
//         NotNan(-self.0)
//     }
// }

// impl<T: Float> Neg for &NotNan<T> {
//     type Output = NotNan<T>;

//     #[inline]
//     fn neg(self) -> Self::Output {
//         NotNan(-self.0)
//     }
// }