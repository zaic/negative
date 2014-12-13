pub mod vector;
pub mod map;
pub mod inner;

#[cfg(not(test))]
fn main() {
    println!("Hi, cargo");

    let mut v = vector::pers_vector::PersVector::<int>::new();
}
