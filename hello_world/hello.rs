fn main() {
    println!("Hello World!");

    println!("{} days", 31);

    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    println!("{hoge} {piyo} {piyo} {fuga}", hoge = "ほげ", piyo = "🐤", fuga = 42);

    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);

    println!("{number:>0width$}", number = 123, width = 6);

    //    println!("This struct `{}` won't print...", Structure(3));

    let pi = 3.141592;
    println!("Pi is roughly {pi:.prec$}", prec = 3, pi = pi);
}
