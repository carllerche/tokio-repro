fn main() {
    const NR_ITERATIONS: usize = 1_000_000;

    let mut rt = tokio::runtime::Builder::new()
        .threaded_scheduler()
        .num_threads(1)
        .build()
        .unwrap();

    for i in 0 .. NR_ITERATIONS {
        if i % 1_000 == 0 { println!(" + {}", i) }

        rt.block_on(async {
            let handle = tokio::spawn(async {});
            handle.await.unwrap();
        });
    }
}
