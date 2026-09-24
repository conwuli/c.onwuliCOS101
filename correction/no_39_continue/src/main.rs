fn main() {
    let mut count = 0;
    for i in 1..11 {
        if i % 2 == 0 {
            continue;
        }
        count = count + 1;
    }
   println!("{:?}", count );
}
