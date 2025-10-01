// Queue Implementation in Rust
use text_io::read;

fn enque(queue: [i32; 10], front: i32, rear: i32, element: i32) {}

fn deque(queue: [i32; 10], front: i32, rear: i32) -> i32 {
    return 0;
}

fn display(queue: [i32; 10]) {
    for i in queue {
        println!("{}", i);
    }
}

fn main_loop(queue: &[i32; 10], front: &i32, rear: &i32) {
    let mut ch: i32 = 0;
    while ch != -1 {
        println!("-----------");
        println!("Queue Implementation");
        println!("-----------");
        println!("1. Enque");
        println!("2. Deque");
        println!("3. Display");
        println!("4. Exit");
        println!("-----------");
        println!("Enter your choice: ");
        ch = read!();
        match ch {
            1 => {
                println!("Enter a number to enque: ");
                let ele: i32 = read!();
                enque(*queue, *front, *rear, ele);
                println!("The element {} has been enqueued!", ele);
            }
            2 => {}
            3 => {}
            4 => {
                ch = -1;
            }
            _ => {}
        }
    }
}

fn main() {
    let mut queue: [i32; 10] = [-1; 10];
    let mut front = -1;
    let mut rear = -1;

    main_loop(&queue, &front, &rear);
}
