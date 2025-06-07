use std::collections::HashMap;
use std::thread;
use std::time::Duration;
use crossbeam::channel::{unbounded, Sender, Receiver, select};
use rand::Rng;

fn main() {
    let (c_sender, c_receiver): (Sender<String>, Receiver<String>) = unbounded();
    let (d_sender, d_receiver): (Sender<(String, i32)>, Receiver<(String, i32)>) = unbounded();

    //creazione sender dati
    for i in 0..10 {
        let d_sender = d_sender.clone();
        thread::spawn(move || {
            for _ in 0..5 {
                let data = (format!("sensor_{}", i), rand::random_range(0..100));
                d_sender.send(data).unwrap();
                thread::sleep(Duration::from_millis(rand::random_range(500..1000)));
            }
        });
    }

    //creazione sender comandi
    for i in 0..10 {
        let c_sender = c_sender.clone();
        thread::spawn(move || {
            for _ in 0..5 {
                let command = format!("average sensor_{}", rand::random_range(0..10));
                c_sender.send(command).unwrap();
                thread::sleep(Duration::from_millis(rand::random_range(500..1000)));
            }
        });
    }

    let h = thread::spawn(move || {
        let mut data: HashMap<String, Vec<i32>> = HashMap::new();
        loop {
            select! {
                recv(d_receiver)-> value=>{
                    match value {
                        Ok((sensor, val))=>{
                            println!("Received data from {}: {}", sensor, val);
                            data.entry(sensor).or_insert(Vec::new()).push(val);
                        },
                        Err(_)=>{
                            println!("Data channel closed");
                            return;
                        }
                    }
                }

                recv(c_receiver) -> cmd =>{
                    match cmd {
                        Ok(value)=>{
                            if value.starts_with("average") {
                                let sensor = value.split_whitespace().nth(1).unwrap();
                                let datas = data.get(sensor);
                                if let Some(datas) = datas{
                                    let avg:f64 = datas.iter().map(|&v| v as f64).sum::<f64>()/datas.len() as f64;
                                    println!("Average of sensor {}: {}", sensor, avg);
                                }
                                else{
                                    println!("No data for sensor {}", sensor);
                                }
                            }
                            else{
                                println!("Command not recognized");
                            }
                        },
                        Err(_)=>{
                            println!("Command channel closed");
                            return;
                        }
                    }
                }
            }
        }
    });

    h.join();
}
