fn main() {
   // array
    let l1: [i32; 3]  = [1, 2, 3];
    let l2: [i32; 1000] = [0; 1000];

    println!("{:?}", l1);

    let i: i32 = l1[0];

    let [x,yi32,z] = l1;
    // slice
    let l3: &[i32] = &l1[0..2];
    println!("{:?}", l3);
}
