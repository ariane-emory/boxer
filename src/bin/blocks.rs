// #![allow(unreachable_code)]
// #![allow(unused_variables)]
// #![allow(unused_imports)]
// #![allow(unused_mut)]
//#![allow(dead_code)]

use boxer::block;
use boxer::block::*;
use boxer::process_file::*;
use boxer::simple_geo::find_rectangles;
use boxer::simple_geo::line_methods::*;
use std::cell::RefCell;
use std::io::{self};

////////////////////////////////////////////////////////////////////////////////////////////////////
fn main() -> io::Result<()> {
  // loop {
  let mut left = block::Value::new(1);
  let mut twenty = block::Value::new(20);
  let mut adder = block::MathAdd::new(&left.output, &left.output);
  let mut subber = block::MathSub::new(&twenty.output, &adder.output);

  println!("Adder: {}", adder.output.borrow().read());
  println!("Subber: {}", subber.output.borrow().read());

  adder.step();
  subber.step();

  println!("Adder: {}", adder.output.borrow().read());
  println!("Subber: {}", subber.output.borrow().read());

  left.output.borrow_mut().set(8);

  adder.step();
  subber.step();

  println!("Adder: {}", adder.output.borrow().read());
  println!("Subber: {}", subber.output.borrow().read());
  // }

  // let five = block::Value::new(5);

  // println!("Adder: {}", adder.output.read());
  // adder.step();
  // println!("Adder: {}", adder.output.read());
  // left.output.set(2);
  // adder.step();
  // println!("Adder: {}", adder.output.read());

  // let mut flip = block::Value::new(false);
  // let ten = block::Value::new(10);
  // let mut ctr = block::RiseCounter::new(&flip.output, &ten.output);
  // println!("Ctr: {}", ctr.count.read());
  // ctr.step();
  // println!("Ctr: {}", ctr.count.read());
  // flip.output.set(true);
  // ctr.step();
  // println!("Ctr: {}", ctr.count.read());

  Ok(())
}
