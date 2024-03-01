// #![allow(unreachable_code)]
#![allow(unused_variables)]
// #![allow(unused_imports)]
// #![allow(unused_mut)]
// #![allow(dead_code)]

use boxer::block;
use boxer::block::*;
use std::io::{self};

fn render(char: u8, char2: u8, signal: usize, width: usize) {
  for _ in 0..signal {
    print!("{}", char as char);
  }
  for _ in 0..(width - signal) {
    print!("{}", char2 as char);
  }
  println!("");
}

////////////////////////////////////////////////////////////////////////////////////////////////////
fn main() -> io::Result<()> {
  let one = Value::new(1);
  let never = Value::new(false);
  let sixteen = Value::new(16);
  let max = Value::new(128);
  let ifour = Value::<isize>::new(4);
  let imax = Value::<isize>::new(max.output_value() as isize);
  let ithree = Value::<isize>::new(3);
  let itwo = Value::<isize>::new(2);
  let izero = Value::new(0);
  let mut clock = SquareWave::new(one.output());

  //loop {
  {
    let mut counter_reset = Feedback::new();
    let counter_max = Value::new(64);
    let mut counter = UpCounter::new(clock.output(), counter_reset.output(), counter_max.output());
    let mut add = block::Add::new(counter.output(), one.output());
    let two = Value::new(2);
    let mut div = Div::new(add.output(), two.output());
    let mut square = SquareWave::new(div.output());
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

      render(b'x', b'-', *select.output().borrow().read(), *max.output().borrow().read());
    }
  }

  {
    let mut counter_reset = Feedback::new();
    let mut counter = UpCounter::new(clock.output(), &counter_reset.output(), max.output());
    let mut sr_set = Feedback::new();
    let mut sr_reset = Feedback::new();
    let mut sr = SRLatch::new(sr_set.output(), sr_reset.output());
    let mut not_latched = Not::new(sr.output());
    let mut counter_at_max_and_latched = And::new(counter.at_max(), sr.output());
    let mut counter_at_max_and_not_latched = And::new(counter.at_max(), not_latched.output());
    let mut sub = Sub::new(max.output(), counter.output());
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

      render(b'x', b'-', *select.output().borrow().read(), *max.output().borrow().read());
    }
  }

  {
    let mut square = SquareWave::new(sixteen.output());
    let mut select = Select::new(square.output(), izero.output(), imax.output());

    let mut held_value = Feedback::<isize>::new();
    let mut div_held_value_by_itwo = Div::<isize>::new(held_value.output(), itwo.output());
    let mut div_new_input_by_itwo = Div::<isize>::new(select.output(), itwo.output());
    let mut add = Add::new(div_held_value_by_itwo.output(), div_new_input_by_itwo.output());
    let mut sample_and_hold = SampleAndHold::new(add.output(), clock.output(), never.output());
    held_value.set_input(&sample_and_hold.output());


    for _ in 0..511 {
      clock.step();
      square.step();
      select.step();
      held_value.step();
      div_new_input_by_itwo.step();
      div_held_value_by_itwo.step();
      add.step();
      sample_and_hold.step();

      // println!("");
      // println!("counter input:  {}", clock.output().borrow().read());
      // println!("counter output: {}", counter.output().borrow().read());
      // println!("add output:     {}", add.output().borrow().read());
      // println!("square period:  {}", square.period().borrow().read());
      // println!("square output:  {}", square.output().borrow().read());
      // println!("select output:  {}", select.output().borrow().read());
      // println!("s&h output:  {}", sample_and_hold.output().borrow().read());
      //render(b'x', b'-', select.output(), imax.output());

      render(
        b'x',
        b'-',
        *add.output().borrow().read() as usize,
        *imax.output().borrow().read() as usize,
      );
    }
  }
  //  }
  Ok(())
}
