// 先ほどのエクササイズを踏まえ、全てのスレッドがそれぞれのタスクを完了させるようにしたい。
// しかしこのエクササイズでは`JobStatus.jobs_done`という共有された値を更新する担当が必要である。

use std::{sync::{Arc, Mutex}, thread, time::Duration};

struct JobStatus {
    jobs_done: u32,
}

fn main() {
    // TODO: もしも可変な共有された状態が必要ならば、`Arc`は不十分である。
    // 必要なライブラリがあれば適宜useの行で追加すること。
    let status = Arc::new(Mutex::new(JobStatus { jobs_done: 0 }) );

    let mut handles = Vec::new();
    for i in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));

            println!("current thread is {}. job_done {}",i,status_shared.lock().unwrap().jobs_done);

            // TODO: 共有された値の更新する前にこれを実行する必要がある。
            status_shared.lock().unwrap().jobs_done += 1;
            status_shared
        });
        handles.push(handle);
    }

    // 全てのジョブが完了するまで待つ。
    for handle in handles {
        handle.join().unwrap();
    }

    // TODO: `JobStatus.jobs_done`の値を出力してください。
    println!("Jobs done: {}", status.lock().unwrap().jobs_done );
}
