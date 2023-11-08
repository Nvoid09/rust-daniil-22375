#[derive(Debug)]
pub struct Complex <T> {
    re: T,
    im: T
}

use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;
use std::ops::AddAssign;
use std::ops::SubAssign;
use std::ops::MulAssign;
use std::ops::DivAssign;

impl<T: Add<Output=T>> Add for Complex<T> {
    type Output = Complex<T>;
 
    fn add(self, other: Self) -> Self::Output {
        Complex {
            re: self.re + other.re, 
            im: self.im + other.im,
        }
    }
}

impl<T: Add<Output=T> + Copy> AddAssign for Complex<T> { 
    fn add_assign(&mut self, other: Self) {
        self.re = self.re + other.re;
        self.im =self.im + other.im;
    }
}

impl<T: Sub<Output=T>> Sub for Complex<T> {
    type Output = Complex<T>;
 
    fn sub(self, other: Self) -> Self::Output {
        Complex {
            re: self.re - other.re, 
            im: self.im - other.im,
        }
    }
}
impl<T: Sub<Output=T> + Copy> SubAssign for Complex<T> { 
    fn sub_assign(&mut self, other: Self) {
        self.re = self.re - other.re;
        self.im = self.im - other.im;
    }
}

impl<T: Mul<Output=T> + Add<Output=T> + Sub<Output=T> + Copy> Mul for Complex<T> {
    type Output = Complex<T>;
    fn mul(self, other: Self) -> Self::Output {
        Complex {
            re: self.re * other.re - self.im * other.im, 
            im: self.re * other.im + self.im * other.re,
        }
    }
}
impl<T: Mul<Output=T> + Add<Output=T> + Sub<Output=T> + Copy> MulAssign for Complex<T> { 
    fn mul_assign(&mut self, other: Self) {
        let tmp_re = self.re * other.re - self.im * other.im;
        let tmp_im = self.re * other.im + self.im * other.re;
        self.re = tmp_re;
        self.im = tmp_im;
    }
}

impl<T: Mul<Output=T> + Add<Output=T> + Sub<Output=T> + Div<Output=T> + Copy> Div for Complex<T> {
    type Output = Complex<T>;
 
    fn div(self, other: Self) -> Self::Output {
        let modul = other.re * other.re + other.im * other.im;
        Complex {
            re: (self.re * other.re + self.im * other.im) / modul, 
            im: (self.im * other.re - self.re * other.im) / modul,
        }
    }
}

impl<T: Mul<Output=T> + Add<Output=T> + Sub<Output=T> + Div<Output=T> + Copy> DivAssign for Complex<T> { 
    fn div_assign(&mut self, other: Self) {
        let modul = other.re * other.re + other.im * other.im;
        let tmp_re = (self.re * other.re + self.im * other.im) / modul;
        let tmp_im = (self.im * other.re - self.re * other.im) / modul;
        self.re = tmp_re;
        self.im = tmp_im;
    }
}

#[test]
fn test_add() {
	let x1 : Complex<i32> = Complex { re: 1, im: 2 };
    let x2 : Complex<i32> = Complex { re: 2, im: 3 };
    let x3 : Complex<i32> = Complex { re: 3, im: 5 };
    let mut x4 = x1 + x2;
	assert_eq!(x4.re, x3.re);
	assert_eq!(x4.im, x3.im);
    x4 += x3;
	assert_eq!(x4.re, 6);
	assert_eq!(x4.im, 10);
}
#[test]
fn test_sub() {
    let x1 : Complex<i32> = Complex { re: 2, im: 3 };
    let x2 : Complex<i32> = Complex { re: 1, im: 2 };
    let x3 : Complex<i32> = Complex { re: 1, im: 1 };
    let mut x4 = x1 - x2;
	assert_eq!(x4.re, x3.re);
	assert_eq!(x4.im, x3.im);
    x4 -= x3;
	assert_eq!(x4.re, 0);
	assert_eq!(x4.im, 0);
}
#[test]
fn test_mul() {
    let x1 : Complex<i32> = Complex { re: 2, im: 3 };
    let x2 : Complex<i32> = Complex { re: 1, im: 2 };
    let x3 : Complex<i32> = Complex { re: -4, im: 7 };
    let mut x4 = x1 * x2;
	assert_eq!(x4.re, x3.re);
	assert_eq!(x4.im, x3.im);
    x4 *= x3;
	assert_eq!(x4.re, -33);
	assert_eq!(x4.im, -56);
}
#[test]
fn test_div() {
    let x1 : Complex<f32> = Complex { re: 2.0, im: 3.0 };
    let x2 : Complex<f32> = Complex { re: 1.0, im: 2.0 };
    let x3 : Complex<f32> = Complex { re: 1.6, im: -0.2 };
    let mut x4 = x1 / x2;
	assert_eq!(x4.re, x3.re);
	assert_eq!(x4.im, x3.im);
    x4 /= x3;
    println!("{}, {}", x4.re, x4.im);
	assert_eq!(x4.re, 1.0);
	assert_eq!(x4.im, 0.0);
}
