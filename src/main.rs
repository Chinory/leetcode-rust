use std::cell::RefCell;
use std::mem;
use std::rc::Rc;

mod data;

fn main() {
  let mut a = RefCell::new(1usize);
  let n = mem::size_of_val(&a);
  print!("{:?}",n);
}
