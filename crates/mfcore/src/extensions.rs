

pub trait SwapExt {
    fn swap(&mut self, other: &mut Self);
}

impl<T> SwapExt for T {
    #[inline(always)]
    fn swap(&mut self, other: &mut Self) {
        ::core::mem::swap(self, other)
    }
}

pub trait ReplaceExt {
    fn replace(&mut self, value: Self) -> Self;
}

impl<T> ReplaceExt for T {
    #[inline(always)]
    fn replace(&mut self, value: Self) -> Self {
        ::core::mem::replace(self, value)
    }
}