use std::rc::Rc;

fn rc_v1() {
    let x = Rc::new(45);
    let y1 = x.clone(); //增加引用计数器
    let y2 = x.clone(); //增加引用计数器
    println!("{:?}", Rc::strong_count(&x));

    let w = Rc::downgrade(&x); // 增加弱引用计数
    println!("{:?}", Rc::weak_count(&x));

    let y3 = &*x; // 不增加计数
    println!(" {} ", 100 - *x);
}
pub fn main() {
    rc_v1();
}
