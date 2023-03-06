use std::time::Duration;

use futures::StreamExt;
use tokio::time::sleep;
use wasmrs_rx::*;

use anyhow::{Error, Result};

#[tokio::main]
async fn main() -> Result<()> {
  basic_mono().await?;

  mono_future().await?;

  mono_later().await?;

  basic_flux().await?;

  flux_channels().await?;

  errors().await?;

  Ok(())
}

async fn basic_mono() -> Result<()> {
  let mono = Mono::<_, Error>::new_success(100);

  let result = mono.await?;

  println!("{}", result);

  Ok(())
}

async fn mono_future() -> Result<()> {
  let mono = Mono::<_, Error>::from_future(async move { Ok(101) });

  let result = mono.await?;

  println!("{}", result);

  Ok(())
}

async fn mono_later() -> Result<()> {
  let mut mono = Mono::<u32, Error>::new();

  mono.success(100);

  let result = mono.await?;

  println!("{}", result);

  Ok(())
}

async fn basic_flux() -> Result<()> {
  let mut flux = FluxChannel::<_, Error>::new();

  flux.send(100)?;
  flux.send(101)?;
  flux.send(102)?;
  flux.complete();

  while let Some(payload) = flux.next().await {
    println!("{}", payload?);
  }

  Ok(())
}

async fn flux_channels() -> Result<()> {
  let flux = FluxChannel::<_, Error>::new();
  let mut rx = flux.take_rx()?;

  let task = tokio::spawn(async move {
    sleep(Duration::from_millis(500)).await;
    flux.send(100).unwrap();
    flux.send(101).unwrap();
    flux.send(102).unwrap();
    flux.complete()
  });

  while let Some(payload) = rx.next().await {
    println!("{}", payload?);
  }
  task.await?;

  Ok(())
}

async fn errors() -> Result<()> {
  let mut flux = FluxChannel::<_, Error>::new();

  flux.send(100)?;
  flux.send(101)?;
  flux.send(102)?;
  flux.error(anyhow::anyhow!("error"))?;
  flux.complete();

  while let Some(payload) = flux.next().await {
    println!("{:?}", payload);
  }
  Ok(())
}
