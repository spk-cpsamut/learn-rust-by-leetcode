mod easy14;
mod easy20;
 mod array {
   pub mod easy26;
}

fn main() {
   // print!("{}", easy14::Solution::longest_common_prefix(["flow".to_owned(), "flower".to_owned(), "flo".to_owned()].to_vec()));

   // print!("{}", easy20::Solution::is_valid("[[]][".to_string()));

   print!("{}", array::easy26::Solution::remove_duplicates(&mut [1,1,1,2,3,4,4,5,6].to_vec()));
}
