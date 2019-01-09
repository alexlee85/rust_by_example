extern crate futures;
extern crate futures_cpupool;

use futures::Future;
use futures_cpupool::CpuFuture;
use futures_cpupool::CpuPool;

// prime numbers generator http://www.numberempire.com/primenumbers.php
const BIG_PRIME: u64 = 1112298787;

fn is_prime(num: u64) -> bool {
    for i in 2..num {
        if num % i == 0 {
            return false;
        }
    }

    true
}

fn prime_future(num: u64) -> CpuFuture<bool, ()> {
    let pool = CpuPool::new_num_cpus();

    let result = pool.spawn_fn(move || {
        let prime = is_prime(num);
        let res: Result<bool, ()> = Ok(prime);

        res
    });

    result
}

fn main() {
    let f = prime_future(BIG_PRIME).map(|x| {
        if x {
            println!("Future Prime")
        } else {
            println!("Future Not Prime")
        }
    });

    // await here, will take double time. cause future wait will block the code.
    // f.wait().unwrap();

    if is_prime(BIG_PRIME) {
        println!("Prime");
    } else {
        println!("Not Prime");
    }

    // await here, will take less time.
    f.wait().unwrap();
}
