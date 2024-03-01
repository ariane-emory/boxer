// #![allow(unreachable_code)]
// #![allow(unused_variables)]
// #![allow(unused_imports)]
// #![allow(unused_mut)]
// #![allow(dead_code)]

use boxer::block;
use boxer::block::*;
use std::io::{self};

////////////////////////////////////////////////////////////////////////////////////////////////////
fn main() -> io::Result<()> {
  // loop {
  // let left = block::Value::new(1);
  // let twenty = block::Value::new(20);
  // let mut adder = block::MathAdd::new(left.output(), left.output());
  // let mut subber = block::MathSub::new(twenty.output(), adder.output());

  // println!("Adder: {}", adder.output().borrow().read());
  // println!("Subber: {}", subber.output().borrow().read());

  // adder.step();
  // subber.step();

  // println!("Adder: {}", adder.output().borrow().read());
  // println!("Subber: {}", subber.output().borrow().read());

  // left.output.borrow_mut().set(8);

  // adder.step();
  // subber.step();

  // println!("Adder: {}", adder.output().borrow().read());
  // println!("Subber: {}", subber.output().borrow().read());
  // println!("");

  let one = Value::new(1);
  let mut fast_square = SquareWave::new(one.output());

  let counter_reset = block::Value::new(false);
  let counter_max = Value::new(8);
  let mut counter = Counter::new(
    fast_square.output(),
    counter_reset.output(),
    counter_max.output(),
  );

  let mut adder = block::MathAdd::new(counter.output(), one.output());

  let mut square = SquareWave::new(counter.output());

  let max = Value::new(64);
  let zero = Value::new(0);
  let mut select = Select::new(square.output(), zero.output(), max.output());


  for _x in 0..128 {
    fast_square.step();
    counter.step();
    adder.step();
    square.step();
    select.step();

    if *counter.at_max.borrow().read() {
      counter_reset.output.borrow_mut().set(true);
      counter.step();
      counter_reset.output.borrow_mut().set(false);
    }

    println!("");
    println!("counter input:  {}", fast_square.output.borrow().read());
    println!("counter output: {}", counter.output().borrow().read());
    println!("adder output:   {}", adder.output().borrow().read());
    println!("square period:  {}", square.period.borrow().read());
    println!("square output:  {}", square.output().borrow().read());
    println!("select output:  {}", select.output().borrow().read());

    // for _ in 0..*select.output().borrow().read() {
    //   print!("x");
    // }
    // println!("");

    // let counter_input_val = *counter_input.output().borrow().read();
    // counter_input.output().borrow_mut().set(!counter_input_val);
  }

  Ok(())
}
