fn main() {
    use impl_here::impl_here;

    #[impl_here(ArrayTrait)]
    impl<T, const L: usize> [T; L] {
        const TOTAL_SIZE: usize = size_of::<Self>();
        fn length(&self) -> usize {
            L
        }
    }

    assert_eq!([0.3; 125].length(), 125);
    assert_eq!(<[String; 2024]>::TOTAL_SIZE, 2024 * 24);

    #[impl_here(I32Square)]
    impl i32 {
        pub fn square(self) -> i32 {
            self * self
        }
    }
    // I32Square is public because fn square is public

    assert_eq!(13.square(), 13 * 13);

    let mut a = ();
    let a = &mut a;
    a.dbga();
    a.dbga();
    dbg!(14.1.square());
    dbg!(1.1.add_t(11u8));
    13.dbg();
    String::new().dbg();
    String::AAA.dbg();
    <[String; usize::MAX]>::LENGTH.dbg();
    <[String; 22]>::SIZE.dbg();
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
    const AAA: u32 = 2;
    pub fn dbg(&self) {
        println!("{:?}", self);
    }
}

#[impl_here(Array)]
impl<T, const L: usize> [T; L] {
    const LENGTH: usize = L;
    const SIZE: usize = { size_of::<Self>() };
    fn size(&self) -> usize {
        Self::SIZE
    }
}

struct AA;
impl AA {
    // type A = f64;
    pub const A: usize = 2;
}
#[impl_here(RefUnit)]
impl &mut () {
    pub fn dbga(self) {
        println!("&()");
    }
}
