extern crate rayon;
use rayon::prelude::*;

extern crate num;
use num::bigint::BigInt;
use num::{FromPrimitive, One};




fn main() {
    let start = BigInt::from_u32(0).unwrap();
    let end = BigInt::from_u32(10).unwrap();
    let range = num::range(start, end);
    let res = range
                   .into_iter()
                   //.into_par_iter()
                   .map(|x| x + BigInt::one())
                   .map(|x| BigInt::to_str_radix(&x, 10))
                   .collect::<Vec<String>>();
    println!("Result: {:?}", res);
}
