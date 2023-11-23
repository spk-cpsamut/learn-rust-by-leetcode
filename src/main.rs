mod easy14;
mod easy20;
 mod array {
   pub mod easy26;
   pub mod easy27;
   pub mod easy35;
   pub mod meduim11;
}

mod hash_table {
   pub mod easy169;
}

mod algo_monster {
   pub mod two_pointers {
      pub mod remove_duplicates;
   }
}

fn main() {

   array::easy35::Solution::run_units_test_easy_35();
   array::meduim11::Solution::run_test_cases();
   hash_table::easy169::Solution::run_test_cases();
   algo_monster::two_pointers::remove_duplicates::Solution::run_test_cases();
}
