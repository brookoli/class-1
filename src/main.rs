fn main(){

}

//==========================================================================
fn sort(arr : &mut [i32], comp: fn(&i32, &i32)->bool) {
    let mut swapped;
    loop {
        swapped = false;
        for i in 0..arr.len() - 1 {
            if comp(&arr[i], &arr[i + 1]) {
                arr.swap(i, i + 1);
                swapped = true;
            }
        }

        // If no swaps were made during the iteration, the array is sorted.
        if !swapped {
            break;
        }
    }
}

#[test]
fn sort_test() {
    let mut arr = [1, 5, 2, 3, 4, 6];
    sort(&mut arr, |a, b| a > b);
    assert_eq!(arr, [1, 2, 3, 4, 5, 6]);
    // println!("{:?}", arr);
}

//==========================================================================
fn is_power_of_two(n: i32) -> bool { 
    return n & (n - 1) == 0
}

#[test]
fn power28_test() {
    let num = 28;
    assert_eq!(is_power_of_two(num), false);
}

#[test]
fn power1_test() {
    let num = 1;
    assert_eq!(is_power_of_two(num), true);
}

//==========================================================================
fn reverse_bits(x: u32) -> u32 {
    let mut reversed_x: u32 = 0;
    let mut val:u32 = 0;
    let mut tmp:u32;
    while val < 32 
    {
        tmp = x & (1 << val);
        if tmp > 0
        {
            reversed_x = reversed_x | (1 << ((32 - 1) - val));
        }
        val = val + 1;
    }
    return reversed_x;
}

#[test]
fn reverse_test() {
    let n: u32 = 43261596;
    assert_eq!(reverse_bits(n), 964176192);
}
    