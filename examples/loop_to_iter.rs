fn main() {
    let mut i = 0;
    for x in 0..5 {
        for y in x * 100..x * 110 {
            if (i + y) % 3 == 0 {
                println!("{}:{}", i, y);
            }
            i += 1;
        }
    }

    (0..5)
        .flat_map(|x| (x * 100..x * 110).enumerate()
            .map(|(i, y)| { (i, y) }))
        .filter(|(i, y)| (i + y) % 3 == 0)
        .for_each(|(i, y)| println!("{}:{}", i / 3, y));
}
