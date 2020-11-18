use serde::{Deserialize, Serialize};
use serde_json::{ Value };
use std::collections::HashMap;
use std::net::UdpSocket;
use std::{thread, time};
use clap::{App, load_yaml};

#[derive(Serialize, Deserialize)]   
struct ExternalCommand
{
    command_name: String,
    params: HashMap<String, Value>
}

fn worker_task ( command_str:String )
{
    let command: ExternalCommand = serde_json::from_str(command_str.as_str()).expect("Can't continue without data");
    let amount_of_time_to_wait = command.params.get("amount_of_time_to_wait").expect("Needs this param");

    let current_thread_index = rayon::current_thread_index().unwrap_or(0);
    let time_to_wait = amount_of_time_to_wait.as_i64().unwrap() as u64;

    println!("Thread#: {}, param: {}", current_thread_index, &time_to_wait);
    let sleep_duration = time::Duration::from_secs(time_to_wait);
    thread::sleep(sleep_duration);
}

fn serve(n_threads: i64, port: String ) 
{
    let pool = rayon::ThreadPoolBuilder::new().num_threads(n_threads as usize).build().unwrap();

    let remote = format!("127.0.0.1:{}", port);
    let socket: UdpSocket = UdpSocket::bind(remote).expect("Couldn't create socket");

    let mut buf = [0; 1024];

    loop {
        match socket.recv(&mut buf) {
            Ok(received) => {
                let received_str: String = String::from_utf8((&buf[..received]).to_vec()).unwrap_or(String::from("{}"));
                println!("received {} bytes {}", received, &received_str);
                buf = [0; 1024];

                pool.spawn(move || worker_task(received_str));
            },
            Err(e) => {
                println!("recv function failed: {:?}", e);
                break;
            },
        }
    }
    println!("Bye");
}

fn main() {

    let yaml = load_yaml!("options.yaml");
    let matches = App::from(yaml).get_matches();

    let port_str = matches.value_of("port").expect("Port can't be none it is required");
    let n_worker = matches.value_of("n_worker").unwrap_or("8").parse::<i64>().expect("No funny biz with worker number");

    serve(n_worker, String::from(port_str));
}
