fn main() {
    let mut arr = [5, -4, 9, 0, -7, -1, 3, 5, 3, 1];
    double_negatives(&mut arr);
    println!("{:?}",arr);
}

fn double_negatives(arr: &mut [i32; 10]) {
    for i in 0..10 {
        if (*arr)[i] < 0 { //arr[i] < 0 { a[i] *= 2;}
            (*arr)[i] *= 2;
        }
    }
}
