use num::PrimInt;

trait Collector<T: PrimInt> {
  fn collect(&mut self, item: T);
}

struct IntCollector<T: PrimInt> {
  data: Vec<T>,
}

impl<T: PrimInt> Collector<T>
    for IntCollector<T> {
  fn collect(&mut self, item: T) {
    self.data.push(item);
  }
}

fn main() {
  let mut c1 = IntCollector::<i8>
      { data: vec![] };
  c1.collect(1);
  c1.collect(-2);
  println!("{:?}", c1.data);

  let mut c2 = IntCollector::<i16> { data: vec![] };
  c2.collect(-10);
  c2.collect(20);
  println!("{:?}", c2.data);
}