fn iter_bytes<T: AsRef<[u8]>>(arg: T) {
    for i in arg.as_ref() {
        println!("{}", i);
    }
}

fn main() {
    let s: String = String::from("this is a string");
    let v: Vec<u8> = vec![1, 2, 3];
    let c: &str = "hello";
    iter_bytes(s);
    iter_bytes(v);
    iter_bytes(c);
}
