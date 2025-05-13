use std::fs;
fn main(){
    let content = read_file("assets/juventus.csv");
    println!("{}",content);
}

fn read_file(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("Error")
}