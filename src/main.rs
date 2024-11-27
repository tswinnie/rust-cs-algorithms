mod bitwise;
use bitwise::operators::{*};

fn main() {

    // let mut x: i32 = 7;
    // let mut y: i32 = 2;
    // // let result = !x;
    // // println!("{}", result);
    // let result = operator_shift_right(x, y);
    // println!("{}", result);

    let nums = vec![12, 12, 14, 90, 14, 14, 14 ];
    let res = find_odd_num(nums);
    println!("{:?}", res);

    let x = 6;
    let is_odd = is_num_odd(5);
    println!("{}", is_odd);





}
