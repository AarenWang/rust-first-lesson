use std::cell::Cell;

fn sp_1() {
    let x = Box::new("Hello");
    let y = x;
    // println!("{:?}",x);  // value borrowed here after move
    //  move occurs because `x` has type `Box<&str>`, which does not implement the `Copy` trait
}

/// 对于Box＜T＞类型来说 如果包含的类型T属于复制语义 则执行按位复制；如果属于移动语义 则移动所有权
///解引用 智能指针
/// 变量a装箱的字符串字面量进行了按位复制
/// 而变量b装箱的String类型是引用语义，必须转移所有权
fn sp_2() {
    let a = Box::new("Hello");
    let b = Box::new("Rust".to_string());

    let c = *a;
    let d = *b;

    println!("{:?}", a);
    //println!("{:?}",b); // value borrowed here after move
    //  move occurs because `*b` has type `String`, which does not implement the `Copy` trait
}

pub fn main() {
    //sp_1();

    sp_2();
}
