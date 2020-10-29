use tree_buf::prelude::*;
// use tree_buf::experimental::*;

fn main() {
    
    println!("Loading data...");
    let defense_data = boss::cache::load_defense();

    println!("Converting to treebuf...");
    let bytes = write(&defense_data);

    println!("Checking file integrity...");
    let copy: Vec<boss::defense::Defense> = read(&bytes).unwrap();
    assert_eq!(&copy, &defense_data);

    println!("Writing to disk...");
    let file_name = "F:\\Baseball\\defense.tbf";

    println!("Debugging...");
    // let tree = tree_buf::internal::read_root(&bytes);
    // dbg!(tree.unwrap());

    let sizes = tree_buf::experimental::stats::size_breakdown(&bytes);
    println!("{}", sizes.unwrap());

    std::fs::write(file_name, bytes).unwrap();



}