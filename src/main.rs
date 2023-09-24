use std::thread;
use std::time::Duration;
fn enqueue<T>(data: &mut Vec<T>, element: T) {
    data.push(element);
}

fn dequeue<T>(data: &mut Vec<T>) {
    data.remove(0);
}

fn print_queue<T: std::fmt::Debug>(data: &Vec<T>) {
    dbg!(data);
}

fn fill_queue(data: &mut Vec<u32>) {
    for index in 1..100 {
        enqueue(data, index);
    }
}

fn manage_thread(data: &mut Vec<u32>) {
    for index in 1..100 {
        dequeue(data);
        print_queue(&data);
        thread::sleep(Duration::from_secs(1));
    }
}

fn main() {
    let mut data: Vec<u32> = Vec::new();
    fill_queue(&mut data);
    manage_thread(&mut data);
}
