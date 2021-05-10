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