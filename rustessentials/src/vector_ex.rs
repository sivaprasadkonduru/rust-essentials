// This file will have the vector details

pub fn vectors() {

    // Different ways of creating a vector
    let mut vec = vec![];
    let mut vec1 = Vec::new();
    let mut vec2 = vec!["hello", " welcome ", "to ", "rust"];

    vec.push(10);
    vec.push(20);
    vec1.push(100);
    vec1.push(200);

    println!("Vector has elements: {:?}", vec);
    println!("Vector1 has elements: {:?}", vec1);
}