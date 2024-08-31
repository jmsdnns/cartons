use std::convert::{From, Into};

#[derive(Debug, Copy, Clone)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

impl From<Number> for i64 {
    fn from(item: Number) -> i64 {
        item.value.into()
    }
}

#[derive(Debug)]
struct NumberSquared {
    value: i32,
}

impl From<Number> for NumberSquared {
    fn from(item: Number) -> Self {
        NumberSquared {
            value: item.value * item.value,
        }
    }
}

impl From<NumberSquared> for Number {
    fn from(item: NumberSquared) -> Self {
        Number {
            value: (item.value as f32).sqrt() as i32,
        }
    }
}

impl Copy for NumberSquared {}
impl Clone for NumberSquared {
    #[allow(clippy::non_canonical_clone_impl)]
    fn clone(&self) -> Self {
        Self { value: self.value }
    }
}

pub fn run() {
    let num = Number::from(30);
    println!("{:?}", num.value);

    let int: i64 = num.into();
    println!("{}", int);

    let numsqr = NumberSquared::from(num);
    println!("{:?}", &numsqr.value);

    let whoa: i64 = i64::from(Number::from(numsqr));
    println!("{}", whoa);
}
