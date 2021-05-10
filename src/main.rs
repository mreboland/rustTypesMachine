fn main() {
    println!("Hello, world!");
}

// Machine Types

// Rust's type system is a collection of fixed-width numeric types, chosen to match the types that almost all modern processors implement directly in hardware, and the Boolean and character types.

// Size (bits) Unsigned integer Signed integer Floating-point
// 8           u8       i8
// 16          u16      i16
// 32          u32      i32                    f32
// 64          u64      i64                    f64
// Machine word usize            isize

// A machine word is a value the size of an address on the machine the code runs on, usually 32 or 64 bits.

// Integer Types

// Rust's unsigned integer types use their full range to represent positive value and zero:

// Type    Range
// u8      0 to 2^8 –1 (0 to 255)
// u16     0 to 2*16 −1 (0 to 65,535)
// u32     0 to 2^32 −1 (0 to 4,294,967,295)
// u64     0 to 2^64 −1 (0 to 18,446,744,073,709,551,615, or 18 quintillion)
// usize   0 to either 2^32 −1 or 2^64 −1

// Rust's signed integer types use the two's complement representation, using the same bit patterns as the corresponding unsigned type to cover a range of positive and negative values:

// Type    Range
// i8      −2^7 to 2^7 −1 (−128 to 127)
// i16     −2^15 to 2^15 −1 (−32,768 to 32,767)
// i32     −2^31 to 2^31 −1 (−2,147,483,648 to 2,147,483,647)
// i64     −2^63 to 2^63 −1 (−9,223,372,036,854,775,808 to 9,223,372,036,854,775,807)
// isize   Either −2^31 to 2^31 −1, or −2^63 to 2^63 −1

// Rust generally uses the u8 type for byte values. For example, reading data from a file or socket yields a stream of u8 values.

// Rust treats characters as distinct from the numeric types. It's neither a u8 or an i8. Discussed more in Characters a little later on.

// Rust requires array indices to be usize values. Values representing the sizes of arrays or vectors or counts of the number of elements in some data structure also generally have the usize type.

// In debug builds, Rust checks for integer overflow in arithmetic:
// let big_val = std::i32::MAX;
// let x = big_val + 1; // panic: arithmetic operation overflowed

// To avoid the above overflow (in release build, Rust would wrap to a negative number) we'd want to use wrapping arithmetic:
// let x = big_val.wrapping_add(1); // ok

// Integer literals can take a suffix indicating their type: 42u8 is a u8 value, and 1729isize is an isize. We can omit the suffix and Rust will try to infer a type for it from the context. Rust defaults to i32 if possible, however if nothing can be inferred, Rust reports the ambiguity as an error.

// The prefixes 0x, 0o, and 0b designate hexadecimal, octal, and binary literals.

// To make long numbers more legible we can use underscores where we'd normally use commas in written form.

// Examples of integer literals:
// Literal     Type        Decimal value
// 116i8       i8          116
// 0xcafeu32   u32         51966
// 0b0010_1010 Inferred    42
// 0o106       Inferred    70

// Although numeric types and the char type are distinct, Rust does provide byte literals, character-like literals for u8 values: b'X' represents the ASCII code for the character X, as a u8 value. For example, since the ASCII code for A is 65, the literals b'A' and 65u8 are exactly equivalent. Only ASCII characters may appear in byte literals.

// There are a few characters that can't be placed after the single quote. Those that can't use the single quote need a backslash placed in front of them:

// Character           Byte literal    Numeric equivalent
// Single quote,       ' b'\''         39u8
// Backslash, \        b'\\'           92u8
// Newline             b'\n'           10u8
// Carriage return     b'\r'           13u8
// Tab                 b'\t'           9u8

// For characters that are hard to read or write, we can write their code in a hexadecimal instead. A byte literal of the form b'\xHH', where HH is any two-digit hexadecimal number, represents the byte whose value is HH. For example, you can write a byte literal for the ASCII “escape” control character as b'\x1b', since the ASCII code for “escape” is 27, or 1B in hexadecimal. It probably makes sense to use b'\x1b' instead of simply 27 only when you want to emphasize that the value represents an ASCII code.

// We can covert from on integer type to another using the as operator. It's further explained later on but here's a few examples:

assert_eq!( 10_i8 as u16, 10_u16); // in range
assert_eq!( 2525_u16 as i16, 2525_i16); // in range 
assert_eq!( -1_i16 as i32, -1_i32); // sign-extended
assert_eq!(65535_u16 as i32, 65535_i32); // zero-extended

