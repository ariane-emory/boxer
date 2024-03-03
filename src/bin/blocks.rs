#![allow(unreachable_code)]
//#![allow(unused_variables)]
//#![allow(unused_imports)]
//#![allow(unused_mut)]
//#![allow(dead_code)]

use boxer::block::*;
use std::cell::RefCell;
use std::io::{self};
use std::rc::Rc;


////////////////////////////////////////////////////////////////////////////////
const MAX: usize = 1 << 6;
const STEPS: usize = MAX << 2;
const LOOP: bool = false;


////////////////////////////////////////////////////////////////////////////////
fn perform_steps<T: Copy + std::fmt::Debug>(
  steps: usize,
  blocks: &Vec<SteppableRef>,
  ctr: &Rc<RefCell<impl HasOutputSignal<T>>>,
  max: &Rc<RefCell<impl HasOutputSignal<usize>>>,
) where
  T: TryInto<usize>, {
  for _ in 0..steps {
    blocks.iter().for_each(|b| b.step());

    if let Ok(us) = ctr.output_value().try_into() {
      render(b'x', b'-', us, max.output_value());
    } else {
      panic!("Error converting output value to usize.");
    }
  }
}


////////////////////////////////////////////////////////////////////////////////
fn main() -> io::Result<()> {
  loop {
    {
      println!("\nSawtooth:");

      let one = Value::new(1).as_rcrc();
      let max = Value::new(MAX).as_rcrc();
      let clock = SquareWave::new(&one.output()).as_rcrc();
      let ctr_reset = Feedback::new().as_rcrc();
      let ctr_max = Value::new(MAX).as_rcrc();
      let ctr =
        UpCounter::new(&clock.output(), &ctr_reset.output(), &ctr_max.output())
          .as_rcrc();

      ctr_reset.borrow_mut().set_input(&ctr.borrow().at_max());

      let mut blocks: Vec<SteppableRef> = Vec::new();
      push_onto_vec_of_rcrc_steppable(&mut blocks, &clock);
      push_onto_vec_of_rcrc_steppable(&mut blocks, &ctr_reset);
      push_onto_vec_of_rcrc_steppable(&mut blocks, &ctr_max);
      push_onto_vec_of_rcrc_steppable(&mut blocks, &ctr);

      perform_steps(STEPS, &blocks, &ctr, &max);
    }

    {
      println!("\nSawtooth PWM:");

      let one = Value::new(1).as_rcrc();
      let max = Value::new(MAX).as_rcrc();
      let clock = SquareWave::new(&one.output()).as_rcrc();
      let ctr_reset = Feedback::new().as_rcrc();
      let ctr_max = Value::new(MAX).as_rcrc();
      let ctr =
        UpCounter::new(&clock.output(), &ctr_reset.output(), &ctr_max.output())
          .as_rcrc();
      let add = Add::new(&ctr.output(), &one.output()).as_rcrc();
      let two = Value::new(2).as_rcrc();
      let div = Div::new(&add.output(), &two.output()).as_rcrc();
      let square = SquareWave::new(&div.output()).as_rcrc();
      let zero = Value::new(0).as_rcrc();
      let select =
        Select::new(&square.output(), &zero.output(), &max.output()).as_rcrc();

      ctr_reset.borrow_mut().set_input(&ctr.borrow().at_max());

      let mut blocks: Vec<SteppableRef> = Vec::new();
      push_onto_vec_of_rcrc_steppable(&mut blocks, &clock);
      push_onto_vec_of_rcrc_steppable(&mut blocks, &ctr_reset);
      push_onto_vec_of_rcrc_steppable(&mut blocks, &ctr_max);
      push_onto_vec_of_rcrc_steppable(&mut blocks, &ctr);
      push_onto_vec_of_rcrc_steppable(&mut blocks, &add);
      push_onto_vec_of_rcrc_steppable(&mut blocks, &div);
      push_onto_vec_of_rcrc_steppable(&mut blocks, &square);
      push_onto_vec_of_rcrc_steppable(&mut blocks, &select);

      perform_steps(STEPS, &blocks, &select, &max);
    }

    {
      println!("\nTriangle:");

      let max = Value::new(MAX).as_rcrc();
      let one = Value::new(1).as_rcrc();
      let clock = SquareWave::new(&one.output()).as_rcrc();
      let ctr_reset = Feedback::new().as_rcrc();
      let ctr =
        UpCounter::new(&clock.output(), &ctr_reset.output(), &max.output())
          .as_rcrc();
      let sr_set = Feedback::new().as_rcrc();
      let sr_reset = Feedback::new().as_rcrc();
      let sr = SRLatch::new(&sr_set.output(), &sr_reset.output()).as_rcrc();
      let not_latched = Not::new(&sr.output()).as_rcrc();
      let ctr_at_max_and_latched =
        And::new(ctr.borrow().at_max(), &sr.output()).as_rcrc();
      let ctr_at_max_and_not_latched =
        And::new(ctr.borrow().at_max(), &not_latched.output()).as_rcrc();
      let sub = Sub::new(&max.output(), &ctr.output()).as_rcrc();
      let select =
        Select::new(&sr.output(), &ctr.output(), &sub.output()).as_rcrc();

      ctr_reset.borrow_mut().set_input(&ctr.borrow().at_max());
      sr_set
        .borrow_mut()
        .set_input(&ctr_at_max_and_not_latched.output());
      sr_reset
        .borrow_mut()
        .set_input(&ctr_at_max_and_latched.output());

      let mut blocks: Vec<SteppableRef> = Vec::new();
      push_onto_vec_of_rcrc_steppable(&mut blocks, &clock);
      push_onto_vec_of_rcrc_steppable(&mut blocks, &ctr_reset);
      push_onto_vec_of_rcrc_steppable(&mut blocks, &ctr);
      push_onto_vec_of_rcrc_steppable(&mut blocks, &sr_set);
      push_onto_vec_of_rcrc_steppable(&mut blocks, &sr_reset);
      push_onto_vec_of_rcrc_steppable(&mut blocks, &sr);
      push_onto_vec_of_rcrc_steppable(&mut blocks, &not_latched);
      push_onto_vec_of_rcrc_steppable(&mut blocks, &ctr_at_max_and_latched);
      push_onto_vec_of_rcrc_steppable(&mut blocks, &ctr_at_max_and_not_latched);
      push_onto_vec_of_rcrc_steppable(&mut blocks, &sub);
      push_onto_vec_of_rcrc_steppable(&mut blocks, &select);

      perform_steps(STEPS, &blocks, &select, &max);
    }

    {
      println!("\nSharktooth:");

      let max = Value::new(MAX).as_rcrc();
      let imax = Value::<isize>::new(max.output_value() as isize).as_rcrc();
      let one = Value::new(1).as_rcrc();
      let clock = SquareWave::new(&one.output()).as_rcrc();
      let sixteen = Value::new(16).as_rcrc();
      let square = SquareWave::new(&sixteen.output()).as_rcrc();
      let izero = Value::new(0).as_rcrc();
      let select =
        Select::new(&square.output(), &izero.output(), &imax.output())
          .as_rcrc();
      let held_value = Feedback::<isize>::new().as_rcrc();
      let never = Value::new(false).as_rcrc();
      let itwo = Value::<isize>::new(2).as_rcrc();
      let div_held_value_by_itwo =
        Div::<isize>::new(&held_value.output(), &itwo.output()).as_rcrc();
      let div_new_input_by_itwo =
        Div::<isize>::new(&select.output(), &itwo.output()).as_rcrc();
      let add = Add::new(
        &div_held_value_by_itwo.output(),
        &div_new_input_by_itwo.output(),
      )
      .as_rcrc();
      let sample_and_hold =
        SampleAndHold::new(&add.output(), &clock.output(), &never.output())
          .as_rcrc();

      held_value.borrow_mut().set_input(&sample_and_hold.output());

      let mut blocks: Vec<SteppableRef> = Vec::new();
      push_onto_vec_of_rcrc_steppable(&mut blocks, &clock);
      push_onto_vec_of_rcrc_steppable(&mut blocks, &square);
      push_onto_vec_of_rcrc_steppable(&mut blocks, &select);
      push_onto_vec_of_rcrc_steppable(&mut blocks, &held_value);
      push_onto_vec_of_rcrc_steppable(&mut blocks, &div_held_value_by_itwo);
      push_onto_vec_of_rcrc_steppable(&mut blocks, &div_new_input_by_itwo);
      push_onto_vec_of_rcrc_steppable(&mut blocks, &add);
      push_onto_vec_of_rcrc_steppable(&mut blocks, &sample_and_hold);

      perform_steps(STEPS, &blocks, &add, &max);
    }

    if !LOOP {
      break;
    }
  }
  Ok(())
}


////////////////////////////////////////////////////////////////////////////////
fn render(
  char: u8,
  char2: u8,
  signal: usize,
  width: usize,
) {
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
