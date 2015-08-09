use std::thread;

// When you create a Rust library, it changes the name of the function in the compiled output. The reasons for this are outside the scope of this tutorial, but in order for other languages to know how to call the function, we can’t do that. This attribute turns that behavior off
#[no_mangle]
// The pub means that this function should be callable from outside of this module, and the extern says that it should be able to be called from C.
pub extern fn process() {
    let handles: Vec<_> = (0..10).map(|_| {
        thread::spawn(|| {
            let mut x = 0;
            for _ in (0..5_000_000) {
                x += 1
            }
        x
        })
    }).collect();
    
    for h in handles {
        println!("Thread finished with count={}",
        h.join().map_err(|_| "Could not join a thread!").unwrap());
    }    

    println!("done!");
}