// Conversions that are out of range for the destination
// produce values that are equivalent to the original modulo 2^N,
// where N is the width of the destination in bits. This
// is sometimes called "truncation".
assert_eq!( 1000_i16 as u8, 232_u8);
assert_eq!(65535_u32 as i16, -1_i16);
assert_eq!( -1_i8 as u8, 255_u8);
assert_eq!( 255_u8 as i8, -1_i8);

// Like any other sort of value, integers can have methods. For example:
assert_eq!(2u16.pow(4), 16); // exponentiation
assert_eq!((-4i32).abs(), 4); // absolute value
assert_eq!(0b101101u8.count_ones(), 4); // population count
// The type suffixes on the literals are required here. Rust can't look up a value's methods until it knows its type. In real code, there's usually additional context to disambiguate the type, so the suffixes aren't needed.


// Floating-Point Types

// Rust provides IEEE single and double precision floating-point types. These types include positive and negative infinities, distinct positive and negative zero values, and a not-a-number value:
// Type    Precision                                           Range
// f32     IEEE single precision (at least 6 decimal digits)   Roughly –3.4 × 10^38 to +3.4 × 10^38
// f64     IEEE double precision (at least 15 decimal digits)  Roughly –1.8 × 10^308 to +1.8 × 10^308

// 31415 (integer part) .926 (fractional part) e-4 (exponent) f64(type suffix)
// Every part of a floating-point number after the integer part is optional, but at least one of the fractional part, exponent, or type suffix must be present, to distinguish it from an integer literal. The fractional part may consist of a lone decimal point, so 5. is a valid gloating-point constant.
// If lacking a type suffix, Rust infers whether it's an f32 or f64 from the context, defaulting to f64 if both are possible.

// For the purposes of type inference, Rust treats integer literals and floating-point literals as distinct classes. It will never infer a floating-point type for an integer literal, or vice versa.
// Examples:
// Literal             Type        Mathematical value
// –1.5625             Inferred    −(1 ⁄ )
// 2.                  Inferred    2
// 0.25                Inferred    1/4
// 1e4                 Inferred    10,000
// 40f32               f32         40
// 9.109_383_56e-31f64 f64         Roughly 9.10938356 × 10

// The standard library’s std::f32 and std::f64 modules define constants for the IEEE-required special values like INFINITY, NEG_INFINITY (negative infinity), NAN (the not-a-number value), and MIN and MAX (the largest and smallest finite values). The std::f32::consts and std::f64::consts modules provide various commonly used mathematical constants like E, PI, and the square root of two.

// The f32 and f64 types provide a full complement of methods for mathematical calculations; for example, 2f64.sqrt() is the double precision square root of two.
// Examples:
assert_eq!(5f32.sqrt() * 5f32.sqrt(), 5.); // exactly 5.0, per IEEE
assert_eq!(-1.01f64.floor(), -1.0);
assert!((-1. / std::f32::INFINITY).is_sign_negative());

// As before, we don't usually need to write of the suffixes in real code because of context. When it doesn't, we get an error message. Example:
println!("{}", (2.0).sqrt());

// Rust complains:
// Error: no method named `sqrt` found for type `{float}` in the current scope

// It's an odd message as a floating-point type should be able to use a sqrt method. The issue above is that we didn't indicate a type one way or another:
println!("{}", (2.0_f64.sqrt()));
println!("{}", f64::sqrt(2.0));

// Rust performs almost no numeric conversions implicitly. If a function expects an f64 argument, it's an error to pass an i32 value as the argument. Rust won't even implicitly convert an i16 value to an i32 value even though every i16 value is also an i32 value. The key word here is implicitly: we can always write out explicit conversions using the as operator; i as f64, or x as i32. We expand on conversions in Type Casts later on.

// The bool Type

// Rust's Boolean type, bool, has the usual two values for such types, true, and false. Comparison operators like == and < produce bool results.

// Unlike Python that permits strings, list, dics, and even sets in Boolean contexts, Rust is very strict. Control structures like if and while require their conditions to be bool expressions, as do the short-circuiting logical operators && and ||. We must write if x!=0{...}, not simply if x{...}.

// Rust's as operator can convert bool values to integer types:
assert_eq!(false as i32, 0);
assert_eq!(true as i32, 1);
// 'as' however, won't convert in the other direction, from numeric types to bool. Instead we must write out an explicit comparison like x != 0.

// Although a bool only needs a single bit to represent it, Rust uses an entire byte for a bool value in memory, so we can create a pointer to it.

