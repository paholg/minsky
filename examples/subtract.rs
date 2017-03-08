#[macro_use]
extern crate typenum;
extern crate minsky;

use typenum::consts::*;
use minsky::{Execute, Decrement, Increment, Halt, ToGA};

type Instructions = tarr![
    Decrement<U1, U3, U1>,
    Decrement<U0, U2, U0>,
    Increment<U1, U3>,
    Halt
];

fn main() {
    type Minuend = U10;
    type Subtrahend = U7;

    type Registers = tarr![Minuend, Subtrahend];
    println!("Registers before: {:?}", Registers::to_ga());

    type Result = Execute<Registers, Instructions>;
    println!("Registers after: {:?}", <Result as ToGA>::to_ga());

    use minsky::Idx;
    use typenum::Unsigned;
    type Difference = Idx<Result, U0>;
    println!("Computation performed: {} - {} = {:?}",
             Minuend::to_usize(),
             Subtrahend::to_usize(),
             <Difference as Unsigned>::to_usize());
}
