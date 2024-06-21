use std::sync::{Arc, mpsc};
use std::sync::mpsc::TryRecvError;
use std::thread;
use std::time::Duration;
use springql::{SpringPipeline, SpringConfig, SpringSourceRow};

fn main() {
    const SOURCE_PORT: u16 = 54300;

    // create pipeline instans
    let pipeline = SpringPipeline::new(&SpringConfig::default()).unwrap();

    // execute DDLs for build pipeline

    // source stream inputs to SpringQL pipeline
    pipeline.command(
        "CREATE SOURCE STREAM source_temperature_celsius (
            ts TIMESTAMP NOT NULL ROWTIME,
            temperature FLOAT NOT NULL
        );", ).unwrap();

    // sink stream output from SpringQL pipeline
    pipeline.command(
        "CREATE SINK STREAM sink_temperature_fahrenheit (
            ts TIMESTAMP NOT NULL ROWTIME,
            temperature FLOAT NOT NULL
        );", ).unwrap();

    // create pump for fetching rows from source
    pipeline.command(
        "CREATE PUMP c_to_f AS
            INSERT INTO sink_temperature_fahrenheit (ts, temperature)
            SELECT STREAM
                source_temperature_celsius.ts,
                32.0 + source_temperature_celsius.temperature * 1.8
            FROM source_temperature_celsius;", ).unwrap();

    // create sink writer, accessible by name "q", You can fetch row from `pipeline.pop("q")`
    pipeline.command(
        "CREATE SINK WRITER queue_temperature_fahrenheit FOR sink_temperature_fahrenheit
        TYPE IN_MEMORY_QUEUE OPTIONS (
            NAME 'q'
        );", ).unwrap();

    // create source reader, input come from net_server
    pipeline.command(
        "CREATE SOURCE READER tcp_temperature_celsius FOR source_temperature_celsius
        TYPE IN_MEMORY_QUEUE OPTIONS (
           NAME 'p'
        );").unwrap();
    let (tx, rx) = mpsc::channel();
    let shared = Arc::new(pipeline);
    let s2 = shared.clone();
    let handle = thread::spawn(move || {
        eprintln!("start consumer");

        while let Ok(op_row) = s2.pop_non_blocking("q") {
            match rx.try_recv() {
                Ok(_) => {
                    // 接收到消息，退出循环
                    eprintln!("子线程接收到停止信号，即将退出");
                    break;
                }
                Err(TryRecvError::Empty) => {
                    if let Some(row) = op_row {
                        // get field value from row by field index
                        let ts: String = row.get_not_null_by_index(0).unwrap();
                        let temperature_fahrenheit: f32 = row.get_not_null_by_index(1).unwrap();
                        // show in STDERR
                        eprintln!("{}\t{}", ts, temperature_fahrenheit);
                    }
                }
                Err(TryRecvError::Disconnected) => {
                    // 发送者已经关闭通道，退出循环
                    eprintln!("通道已关闭，子线程即将退出");
                    break;
                }
            }
        }
    });
    shared.push("p", SpringSourceRow::from_json(r#"{"ts": "2022-01-01 13:00:00.000000000", "temperature": 5.3}"#).expect("no json"))
        .expect("pipeline error");
    shared.push("p", SpringSourceRow::from_json(r#"{"ts": "2022-01-01 14:00:00.000000000", "temperature": 6.2}"#).expect("no json"))
        .expect("pipeline error");
    thread::sleep(Duration::from_secs(2));
    eprintln!("stop all thread");
    tx.send(()).expect("无法发送停止信号");

    // 等待子线程结束
    handle.join().unwrap();
    // fetch row from "q" , "q" is a sink writer defined above.
}