#[macro_use] extern crate typenum;
extern crate generic_array;

use std::ops::*;
use typenum::*;
use std::marker::PhantomData;

mod array;
pub use array::ToGA;

// Index for type array (TArr)
pub trait Index<I> {
    type Output;
}
pub type Idx<A, I> = <A as Index<I>>::Output;

impl Index<U0> for ATerm {
    type Output = U0;
}

impl<V, A> Index<U0> for TArr<V, A> {
    type Output = V;
}

impl<U, B, V, A> Index<UInt<U, B>> for TArr<V, A> where UInt<U, B>: Sub<B1>, A: Index<Sub1<UInt<U, B>>> {
    type Output = Idx<A, Sub1<UInt<U, B>>>;
}

// Set value at index I to V
pub trait SetIndex<I, V> {
    type Output;
}
pub type Set<A, I, V> = <A as SetIndex<I, V>>::Output;

impl<A, New, Old> SetIndex<U0, New> for TArr<Old, A> {
    type Output = TArr<New, A>;
}

impl<U, B, V, A, New> SetIndex<UInt<U, B>, New> for TArr<V, A> where UInt<U, B>: Sub<B1>, A: SetIndex<Sub1<UInt<U, B>>, New> {
    type Output = TArr<V, Set<A, Sub1<UInt<U, B>>, New>>;
}

// Our operators
pub struct Increment<R, I> {
    _marker: PhantomData<(R, I)>,
}
pub struct Decrement<R, I1, I2> {
    _marker: PhantomData<(R, I1, I2)>,
}
pub struct Halt;


// Evaluate
pub trait Eval<Instruction> {
    type Output;
}
pub type Evaluate<Registers, Instructions, I> = <(Registers, Instructions) as Eval<I>>::Output;

// Halt
impl<Registers, Instructions> Eval<Halt> for (Registers, Instructions) {
    type Output = Registers;
}

// Increment
impl<Registers, Instructions, Reg, Ins> Eval<Increment<Reg, Ins>> for (Registers, Instructions) where
    Registers: Index<Reg>,
    Idx<Registers, Reg>: Add<B1>,
    Registers: SetIndex<Reg, Add1<Idx<Registers, Reg>>>,
    Instructions: Index<Ins>,
    (Set<Registers, Reg, Add1<Idx<Registers, Reg>>>, Instructions): Eval<Idx<Instructions, Ins>>,
{
    type Output = Evaluate<Set<Registers, Reg, Add1<Idx<Registers, Reg>>>, Instructions, Idx<Instructions, Ins>>;
}

// Decrement, first step
impl<Registers, Instructions, Reg, I1, I2> Eval<Decrement<Reg, I1, I2>> for (Registers, Instructions) where
    Registers: Index<Reg>,
    (Registers, Instructions): PrivateDecrement<Reg, I1, I2, Idx<Registers, Reg>>,
{
    type Output = PrivateDecrementOut<Registers, Instructions, Reg, I1, I2, Idx<Registers, Reg>>;
}

pub trait PrivateDecrement<Reg, I1, I2, Value> {
    type Output;
}
pub type PrivateDecrementOut<Registers, Instructions, Reg, I1, I2, Value> =
    <(Registers, Instructions) as PrivateDecrement<Reg, I1, I2, Value>>::Output;

// PrivateDecrement, Register is zero
impl<Registers, Instructions, Reg, I1, I2> PrivateDecrement<Reg, I1, I2, U0> for (Registers, Instructions) where
    Instructions: Index<I1>,
    (Registers, Instructions): Eval<Idx<Instructions, I1>>,
{
    type Output = Evaluate<Registers, Instructions, Idx<Instructions, I1>>;
}

// PrivateDecrement, Register is non-zero
impl<Registers, Instructions, Reg, I1, I2, U, B> PrivateDecrement<Reg, I1, I2, UInt<U, B>>
    for (Registers, Instructions) where
    Registers: Index<Reg>,
    Idx<Registers, Reg>: Sub<B1>,
    Registers: SetIndex<Reg, Sub1<Idx<Registers, Reg>>>,
    Instructions: Index<I2>,
    (Set<Registers, Reg, Sub1<Idx<Registers, Reg>>>, Instructions): Eval<Idx<Instructions, I2>>,
{
    type Output = Evaluate<Set<Registers, Reg, Sub1<Idx<Registers, Reg>>>, Instructions, Idx<Instructions, I2>>;
}

pub type Execute<Registers, Instructions> = Evaluate<Registers, Instructions, Idx<Instructions, U0>>;
