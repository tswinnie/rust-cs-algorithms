pub mod operators {

    pub fn add(x: i32, y: i32) -> i32 {
        x + y
    }

    //Takes 2 numbers as inputs and does an AND on each bit of each binary number
    pub fn operator_and(x: i32, y: i32) -> i32 {
        return x & y;
    }

    pub fn operator_xor(x: i32, y: i32) -> i32 {
        return x ^ y;
    }

    pub fn operator_or(x: i32, y: i32) -> i32 {
        return x | y;
    }

    pub fn operator_not(x: i32) -> i32 {
        return !x;
    }

    pub fn operator_shift_left(x: i32, y: i32) -> i32 {
        return x << y;
    }

    pub fn operator_shift_right(x: i32, y: i32) -> i32 {
        return x >> y;
    }

    pub fn find_odd_num(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        for num in nums {
            res ^= num;
        }
        return res;
    }

    pub fn is_num_odd(x: i32) -> bool {
        let mut is_odd = false;
        if x & 1 == 1 {
            is_odd = true;
        }
        return is_odd;
    }


}