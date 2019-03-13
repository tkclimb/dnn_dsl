#[macro_use]
extern crate dnn_dsl;

use dnn_dsl::ast::{Source, Var};
use dnn_dsl::llvm::initialise_llvm_jit;
#[warn(unused_imports)]
use std::{
  collections::HashMap,
  fs::{create_dir_all, File},
  io::{Result, Write},
  path::{Path, PathBuf},
};

pub fn main() -> () {
  initialise_llvm_jit();
  run_blur(Path::new("./out"))
}

fn run_blur(base_dir: &Path) -> () {
  let (x, y) = (Var::X, Var::Y);
  let input = Source::new("input");
  // let blur_h = input.at(x, y);
  // let blur_h = input.at(x - 1, y);
  let blur_h = (input.at(x - 1, y) + input.at(x, y) + input.at(x + 1, y)) / 3;
  let blur_v = (blur_h.at(x, y - 1) + blur_h.at(x, y) + blur_h.at(x, y + 1)) / 3;
  ()
  // func!(blur_h = (input.at(x - 1, y) + input.at(x, y) + input.at(x + 1, y)) / 3);
  // func!(blur_v = (blur_h.at(x, y - 1) + blur_h.at(x, y) + blur_h.at(x, y + 1)) / 3);
  // let mut sched = Schedule::new();
  // // TODO: remove the distinction between Func and Source in most parts of the code
  // sched.add_source(&input, FuncSchedule::by_row());
  // sched.add_func(&blur_h, FuncSchedule::by_column());
  // sched.add_func(&blur_v, FuncSchedule::by_row());
  // let graph = Graph::new("blur3x3", vec![blur_h, blur_v], sched);

  // compile_and_run(
  //     base_dir,
  //     &graph,
  //     &[(&input, &example_image(6, 6))],
  //     &HashMap::new())
}
