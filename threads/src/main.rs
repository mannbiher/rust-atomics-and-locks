use std::thread;

fn main() {
    let t1 = thread::spawn(f);
    let t2 = thread::spawn(f);
    println!("Hello from the main thread.!");
    t1.join().unwrap();
    t2.join().unwrap();
    let numbers = vec![1, 2, 3];

    // it seems in current version move is not required
    thread::spawn(|| {
        for n in numbers {
            println!("{n}");
        }
    })
    .join()
    .unwrap();
    thread_result();
    scoped_threads();
    scoped_threads_error();
}

fn f() {
    println!("Hello from another thread!");
    let id = thread::current().id();

    println!("This is my thread id: {id:?}");
}

fn thread_result() {
    let numbers = Vec::from_iter(0..=1000);

    let t = thread::spawn(|| {
        let len = numbers.len();
        let sum = numbers.into_iter().sum::<usize>();
        sum / len
    });

    let average = t.join().unwrap();

    println!("average: {average}");
}

fn scoped_threads() {
    let numbers = vec![1, 2, 3];

    thread::scope(|s| {
        s.spawn(|| {
            println!("length: {}", numbers.len());
        });
        s.spawn(|| {
            for n in &numbers {
                println!("{n}");
            }
        });
    });
}

fn scoped_threads_error() {
    let mut numbers = vec![1, 2, 3];

    thread::scope(|s| {
        s.spawn(|| {
            numbers.push(1);
        });
        s.spawn(|| {
            // numbers.push(2); // Error!
            println!("Comment update to remove error");
        });
    });
}
