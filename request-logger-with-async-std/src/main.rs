use bastion::prelude::*;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use async_std::prelude::*;
use async_std::fs::*;

///
/// How to use async-std and bastion together!
///
/// Prologue:
/// This example starts a TCP server and writes all requests to requests.log
/// using fair distribution of workers.
///
/// Here we are offloading our IO bound operations to async-std and using async/await as our
/// interoperation guide.
///
/// We can run side by side, without any problems.
fn main() {
    env_logger::init();

    Bastion::init();

    //
    // Server entrypoint
    Bastion::children(|children: Children| {
        children.with_exec(move |_ctx: BastionContext| {
            // Get a shadowed sharable reference of workers.
            let workers = get_workers();
            let workers = Arc::new(workers);

            async move {
                println!("Server is starting!");

                let listener = TcpListener::bind("127.0.0.1:2278").unwrap();

                // Open logfile with async-std!
                let _ = File::create("requests.log").await.unwrap();

                let mut round_robin = 0;
                for stream in listener.incoming() {
                    // Make a fair distribution to workers
                    round_robin += 1;
                    round_robin %= workers.elems().len();

                    // Distribute tcp streams
                    workers.elems()[round_robin].ask(stream.unwrap()).unwrap();
                }

                // Unreachable, but showing the logic explicitly is nice.
                Bastion::stop();

                Ok(())
            }
        })
    })
        .expect("Couldn't start a new children group.");

    Bastion::start();
    Bastion::block_until_stopped();
}

fn get_workers() -> ChildrenRef {
    Bastion::children(|children: Children| {
        children
            .with_redundancy(10) // Let's have a pool of ten workers.
            .with_exec(move |ctx: BastionContext| {
                async move {
                    println!("Worker started!");

                    // Start receiving work
                    loop {
                        msg! { ctx.recv().await?,
                            stream: TcpStream =!> {
                                let mut stream = stream;
                                let mut data_buf = [0 as u8; 1024];
                                let rb = stream.read(&mut data_buf).unwrap();
                                println!("Received {} bytes", rb);

                                // Let's write requests with async-std.
                                let mut file = OpenOptions::new()
                                    .write(true)
                                    .append(true)
                                    .open("requests.log")
                                    .await
                                    .unwrap();
                                file.write_all(&data_buf).await.unwrap();

                                stream.write("OK".as_bytes()).unwrap();
                            };
                            _: _ => ();
                        }
                    }
                }
            })
    })
        .expect("Couldn't start a new children group.")
}
