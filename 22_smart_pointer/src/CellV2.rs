struct CellV2<T> {
    value: T,
}

impl<T> CellV2<T> {
    fn new(v: T) -> Self
    where
        T: Copy,
    {
        CellV2 { value: v }
    }

    fn set(&self, v: T)
    where
        T: Copy,
    {
        unsafe {
            let p = &(self.value) as *const T as *mut T;
            *p = v;
        }
    }

    fn get(&self) -> T
    where
        T: Copy,
    {
        self.value
    }
}

fn main() {
    let c = CellV2::new(1_isize);
    let p = &c;
    p.set(2);
    println!("{}", c.get());
}
