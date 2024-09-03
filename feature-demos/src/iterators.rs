struct Backpack {
    things: Vec<String>,
}

impl Backpack {
    fn iter(&self) -> BackpackIterator {
        BackpackIterator {
            index: 0,
            stuff: self,
        }
    }
}

pub struct BackpackIterator<'a> {
    index: usize,
    stuff: &'a Backpack,
}

impl Iterator for BackpackIterator<'_> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.stuff.things.len() {
            let thing = Some(self.stuff.things[self.index].clone());
            self.index += 1;
            return thing;
        }

        None
    }
}

pub fn run() {
    let s = Backpack {
        things: vec![
            "laptop".to_string(),
            "headphones".to_string(),
            "hoodie".to_string(),
            "drum sticks".to_string(),
        ],
    };

    for t in s.iter() {
        println!("{}", t);
    }
}
