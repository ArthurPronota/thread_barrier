use std::sync::{Arc, Barrier} ;
use std::thread ;

fn main() {
    let b = Arc::new(
            // Создаёт новый барьер, способный блокировать 
            // определённое количество потоков.
            Barrier::new(3)
        ) ;

    let mut h = vec![] ;

    for i in 0..3 {
        let b = b.clone() ;
        h.push(
            thread::spawn(move || {
                // Вывод гарантирует, что все before напечатаются до 
                // любого after.
                println!("Thread: {i}, before wait.") ;
                // Блокирует текущий поток до тех пор, пока все потоки 
                // не встретятся здесь.                
                b.wait() ;
                println!("Thread: {i}, after wait") ;
            })
        );
    }

    // ожидание завершения всех потоков
    for t in h {
        t.join().unwrap() ;
    }
}
