mod controller;

fn main() {
    // web api with multi threading 
    controller::get_main().unwrap();
    controller::post_main().unwrap();
}