#![allow(unused_variables)]

use bastion::prelude::*;
use lambda_runtime::{error::HandlerError, lambda, Context};
use serde::{Deserialize, Serialize};
use log::*;
use futures::channel::mpsc::unbounded;


/// This is the JSON payload we expect to be passed to us by the client accessing our lambda.
#[derive(Deserialize, Debug)]
struct InputPayload {
    sites: Vec<String>
}

/// This is the JSON payload we will return back to the client if the request was successful.
#[derive(Serialize, Debug)]
struct OutputPayload {
    status: String
}

fn dispatcher(
    payload: InputPayload,
    _c: Context,
) -> Result<OutputPayload, HandlerError> {
    let (p, mut c) = unbounded::<bool>();

    Bastion::children(|children: Children| {
        children.with_exec(move |_ctx: BastionContext| {
            let sites = payload.sites.clone();
            let workers = worker_pool(payload.sites.len());
            let p = p.clone();

            async move {
                info!("Dispatching started");

                for (worker, site) in workers.elems().iter().zip(sites) {
                    info!("Site sent for processing!");
                    let answer = worker.ask_anonymously(site).unwrap();
                    // Or use the returned body
                    let _ = answer.await.unwrap();
                }

                let _ = p.unbounded_send(true);

                Ok(())
            }
        })
    }).unwrap();

    // Wait for completion signal OR data itself
    while let Err(_) = c.try_next() {}

    Ok(OutputPayload { status: "OK".into() })
}

#[fort::root]
async fn main(_: BastionContext) -> Result<(), ()> {
    let _ = simple_logger::init_with_level(log::Level::Info);
    lambda!(dispatcher);

    Ok(())
}

fn worker_pool(pool_size: usize) -> ChildrenRef {
    Bastion::children(|children: Children| {
        children
            .with_redundancy(pool_size)
            .with_exec(move |ctx: BastionContext| {
                async move {
                    info!("Worker started!");

                    // Start receiving work
                    loop {
                        msg! { ctx.recv().await?,
                            site: String =!> {
                                info!("Received site: {}!", site.clone());
                                let body = surf::get(site.clone()).recv_string().await.unwrap();
                                warn!("Site: {} Body: {}", site, body);
                                let _ = answer!(ctx, body);
                            };
                            _: _ => ();
                        }
                    }
                }
            })
    })
    .expect("Couldn't start a new children group.")
}