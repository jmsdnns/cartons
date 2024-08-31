use ndarray::prelude::*;
use ndarray::Array;
use ndarray::{concatenate, stack, Axis};
use std::f64::INFINITY as inf;

fn main() {
    let a = array![[10.,20.,30.,40.,]];
    // [[10., 20., 30., 40.]]
    println!("a {:?}", &a);
    // [1, 4]
    println!("a shape {:?}", &a.shape());

    let b = Array::range(0., 4., 1.);
    // [0., 1., 2., 3.]
    println!("b {:?}", &b);
    // [4]
    println!("b shape {:?}", &b.shape());

    assert_eq!(&a + &b, array![[10., 21., 32., 43.]]);
    assert_eq!(&a - &b, array![[10., 19., 28., 37.]]);
    assert_eq!(&a * &b, array![[0., 20., 60., 120.]]);
    assert_eq!(&a / &b, array![[inf, 20., 15., 13.333333333333334]]);

    let b = b.into_shape_with_order((4,1)).unwrap();
    // [[0.],[1.],[2.],[3.]]
    println!("b reshaped {:?}", &b);
    // [4, 1]
    println!("b reshaped shape {:?}", &b.shape());
    // [[200]]
    println!("{}", a.dot(&b));
    // [[0, 10, 20, 30],
    //  [0, 20, 40, 60],
    //  [0, 30, 60, 90],
    //  [0, 40, 80, 120]]
    println!("{}", a.t().dot(&b.t()));

    // INDEXING, SLICING, ITERATING

    let c = Array::range(0., 10., 1.);
    let mut c = c.mapv(|a: f64| a.powi(3));
    // [0, 1, 8, 27, 64, 125, 216, 343, 512, 729]
    println!("{}", c);
    // 8
    println!("{}", c[[2]]);
    // 8
    println!("{}", c.slice(s![2]));
    // [8, 27, 64]
    println!("{}", c.slice(s![2..5]));

    c.slice_mut(s![..6;2]).fill(1000.);
    // [1000, 1, 1000, 27, 1000, 125, 216, 343, 512, 729]
    println!("{}", c);

    for i in c.iter() {
        // 9.999999999999998,
        // 1,
        // 9.999999999999998,
        // 3,
        // 9.999999999999998,
        // 4.999999999999999,
        // 5.999999999999999,
        // 6.999999999999999,
        // 7.999999999999999,
        // 8.999999999999998,
        println!("{}, ", i.powf(1./3.));
    }

    // STACKING / CONCATENATING

    let d = array![
        [3., 7., 8.],
        [5., 2., 4.],
    ];

    let e = array![
        [1., 9., 0.],
        [5., 4., 1.],
    ];

    // [[[3.0, 7.0, 8.0],
    //   [5.0, 2.0, 4.0]],
    //  [[1.0, 9.0, 0.0],
    //   [5.0, 4.0, 1.0]]]
    println!("{:?}", stack![Axis(0), d, e]);

    // [[[3.0, 7.0, 8.0],
    //   [1.0, 9.0, 0.0]],
    //  [[5.0, 2.0, 4.0],
    //   [5.0, 4.0, 1.0]]]
    println!("{:?}", stack![Axis(1), d, e]);

    // [[[3.0, 1.0],
    //   [7.0, 9.0],
    //   [8.0, 0.0]],
    //  [[5.0, 5.0],
    //   [2.0, 4.0],
    //   [4.0, 1.0]]]
    println!("{:?}", stack![Axis(2), d, e]);

    // [[3.0, 7.0, 8.0],
    //  [5.0, 2.0, 4.0],
    //  [1.0, 9.0, 0.0],
    //  [5.0, 4.0, 1.0]]
    println!("{:?}", concatenate![Axis(0), d, e]);

    // [[3.0, 7.0, 8.0, 1.0, 9.0, 0.0],
    //  [5.0, 2.0, 4.0, 5.0, 4.0, 1.0]]
    println!("{:?}", concatenate![Axis(1), d, e]);

    // SPLITTING

    let f = array![
        [6., 7., 6., 9., 0., 5., 4., 0., 6., 8., 5., 2.],
        [8., 5., 5., 7., 1., 8., 6., 7., 1., 8., 1., 0.]];

    let (s1, s2) = f.view().split_at(Axis(0), 1);
    // [[6, 7, 6, 9, 0, 5, 4, 0, 6, 8, 5, 2]]
    println!("{}", s1);
    // [[8, 5, 5, 7, 1, 8, 6, 7, 1, 8, 1, 0]]
    println!("{}", s2);

    let (s1, s2) = f.view().split_at(Axis(1), 4);
    // [[6, 7, 6, 9],
    //  [8, 5, 5, 7]]
    println!("{}", s1);
    // [[6, 7, 6, 9],
    //  [8, 5, 5, 7]]
    println!("{}", s2);

    // BROADCASTING

    let g = array![
        [1., 1.],
        [1., 2.],
        [0., 3.],
        [0., 4.]
    ];

    let h = array![[0., 1.]];

    let i = array![
        [1., 2.],
        [1., 3.],
        [0., 4.],
        [0., 5.]
    ];

    assert!(i == g + h);

    let j = array![
        [1., 2.],
        [3., 4.],
    ];

    let k = j.broadcast((3, 2, 2)).unwrap();
    // [2, 2]
    println!("{:?}", j.shape());
    // [[[1, 2],
    //   [3, 4]],
    //  [[1, 2],
    //   [3, 4]],
    //  [[1, 2],
    //   [3, 4]]]
    println!("{}", k);
}
