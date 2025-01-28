let val: i32 = 10;
let r1: &i32 = &val;
let val2: i32 = *r1;
println!("Dereferenced value: {val2}")
