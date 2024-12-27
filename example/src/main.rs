fn main() {
    dbg!(14.1.square());
    dbg!(1.1.add_t(11u8));
    13.dbg();
    String::new().dbg();
    // [1.4, 5.1, 323.1].dbg();
    // let aa = [2.2; 3].aa();
}
use std::fmt::Debug;

use impl_here::impl_here;

#[impl_here(A)]
impl f32 {
    fn square(self) -> f32 {
        self * self
    }
    fn add_t<T>(self, rhs: T) -> Self
    where
        f32: TryFrom<T>,
    {
        let r = match f32::try_from(rhs) {
            Ok(v) => v,
            Err(_) => panic!(),
        };
        r + self
    }
}
#[impl_here(F323)]
impl f32 {
    type A = f64;
    pub fn dbg(&self) {
        println!("{:?}", self);
    }
    pub(self) fn aa(self) -> impl Clone {}
    pub(crate) fn a() {}
}
#[impl_here(DebugA)]
impl<T> T
where
    T: Debug,
{
    pub fn dbg(&self) {
        println!("{:?}", self);
    }
}

struct AA;
impl AA {
    // type A = f64;
    const A: usize = 2;
}
