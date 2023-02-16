fn main() {
    assert_eq!(vec!(4,5,5), add_to_array_form(vec!(2,7,4), 181));
    assert_eq!(vec!(1,0,0,0,0,0,0,0,0,0,0), add_to_array_form(vec!(9,9,9,9,9,9,9,9,9,9), 1));
    println!("pass");
}

fn add_to_array_form(num: Vec<i32>, k: i32) -> Vec<i32> {
    let mut k = k;
    let mut out = Vec::<i32>::new();

    let mut acc = 0;
    let mut len = num.len();
    while len > 0 {
        let d = num[len - 1] + k % 10 + acc;

        out.push(d % 10);

        len = len - 1;
        acc = d / 10;
        k = k / 10;
    }

    k = k + acc;
    while k > 0 {
        out.push(k % 10);
        k = k / 10;
    }

    out.reverse();
    return out;
}

// Failed cases
// num = [9,9,9,9,9,9,9,9,9,9]
// k = 1
// Output
// [1,4,1,0,0,6,5,4,0,8]
// Expected
// [1,0,0,0,0,0,0,0,0,0,0]]
//
//fn add_to_array_form(num: Vec<i32>, k: i32) -> Vec<i32> {
//
//    fn to_num(num: Vec<i32>) -> i32 {
//
//        use std::convert::TryInto;
//
//        let mut out: i32 = 0;
//        let mut w: u32 = num.len().try_into().unwrap();
//        for i in num {
//            out = out + i * 10_i32.pow(w - 1);
//            w = w - 1
//        }
//        return out
//    }
//
//    fn to_array(n: i32) -> Vec<i32> {
//        let mut vec: Vec<i32> = Vec::new();
//        let mut n = n;
//
//        while n > 0 {
//            vec.push(n % 10);
//            n = n / 10;
//        }
//        return vec
//    }
//
//    let out = to_num(num) + k;
//    let mut out = to_array(out);
//    out.reverse();
//    return out
//}
