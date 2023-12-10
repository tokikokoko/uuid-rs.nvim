use async_trait::async_trait;
use nvim_rs::{compat::tokio::Compat, create::tokio as create, rpc::IntoVal, Handler, Neovim};
use rmpv::Value;
use std::{env, error::Error, fs};
use tokio::io::Stdout;
use uuid::Uuid;

#[derive(Clone)]
struct NeovimHandler {}

#[async_trait]
impl Handler for NeovimHandler {
    type Writer = Compat<Stdout>;

    async fn handle_request(
        &self,
        name: String,
        _args: Vec<Value>,
        _neovim: Neovim<Compat<Stdout>>,
    ) -> Result<Value, Value> {
        match name.as_ref() {
            "uuid" => Ok(Value::from(Uuid::new_v4().to_string())),
            _ => unimplemented!(),
        }
    }
}

#[tokio::main]
async fn main() {
    let handler: NeovimHandler = NeovimHandler {};
    let (nvim, io_handler) = create::new_parent(handler).await;
    let curbuf = nvim.get_current_buf().await.unwrap();

    let mut envargs = env::args();
    let _ = envargs.next();
    let testfile = envargs.next().unwrap();

    fs::write(testfile, &format!("{:?}", curbuf.into_val())).unwrap();

    match io_handler.await {
        Err(joinerr) => eprintln!("Error joining IO loop: '{}'", joinerr),
        Ok(Err(err)) => {
            if !err.is_reader_error() {
                nvim.err_writeln(&format!("Error: '{}'", err))
                    .await
                    .unwrap_or_else(|e| {
                        eprintln!("Well, dang... '{}'", e);
                    });
            }

            if !err.is_channel_closed() {
                eprintln!("Error: '{}'", err);

                let mut source = err.source();

                while let Some(e) = source {
                    eprintln!("Caused by: '{}'", e);
                    source = e.source();
                }
            }
        }
        Ok(Ok(())) => {}
    }
}
