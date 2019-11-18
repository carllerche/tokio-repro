fn main() {
    const NUM_ITERATIONS: usize = 1_000_000;
    const NUM_THREADS: usize = 2;

    for i in 0.. {
        let mut rt = tokio::runtime::Builder::new()
            .threaded_scheduler()
            .num_threads(NUM_THREADS)
            .build()
            .unwrap();

        for j in 0 .. NUM_ITERATIONS {
            if j % 1_000 == 0 { println!(" + {} - {}", i, j) }

            rt.block_on(async {
                let handle = tokio::spawn(async {});
                handle.await.unwrap();
            });
        }
    }
}
