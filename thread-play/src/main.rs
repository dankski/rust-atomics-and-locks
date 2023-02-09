use std::thread;

fn main() {

    thread::spawn(f);
    thread::spawn(f);

    println!("This is the main Thread");
}

fn f() {
    println!("Hello from another thread!");

    let id = thread::current().id();
    println!("This is my thread id: {id:?}");
}
