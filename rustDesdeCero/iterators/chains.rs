fn main() {
    /* let arr = [66, -8, 43, 19, 0, -31];
    let mut v = vec![];
    for i in 0..arr.len() {
        if arr[i] > 0 {
            v.push(arr[i]*2);
        }
    }
    println!("{:?}",v ); */

    let arr = [66, -8, 43, 19, 0, -31];

    /* let mut v = vec![];
    for n in arr
        .iter()
        .filter(|x| **x > 0)
        .map(|x| *x*2)
    {
            v.push(n);
    } */

    let v = arr
        .iter()
        .filter(|x| **x > 0)
        .map(|x| *x * 2)
        .collect::<Vec<_>>();

    println!("{:?}", v);
}
