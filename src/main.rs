use core::num;

fn main() {
    /* ----------------------------- Variables ----------------------------- */

    // 1- mutable and immutable - let & const

    // -- immutable variables
    // let goosht = 100_000;
    // goosht = 10; // error
    // const SOSIS: i16 = 15;
    // SOSIS = 20; // error
    // println!("let : {} , const : {}", goosht, SOSIS);

    // -- mutable variables
    // let mut muta = 100_000;
    // muta = 10;
    // println!("{}", muta);

    // -- shadowing
    // let _sha = 100_000; // goes out of scope
    // let sha = 10;
    // println!("{}", sha);

    // -- ignore warning "unused variable"
    // let _sha = 100_000;
    // let sha = 10;
    // println!("{}", sha);

    // 2- data types - how to set type (with colum)

    // -- integers
    // let int8: i8 = 127; // -128 to 127
    // let int16: i16 = 32_767; // -32,768 to 32,767
    // let int32: i32 = 2_147_483_647; // -2,147,483,648 to 2,147,483,647 🔴 default
    // let int64: i64 = 2_147_483_647; // -2,147,483,648 to 2,147,483,647
    // let int128: i128 = 170_141_183_460_469_231_731_687_303_715_884_105_727; // -170,141,183,460,469,231,731,687,303,715,884,105,728 to 170,141,183,460,469,231,731,687,303,715,884,105,727
    // println!(
    //     "i8 : {} , i16 : {} , i32 : {} , i64 : {} , i128 : {}",
    //     int8, int16, int32, int64, int128
    // );

    // -- floats
    // let fl64 = 30.0; // 🔴 default
    // let fl32: f32 = 2.0;
    // println!("fl64 : {} , fl32 : {}", fl64, fl32);

    // -- boolean
    // let b = true;
    // println!("allocated size on memory : {} bytes", std::mem::size_of_val(&b));

    // -- character
    // let c = 'z';
    // let z: char = 'ℤ';
    // let heart_eyed_cat = '😻';
    // println!("{} {} {}", c, z, heart_eyed_cat)

    // -- tuple - declare & destructuring
    // let mut tup: (i32, f64, u8) = (500, 6.4, 1);
    // tup = (1, 1.1, 0);
    // let (first, second, third) = tup;
    // println!(
    //     "first : {} , second : {} , third : {}",
    //     first, second, third
    // );
    // println!("first : {} , second : {} , third : {}", tup.0, tup.1, tup.2);

    // -- array - low level languages confused
    // let a = [2, 3, 4, 10];
    // let b: [i8; 2] = [1, 127];
    // println!("first : {} , second : {}", a[2], b[0]);
    // println!("first : {} , second : {}", a[2], b[10]); // resolve wrong value based on memory address on Low-level languages

    /* ----------------------------- Functions & Control Flow ----------------------------- */

    // 1- declare a fun
    // fn first_fun(name: &str) {
    //     println!("hello {}", name)
    // }
    // first_fun("ali");

    // 2- expression vs statements

    // -- statement - it ends with semicolon
    // let sta = 4; // return nothing

    // -- expression
    // let exp = {
    //     let inside = 5;
    //     inside
    // };
    // println!("expression : {}", exp)

    // -- void vs returned functions
    // fn has_ret() -> bool {
    //     let bool: bool = true;
    //     bool
    // }
    // println!("retured value from has_ret : {}", has_ret());

    // 3- controller
    // if 2 > 3 { // arms
    //     println!("never happened")
    // } else {
    //     println!("2 is not greater than 3")
    // }

    /* ----------------------------- Loops ----------------------------- */

    // -- loop
    // let mut breaking: i8 = 1;
    // loop {
    //     println!("infinite");
    //     breaking += 1;
    //     if breaking > 10 {
    //         break;
    //     }
    // }

    // -- while
    // let mut number = 3;
    // while number != 0 {
    //     println!("{}", number);
    //     number -= 1;
    // }

    // -- for - better to use for loop over arrays
    // for number in 1..4 {
    //     println!("{}" , number)
    // }
}
