use thp::RThreadPool;

mod queue;
mod thp;

fn main() {
    let rthp = Box::new(RThreadPool::new(2));

    RThreadPool::execute(&rthp);

    loop {
        rthp.submit(1);
    }
}
