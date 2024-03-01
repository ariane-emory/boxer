// #![allow(unreachable_code)]
#![allow(unused_variables)]
// #![allow(unused_imports)]
// #![allow(unused_mut)]
// #![allow(dead_code)]

use boxer::block;
use boxer::block::*;
use std::io::{self};

fn render(char: u8, char2: u8, signal: &Signal<usize>, width: &Signal<usize>) {
  for _ in 0..*signal.borrow().read() {
    print!("{}", char as char);
  }
  for _ in 0..(*width.borrow().read() - *signal.borrow().read()) {
    print!("{}", char2 as char);
  }
  println!("");
}

////////////////////////////////////////////////////////////////////////////////////////////////////
fn main() -> io::Result<()> {
  let one = Value::new(1);
  let mut clock = SquareWave::new(one.output());

  {
    let mut counter_reset = Feedback::new();
    let counter_max = Value::new(64);
    let mut counter = UpCounter::new(clock.output(), counter_reset.output(), counter_max.output());
    let mut add = block::Add::new(counter.output(), one.output());
    let two = Value::new(2);
    let mut div = Div::new(add.output(), two.output());
    let mut square = SquareWave::new(div.output());
    let max = Value::new(128);
    let zero = Value::new(0);
    let mut select = Select::new(square.output(), zero.output(), max.output());

    counter_reset.set_input(&counter.at_max());

    for _ in 0..511 {
      clock.step();
      counter.step();
      add.step();
      div.step();
      square.step();
      select.step();
      counter_reset.step();

      // println!("");
      // println!("counter input:  {}", clock.output().borrow().read());
      // println!("counter output: {}", counter.output().borrow().read());
      // println!("add output:     {}", add.output().borrow().read());
      // println!("square period:  {}", square.period().borrow().read());
      // println!("square output:  {}", square.output().borrow().read());
      // println!("select output:  {}", select.output().borrow().read());

      render(b'x', b'-', select.output(), max.output());
    }
  }

  {
    let counter_max = Value::new(128);
    let mut counter_reset = Feedback::new();
    let mut counter = UpCounter::new(clock.output(), &counter_reset.output(), counter_max.output());
    let mut sr_set = Feedback::new();
    let mut sr_reset = Feedback::new();
    let mut sr = SRLatch::new(sr_set.output(), sr_reset.output());
    let mut not_latched = Not::new(sr.output());
    let mut counter_at_max_and_latched = And::new(counter.at_max(), sr.output());
    let mut counter_at_max_and_not_latched = And::new(counter.at_max(), not_latched.output());
    let mut sub = Sub::new(counter_max.output(), counter.output());
    let mut select = Select::new(sr.output(), counter.output(), sub.output());

    counter_reset.set_input(&counter.at_max());
    sr_set.set_input(&counter_at_max_and_not_latched.output());
    sr_reset.set_input(&counter_at_max_and_latched.output());

    for _ in 0..511 {
      // println!("");

      clock.step();
      counter.step();
      not_latched.step();
      counter_at_max_and_latched.step();
      counter_at_max_and_not_latched.step();
      sr.step();
      sub.step();
      select.step();
      counter_reset.step();
      sr_set.step();
      sr_reset.step();

      // println!("counter:             {}", counter.output().borrow().read());
      // println!("counter.at_max:      {}", counter.at_max.borrow().read());
      // println!("counter reset:       {}", counter_reset.output().borrow().read());
      // println!("sr_reset:            {}", sr_reset.output().borrow().read());
      // println!("sr:                  {}", sr.output().borrow().read());
      // println!("sub:                 {}", sub.output().borrow().read());
      // println!("select:              {}", select.output().borrow().read());
      // println!("not_latched:         {}", not_latched.output().borrow().read());
      // println!("max_and_latched:     {}", max_and_latched.output().borrow().read());
      // println!("max_and_not_latched: {}", max_and_not_latched.output().borrow().read());
      render(b'x', b'-', select.output(), counter_max.output());
    }
  }

  {
    let div_input = Feedback<isize>::new();
    let add_left_input = Feedback<isize>::new();

    let two = Value<isize>::new(2);
    let div_by_two = Div<isize>::new(div_input, two.output());

    
    let add = Add::new(add_left_input.output(), div_by_two.output());
    let sample_and_hold = SampleAndHold::new(add.output(), clock.output());
    
    let sixteen = Value::new(16);
    let mut square = SquareWave::new(sixteen.output());
    let max = Value::new(128);
    let zero = Value::new(0);
    let mut select = Select::new(square.output(), zero.output(), max.output());

    for _ in 0..511 {
      clock.step();
      square.step();
      select.step();

      // println!("");
      // println!("counter input:  {}", clock.output().borrow().read());
      // println!("counter output: {}", counter.output().borrow().read());
      // println!("add output:     {}", add.output().borrow().read());
      // println!("square period:  {}", square.period().borrow().read());
      // println!("square output:  {}", square.output().borrow().read());
      // println!("select output:  {}", select.output().borrow().read());

      render(b'x', b'-', select.output(), max.output());
    }
  }

  Ok(())
}
