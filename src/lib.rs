#![deny(clippy::all)]
use clap::Parser;
use napi_derive::napi;

use rqjs_cli::start::{start, Args};

#[tokio::main]
pub async fn sync_start(v: Vec<String>) {
  let args = Args::parse_from(v);
  start(args).await
}

#[napi]
pub fn rqjs_start(v: Vec<String>) {
  sync_start(v)
}
