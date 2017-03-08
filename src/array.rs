//! Stolen from dimensioned and modified for unsigned integers

use typenum::{Add1, B1, Length, TArr, ATerm, Len, Unsigned, U0};
use generic_array::{GenericArray, ArrayLength};
use std::ops::Add;


pub trait ToGA {
    /// The type of the `GenericArray` to which we've converted
    type Output;
    /// Create a `GenericArray` of integers from a `TArr` of type numbers.
    fn to_ga() -> Self::Output;
}


impl ToGA for ATerm {
    type Output = GenericArray<usize, U0>;
    fn to_ga() -> Self::Output {
        GenericArray::new()
    }
}


impl<V, A> ToGA for TArr<V, A>
    where V: Unsigned,
          A: Len + ToGA,
          <A as ToGA>::Output: AppendFront<usize>,
          Length<A>: Add<B1>,
          Add1<Length<A>>: Unsigned + ArrayLength<usize>
{
    type Output = <<A as ToGA>::Output as AppendFront<usize>>::Output;
    fn to_ga() -> Self::Output {
        A::to_ga().append_front(V::to_usize())
    }
}

pub trait AppendFront<T> {
    /// The resulting type after performing the append
    type Output;
    /// Append `element` to the front of `self`.
    fn append_front(self, element: T) -> Self::Output;
}

impl<T, N> AppendFront<T> for GenericArray<T, N>
    where T: Default,
          N: Add<B1> + ArrayLength<T>,
          Add1<N>: ArrayLength<T>
{
    type Output = GenericArray<T, Add1<N>>;
    fn append_front(self, element: T) -> Self::Output {
        let mut a = GenericArray::new();
        a[0] = element;
        for (i, el) in self.into_iter().enumerate() {
            a[i + 1] = el;
        }
        a
    }
}
