use core::marker;

#[repr(transparent)]
pub struct AssertSizeIsLessOrEqualThan<const SIZE: usize, const EXPECT: usize>(marker::PhantomData<([(); SIZE], [(); EXPECT])>);
impl<const SIZE: usize, const EXPECT: usize> AssertSizeIsLessOrEqualThan<SIZE, EXPECT> {
    pub const RESULT: () = {
        assert!(SIZE <= EXPECT, "Buffer SIZE overflows EXPECT");
    };
}
