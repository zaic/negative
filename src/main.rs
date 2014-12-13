pub mod vector;
pub mod map;

#[cfg(not(test))]
fn main() {
    println!("Hi, cargo");

    let mut v = vector::pers_vector::PersVector::<int>::new();
}
