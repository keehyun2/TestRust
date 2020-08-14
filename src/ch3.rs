pub fn chater3() {

    // 3. rust 변수, 가변성, 타입, 함수 동작, 주석, 제어문

    // let x = 5;
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // 데이터 타입들
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("guess = {}", guess);

    // Length	Signed	Unsigned
    // 8-bit	i8	    u8
    // 16-bit	i16	    u16
    // 32-bit	i32	    u32
    // 64-bit	i64	    u64
    // arch	    isize	usize

    // Rust의 정수형 리터럴들 ?? 
    // Number literals	Example
    // Decimal	98_222
    // Hex	0xff
    // Octal	0o77
    // Binary	0b1111_0000
    // Byte (u8 only)	b'A'

    let a = 98_222;
    println!("a = {}", a);

    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
    another_function(5, 6);

    ;
    println!("ddd : {}", five(5));
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y)
}

fn five(x: i32) -> i32 {
    x * 5
}