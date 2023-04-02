extern crate std_semaphore;

use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;
use std_semaphore::Semaphore;

const NRO_FILOSOFOS: i32 = 5;

fn main() {

    let chopsticks:Arc<Vec<Semaphore>> = Arc::new((0 .. NRO_FILOSOFOS)
        .map(|_| Semaphore::new(1))
        .collect());

    let table: Arc<Semaphore> = Arc::new(Semaphore::new(1));

    let mut handles = Vec::with_capacity((NRO_FILOSOFOS) as usize);
    for i in 0..NRO_FILOSOFOS {
        handles.push(nuevo_filosofo(i as usize, chopsticks.clone(),table.clone()));
    }
    for handle in handles {
        handle.join().expect("Error en handle al realizar el join");
    }
}

fn nuevo_filosofo(i:usize, chopsticks: Arc<Vec<Semaphore>>, table: Arc<Semaphore>) -> JoinHandle<()> {
    return thread::spawn(move || {philosopher_eats(i,chopsticks, table)});
}

fn philosopher_eats(i:usize, chopsticks :Arc<Vec<Semaphore>>, table: Arc<Semaphore>) {
    let mut j = i +1;
    if i == 4 {
        j = 0
    }
    loop {
        table.acquire();
        println!("Soy el filosofo {}, espero palillo {}", i, i);
        chopsticks[i].acquire();
        println!("Soy el filosofo {}, espero palillo {}", i, j);
        chopsticks[j].acquire();
        println!("Soy el filosofo {}. voy a comer", i);
        chopsticks[i].release();
        chopsticks[j].release();
        println!("Soy el filosofo {}. Libere mis palillos", i);
        table.release();
    }
}
