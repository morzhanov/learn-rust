fn main() {
    // scalar types
    // Rust has four primary scalar types:
    // - integers
    // - floating-point numbers
    // - Booleans
    // - characters

    // integer types
    // 8-bit	i8	u8
    // 16-bit	i16	u16
    // 32-bit	i32	u32
    // 64-bit	i64	u64
    // 128-bit	i128	u128
    // arch	isize	usize

    let _8_bit_signed: i8 = 0;
    let _8_bit_unsigned: u8 = 0;
    let _16_bit_signed: i16 = 0;
    let _16_bit_unsigned: u16 = 0;
    let _32_bit_signed: i32 = 0;
    let _32_bit_unsigned: u32 = 0;
    let _64_bit_signed: i64 = 0;
    let _64_bit_unsigned: u64 = 0;
    let _128_bit_signed: i128 = 0;
    let _128_bit_unsigned: u128 = 0;
    // The pointer-sized signed integer type.
    // The size of this primitive is how many bytes it takes to reference any location in memory.
    // For example, on a 32 bit target, this is 4 bytes and on a 64 bit target, this is 8 bytes.
    let _arch_bit_signed: isize = 0;
    let _arch_bit_unsigned: usize = 0;

    // number literals
    // Decimal	98_222
    // Hex	0xff
    // Octal	0o77
    // Binary	0b1111_0000
    // Byte (u8 only)	b'A'
    let _decimal_number_literal = 98_122;
    let _hex_number_literal = 0xff;
    let _octal_number_literal = 0o77;
    let _binary_number_literal = 0b1111_0000_1010;
    let _byte_number_literal = b'A';

    // floating points
    // the default type is f64
    let _f64_number = 1.1;
    let _f32_number: f32 = 1.1;

    // numeric operations

    // addition
    let _sum = 5 + 10;
    // subtraction
    let _difference = 95.5 - 4.3;
    // multiplication
    let _product = 4 * 30;
    // division
    let _quotient = 56.7 / 32.2;
    // Results in 0
    let _floored = 2 / 3;
    // remainder
    let _remainder = 43 % 5;

    // boolean

    // char type
    // char type is the language’s most primitive alphabetic type
    let _c = 'z';
    let _z = 'ℤ';
    let _heart_eyed_cat = '😻';

    // compound types

    // tuples
    // tuple is a general way of grouping together a number of values with a variety of types into one compound type
    // tuples have a fixed length: once declared, they cannot grow or shrink in size
    let _tup: (i32, f64, u8) = (500, 6.4, 1);

    // getting values from tuple (destructuring)
    let (_x, _y, _z) = tup;

    // we can also access a tuple element directly by using a period (.) followed by the index of the value we want to access
    let _five_hundred = x.0;
    let _six_point_four = x.1;
    let _one = x.2;

    // tuple without any values, (), is a special type that has only one value, also written ()
    // the type is called the `unit type` and the value is called the `unit value`
    // expressions implicitly return the unit value if they don’t return any other value
    let _unit: () = ();

    // array
    // unlike arrays in some other languages, arrays in Rust have a fixed length
    // an array isn’t as flexible as the vector type, though
    // a vector is a similar collection type provided by the standard library that is allowed to grow or shrink in size
    let _a = [1, 2, 3, 4, 5];
    let _months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    // you write an array’s type using square brackets with the type of each element, a semicolon, and then the number of elements in the array, like so
    let _a: [i32; 5] = [1, 2, 3, 4, 5];

    // you can also initialize an array to contain the same value for each element by specifying the initial value
    // followed by a semicolon, and then the length of the array in square brackets, as shown here
    let _a = [3; 5]; // let a = [3, 3, 3, 3, 3]

    // you can access elements of an array using indexing, like this
    let _first = _a[0];
    let _second = _a[1];
}
