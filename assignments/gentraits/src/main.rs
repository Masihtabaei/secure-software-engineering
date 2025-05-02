use num::PrimInt;

trait Collector<T: PrimInt> {
    fn collect(&mut self, item: T);
}

trait SumAdder<T> {
    fn sum(&mut self) -> T;
}

struct IntCollector<T: PrimInt> {
    data: Vec<T>,
}

impl<T: PrimInt> Collector<T> for IntCollector<T> {
    fn collect(&mut self, item: T) {
        self.data.push(item);
    }
}

impl<T: PrimInt> SumAdder<T> for IntCollector<T> {
    fn sum(&mut self) -> T {
        let mut result : T = T::zero();
        for item in &self.data {
            result = result + *item;
        }
        result
    }
}

fn main() {
    let mut c1 = IntCollector::<i8> { data: vec![] };
    c1.collect(1);
    c1.collect(-2);
    println!("{:?}", c1.data);

    let mut c2 = IntCollector::<i16> { data: vec![] };
    c2.collect(-10);
    c2.collect(20);
    println!("{:?}", c2.data);

    let mut c3 = IntCollector::<i32> { data: vec![] };

    let mut c4 = IntCollector::<i8> { data: vec![] };
    c4.collect(1);
    c4.collect(2);
    c4.collect(3);
    c4.collect(4);
    c4.collect(5);
    c4.collect(6);
    c4.collect(7);
    c4.collect(8);

    println!("Sum of c1: {}", c1.sum());
    println!("Sum of c2: {}", c2.sum());
    println!("Sum of c3: {}", c3.sum());
    println!("Sum of c4: {}", c4.sum());
}
