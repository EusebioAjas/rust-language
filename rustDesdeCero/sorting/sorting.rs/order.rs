use std::cmp::Ordering;
fn main(){
    let mut arr = [4, 8, 1, 10, 0, 45, 12, 7];
    //arr.sort();
    println!("{:?}",arr);

    //arr.sort_by(desc);
    //println!("{:?}",arr);

    //Closure
    // let desc = |a: &i32, b: &i32| -> Ordering {
    //     if a < b {
    //         Ordering::Greater
    //     }
    //     else if a > b {
    //         Ordering::Less
    //     }
    //     else {
    //         Ordering::Equal
    //     }    
    // };
   
    arr.sort_by(|a, b| 
        if a < b { Ordering::Greater }
        else if a > b { Ordering::Less }
        else { Ordering::Equal }    
    );
   
    //arr.sort_by(|a, b| b.cmp(a));
    println!("{:?}",arr);
}

/* fn desc(a: &i32, b: &i32) -> Ordering {
    if a < b {Ordering::Greater}
    else if a > b{Ordering::Less}
    else {Ordering::Equal}
}
 */