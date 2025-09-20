pub fn main() {
    let mut path: String = "".to_string();
    println!("Please paste the path to your file");
    std::io::stdin().read_line(&mut path).expect("And this somehow broke...");
    path = path.trim().to_string();

    
}