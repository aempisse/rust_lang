fn main() {
    let mut s1 = String::from("hello");

    change(&mut s1);

    let len = calculate_length(&mut s1);
    
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &mut String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.

fn change(s: &mut String) {
    s.push_str(", world!");
}
