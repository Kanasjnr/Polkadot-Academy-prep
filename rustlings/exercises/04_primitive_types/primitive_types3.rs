fn main() {
    // TODO: Create an array called `a` with at least 100 elements in it.
    let a = [1; 100];

    if a.len() >= 100 {
        println!("{}", a.len());
        println!("Wow, that's a big array!");
    } else {
        println!("{}", a.len());
        println!("{}", a[0]);
        println!("Meh, I eat arrays like that for breakfast.");
        // panic!("Array not big enough, more elements needed");
    }
}
