fn main() {
    for i in 1..10 {
        let s = (1..10)
            .map(|k| format!("{:3}", i * k))
            .collect::<Vec<String>>().join(",");
        println!("{}", s);
    }
}