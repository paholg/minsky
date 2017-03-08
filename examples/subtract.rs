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

type Registers = tarr![U10, U7];

fn main() {
    type Result = Execute<Registers, Instructions>;
    println!("{:?}", <Result as ToGA>::to_ga());
}
