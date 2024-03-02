#![allow(unreachable_code)]
//#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(dead_code)]

use boxer::block;
use boxer::block::*;

use std::cell::RefCell;
use std::io::{self};
use std::rc::Rc;

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
type RcRcSteppable = Rc<RefCell<dyn Steppable>>;

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
fn add_to_rcrc_steppable_vec<T: 'static + Steppable>(blocks: &mut Vec<RcRcSteppable>, item: &Rc<RefCell<T>>) {
  let steppable_item: RcRcSteppable = item.clone() as Rc<RefCell<dyn Steppable>>;
  blocks.push(steppable_item);
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
fn new_rcrc<T>(item: T) -> Rc<RefCell<T>> { Rc::new(RefCell::new(item)) }

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
fn render(char: u8, char2: u8, signal: usize, width: usize) {
  let mut printed = 0;
  let quarterways = width >> 2;
  let mut next_div = quarterways;

  print!("|");
  for _ in 0..signal {
    if printed == next_div {
      print!("|");
      next_div = next_div + quarterways;
    }
    print!("{}", char as char);
    printed = printed + 1;
  }
  for _ in 0..(width - signal) {
    if printed == next_div {
      print!("|");
      next_div = next_div + quarterways;
    }
    print!("{}", char2 as char);
    printed = printed + 1;
  }
  println!("|");
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
fn main() -> io::Result<()> {
  // let never = Value::new(false);
  // let itwo = Value::<isize>::new(2);
  // let ifour = Value::<isize>::new(4);

  // let ithree = Value::<isize>::new(3);

  //loop {
  {
    let one = new_rcrc(Value::new(1));
    let max = new_rcrc(Value::new(128));
    let clock = new_rcrc(SquareWave::new(one.borrow_mut().output()));
    let counter_reset = new_rcrc(Feedback::new());
    let counter_max = new_rcrc(Value::new(64));
    let counter = new_rcrc(UpCounter::new(clock.borrow_mut().output(), counter_reset.borrow_mut().output(), counter_max.borrow_mut().output()));
    let add = new_rcrc(Add::new(counter.borrow_mut().output(), one.borrow_mut().output()));
    let two = new_rcrc(Value::new(2));
    let div = new_rcrc(Div::new(add.borrow_mut().output(), two.borrow_mut().output()));
    let square = new_rcrc(SquareWave::new(div.borrow_mut().output()));
    let zero = new_rcrc(Value::new(0));
    let select = new_rcrc(Select::new(square.borrow_mut().output(), zero.borrow_mut().output(), max.borrow_mut().output()));

    counter_reset.borrow_mut().set_input(&counter.borrow_mut().at_max());

    let mut blocks: Vec<RcRcSteppable> = Vec::new();
    add_to_rcrc_steppable_vec(&mut blocks, &one);
    add_to_rcrc_steppable_vec(&mut blocks, &max);
    add_to_rcrc_steppable_vec(&mut blocks, &clock);
    add_to_rcrc_steppable_vec(&mut blocks, &counter_reset);
    add_to_rcrc_steppable_vec(&mut blocks, &counter_max);
    add_to_rcrc_steppable_vec(&mut blocks, &counter);
    add_to_rcrc_steppable_vec(&mut blocks, &add);
    add_to_rcrc_steppable_vec(&mut blocks, &two);
    add_to_rcrc_steppable_vec(&mut blocks, &div);
    add_to_rcrc_steppable_vec(&mut blocks, &square);
    add_to_rcrc_steppable_vec(&mut blocks, &zero);
    add_to_rcrc_steppable_vec(&mut blocks, &select);

    for _ in 0..511 {
      blocks.iter_mut().for_each(|b| b.borrow_mut().step());
      render(b'x', b'-', select.borrow_mut().output_value(), max.borrow_mut().output_value());
    }
  }

  {
    let izero = new_rcrc(Value::new(0));
    let max = new_rcrc(Value::new(128));
    let imax = new_rcrc(Value::<isize>::new(max.borrow_mut().output_value() as isize));
    let one = new_rcrc(Value::new(1));
    let clock = new_rcrc(SquareWave::new(one.borrow_mut().output()));
    let counter_reset = new_rcrc(Feedback::new());
    let counter = new_rcrc(UpCounter::new(clock.borrow_mut().output(), &counter_reset.borrow_mut().output(), max.borrow_mut().output()));
    let sr_set = new_rcrc(Feedback::new());
    let sr_reset = new_rcrc(Feedback::new());
    let sr = new_rcrc(SRLatch::new(sr_set.borrow_mut().output(), sr_reset.borrow_mut().output()));
    let not_latched = new_rcrc(Not::new(sr.borrow_mut().output()));
    let counter_at_max_and_latched = new_rcrc(And::new(counter.borrow_mut().at_max(), sr.borrow_mut().output()));
    let counter_at_max_and_not_latched = new_rcrc(And::new(counter.borrow_mut().at_max(), not_latched.borrow_mut().output()));
    let sub = new_rcrc(Sub::new(max.borrow_mut().output(), counter.borrow_mut().output()));
    let select = new_rcrc(Select::new(sr.borrow_mut().output(), counter.borrow_mut().output(), sub.borrow_mut().output()));

    counter_reset.borrow_mut().set_input(&counter.borrow_mut().at_max());
    sr_set.borrow_mut().set_input(&counter_at_max_and_not_latched.borrow_mut().output());
    sr_reset.borrow_mut().set_input(&counter_at_max_and_latched.borrow_mut().output());

    let mut blocks: Vec<RcRcSteppable> = Vec::new();
    add_to_rcrc_steppable_vec(&mut blocks, &izero);
    add_to_rcrc_steppable_vec(&mut blocks, &max);
    add_to_rcrc_steppable_vec(&mut blocks, &imax);
    add_to_rcrc_steppable_vec(&mut blocks, &one);
    add_to_rcrc_steppable_vec(&mut blocks, &clock);
    add_to_rcrc_steppable_vec(&mut blocks, &counter_reset);
    add_to_rcrc_steppable_vec(&mut blocks, &counter);
    add_to_rcrc_steppable_vec(&mut blocks, &sr_set);
    add_to_rcrc_steppable_vec(&mut blocks, &sr_reset);
    add_to_rcrc_steppable_vec(&mut blocks, &sr);
    add_to_rcrc_steppable_vec(&mut blocks, &not_latched);
    add_to_rcrc_steppable_vec(&mut blocks, &counter_at_max_and_latched);
    add_to_rcrc_steppable_vec(&mut blocks, &counter_at_max_and_not_latched);
    add_to_rcrc_steppable_vec(&mut blocks, &sub);
    add_to_rcrc_steppable_vec(&mut blocks, &select);

    for _ in 0..511 {
      blocks.iter_mut().for_each(|b| b.borrow_mut().step());
      render(b'x', b'-', select.borrow_mut().output_value(), max.borrow_mut().output_value());
    }
  }

  {
    let max = new_rcrc(Value::new(128));
    let imax = new_rcrc(Value::<isize>::new(max.borrow_mut().output_value() as isize));
    let one = new_rcrc(Value::new(1));
    let clock = new_rcrc(SquareWave::new(one.borrow_mut().output()));
    let sixteen = new_rcrc(Value::new(16));
    let square = new_rcrc(SquareWave::new(sixteen.borrow_mut().output()));
    let izero = new_rcrc(Value::new(0));
    let select = new_rcrc(Select::new(square.borrow_mut().output(), izero.borrow_mut().output(), imax.borrow_mut().output()));
    let held_value = new_rcrc(Feedback::<isize>::new());
    let never = new_rcrc(Value::new(false));
    let itwo = new_rcrc(Value::<isize>::new(2));
    let div_held_value_by_itwo = new_rcrc(Div::<isize>::new(held_value.borrow_mut().output(), itwo.borrow_mut().output()));
    let div_new_input_by_itwo = new_rcrc(Div::<isize>::new(select.borrow_mut().output(), itwo.borrow_mut().output()));
    let add = new_rcrc(Add::new(div_held_value_by_itwo.borrow_mut().output(), div_new_input_by_itwo.borrow_mut().output()));
    let sample_and_hold = new_rcrc(SampleAndHold::new(add.borrow_mut().output(), clock.borrow_mut().output(), never.borrow_mut().output()));

    held_value.borrow_mut().set_input(&sample_and_hold.borrow_mut().output());

    let mut blocks: Vec<RcRcSteppable> = Vec::new();
    add_to_rcrc_steppable_vec(&mut blocks, &max);
    add_to_rcrc_steppable_vec(&mut blocks, &imax);
    add_to_rcrc_steppable_vec(&mut blocks, &one);
    add_to_rcrc_steppable_vec(&mut blocks, &clock);
    add_to_rcrc_steppable_vec(&mut blocks, &sixteen);
    add_to_rcrc_steppable_vec(&mut blocks, &square);
    add_to_rcrc_steppable_vec(&mut blocks, &izero);
    add_to_rcrc_steppable_vec(&mut blocks, &select);
    add_to_rcrc_steppable_vec(&mut blocks, &held_value);
    add_to_rcrc_steppable_vec(&mut blocks, &div_held_value_by_itwo);
    add_to_rcrc_steppable_vec(&mut blocks, &div_new_input_by_itwo);
    add_to_rcrc_steppable_vec(&mut blocks, &add);
    add_to_rcrc_steppable_vec(&mut blocks, &sample_and_hold);

    for _ in 0..511 {
      blocks.iter_mut().for_each(|b| b.borrow_mut().step());
      render(b'x', b'-', add.borrow_mut().output_value() as usize, imax.borrow_mut().output_value() as usize);
    }
  }
  Ok(())
}
