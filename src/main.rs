fn main(){
    
}

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