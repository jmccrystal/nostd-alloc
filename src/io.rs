use core::ffi::c_void;
use core::fmt;
use core::any::TypeId;
use core::convert::Into;
use core::ops::{Div, DivAssign, Mul, MulAssign, Rem, Shl, Sub};
use core::str::from_utf8_unchecked;

pub fn print(msg: impl Printable) {
    msg.print();
}

pub fn println(msg: impl Printable) {
    print(msg);
    print("\n");
}

extern "C" {
    fn write(fd: i32, buf: *const c_void, count: usize) -> isize;
}

fn rust_write(buf: &[u8]) -> isize {
    // Write to 
    unsafe { write(1, buf.as_ptr().cast::<c_void>(), buf.len()) }
}

pub trait Printable {
    fn print(&self);
    fn println(&self) {
        self.print();
        "\n".print();
    }
}

impl Printable for &str {
    fn print(&self) {
        rust_write(self.as_ref());
    }
}

impl<T: Printable + Clone> Printable for &[T] {
    fn print(&self) {
        for element in *self {
            print(element.clone());
        }
    }
}

impl<T: Integer> Printable for T {
    fn print(&self) {
        let mut number = *self;

        let zero = T::try_from(0u8).unwrap();
        let ten = T::try_from(10u8).unwrap();


        let min = T::min_value();
        // Stupid workaround for Self::MIN overflows
        if number == min {
            // Print number with last digit truncated
            print(min / ten);
            // Print last digit
            print(Self::abs(min % ten));
            return
        }
        // We can convert 0 into any integer type, so unwrap is safe
        if number < zero {
            // Number must be a signed integer
            print("-");
            number = T::abs(number);
        }
        get_next_digit(number.as_unsigned());
    }
}

fn print_digit(digit: u8) {
    // add ASCII offset, convert to single item array, then print
    unsafe { print(from_utf8_unchecked(&[digit + 48])) }
}

fn get_next_digit<T: UInteger>(mut number: T) {
    // If number is less than 10, it must be the final digit, so we can print right away
    if number < 10.into() {
        // Convert number to u8 and print
        // Unwrap is safe since digit is less than 10
        print_digit(number.try_into().unwrap());
        return
    }
    let digit_count = count_digits(number);
    let ten: T = 10.into();
    // Find the power of 10 of the same magnitude as the number, e.g. 10000 for 98765
    let pow = ten.pow(digit_count - 1);

    // Find the most significant digit
    let digit = number / pow;

    // Subtract most significant digit, stripping it from the original number
    number = number - digit * pow;
    
    // Print most significant digit
    print_digit(digit.try_into().unwrap());

    // Count digits a second time
    let diff = digit_count - count_digits(number) - 1;

    // print the amount of zeroes that it is different
    for _ in 0..diff {
        print_digit(0);
    }
    
    // Recursively call this function to get the remaining digits
    get_next_digit(number);
}

/// Counts the digits in an integer.
pub fn count_digits<T: UInteger>(mut number: T) -> u32 {
    // Edge case of 0
    if number == 0.into() {
        return 1
    }
    let mut digit_count = 0;
    // Repetitively divide by 10 until number reaches 0 by truncation
    while number > 0.into() {
        number /= 10.into();
        digit_count += 1;
    }
    digit_count
}

pub trait Integer:
// To ensure this trait only contains integer types,
// we must restrict what can be considered an `Integer`
TryFrom<u8, Error: fmt::Debug> +
TryFrom<usize, Error: fmt::Debug> +
Eq + // Eq is implemented for integers but not floats
Ord + // Can compare two integers
Copy + // No need to clone
DivAssign<Self> + // Must DivAssign in count_digits
MulAssign<Self> +
TryInto<Self::Unsigned, Error: fmt::Debug> + // TryInto ONLY the type's unsigned counterpart, ensuring efficiency
Div<Output = Self> + // division
Mul<Output = Self> + // multiplication
Sub<Output = Self> + // subtraction
Rem<Output = Self> +
Shl<Output = Self> where Self: 'static
{
    type Unsigned: UInteger;

    // Will panic if number is negative. Ensure number is greater than 0 before calling
    fn as_unsigned(self) -> Self::Unsigned
    where Self::Unsigned: UInteger {
        self.try_into().unwrap()
    }
    // Panics if Self::MIN is passed in
    fn abs(self) -> Self {
        if self < Self::try_from(0u8).unwrap() {
            // Subtract self from 0 to flip sign
            Self::try_from(0u8).unwrap() - self
        }
        else { self }
    }
    fn min_value() -> Self {
        let bytes = size_of::<Self>();
        // Size of type in bits will never exceed limit of type
        let bits = Self::try_from(bytes * 8).unwrap();
        
        // Check for unsigned int
        if TypeId::of::<Self>() == TypeId::of::<Self::Unsigned>() {
            // Unsigned type, minimum is 0
            Self::try_from(0u8).unwrap()
        } else {
            let one = Self::try_from(1u8).unwrap();
            // Bit shifts 1 to the sign bit.
            // e.g. 00000001 -> 10000000 = -128
            one << (bits - one)
        }
    }
}

pub trait UInteger:
Integer +
TryInto<u8, Error: fmt::Debug> +
From<u8> + // Can be converted to from the smallest integer type
{
    fn pow(self, exp: u32) -> Self {
        if exp == 0 {
            return 1.into()
        }
        let mut output = self;
        for _ in 0..exp - 1 {
            output *= self;
        }
        output
    }
}

impl Integer for u8 {
    type Unsigned = Self;
}
impl Integer for u16 {
    type Unsigned = Self;
}
impl Integer for u32 { 
    type Unsigned = Self;
}
impl Integer for u64 { 
    type Unsigned = Self;
}
impl Integer for u128 {
    type Unsigned = Self;
}
impl Integer for usize {
    type Unsigned = Self;
}
impl Integer for i8 {
    type Unsigned = u8;
}
impl Integer for i16 {
    type Unsigned = u16;
}
impl Integer for i32 {
    type Unsigned = u32;
}
impl Integer for i64 {
    type Unsigned = u64;
}
impl Integer for i128 {
    type Unsigned = u128;
}
impl Integer for isize {
    type Unsigned = usize;
}
impl UInteger for u8 {}
impl UInteger for u16 {}
impl UInteger for u32 {}
impl UInteger for u64 {}
impl UInteger for u128 {}
impl UInteger for usize {}