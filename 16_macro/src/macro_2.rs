use std::fmt::{Display, Formatter};
macro_rules! times_five {
    ($e:expr) => {
        5 * $e
    };
}

macro_rules! multiply_add {
    ($a:expr, $b:expr, $c:expr) => {
        $a * ($b + $c)
    };
}

macro_rules! what_is {
    (self) => {
        "the keyword `self`"
    };
    ($i:ident) => {
        concat!("the identifier `", stringify!($i), "`")
    };
}

macro_rules! call_with_ident {
    ($c:ident($i:ident)) => {
        $c!($i)
    };
}

macro_rules! vec_strs {
    (
        // Start a repetition:
        $(
            // Each repeat must contain an expression...
            $element:expr
        )
        // ...separated by commas...
        ,
        // ...zero or more times.
        *
    ) => {
        // Enclose the expansion in a block so that we can use
        // multiple statements.
        {
            let mut v = Vec::new();

            // Start a repetition:
            $(
                // Each repeat will contain the following statement, with
                // $element replaced with the corresponding expression.
                v.push(format!("{}", $element));
            )*

            v
        }
    };
}

fn main() {
    let r1 = times_five!(2);
    println!("r1={}", r1);

    let r2 = multiply_add!(5, 4, 6);
    println!("r2={}", r2);

    let r3 = vec_strs!["a", 1, 8u8, String::from("Hello")];
    println!("r3={:?}", r3);

    println!("{}", what_is!(self));
    println!("{}", call_with_ident!(what_is(self)));
}
