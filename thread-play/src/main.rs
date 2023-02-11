use std::thread;

fn main() {

    let t1 = thread::spawn(f);
    let t2 = thread::spawn(f);

    println!("This is the main Thread");

    t1.join().unwrap();
    t2.join().unwrap();

}

fn f() {
    println!("Hello from another thread!");

    let id = std::thread::current().id();
    println!("This is my thread id: {id:?}");
}
