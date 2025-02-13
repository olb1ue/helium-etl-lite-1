use crate::*;
use tokio_postgres::{Client, Statement};
use helium_api::models::transactions::Reward;
use std::convert::TryFrom;

pub async fn prepare(client: &Client) -> Result<Statement>{
  let stmt = client.prepare("INSERT INTO rewards (block, transaction_hash, time, account, gateway, amount, type) 
    VALUES ($1, $2, $3, $4, $5, $6, $7)").await;
  match stmt {
    Ok(s) => Ok(s),
    Err(e) => Err(error::Error::PgError(e)),
  }
}

pub async fn add_reward(client: &Client, 
  block: u64, 
  time: u64, 
  hash: String, 
  reward: &Reward) -> Result<Vec<tokio_postgres::Row>> {
  let stmt = prepare(&client).await.unwrap();
  let gateway: &String;
  let default: &String = &String::from("1Wh4bh");

  gateway = match &reward.gateway {
    Some(g) => g,
    None => default,
  };
  // for overages
  let account = match &reward.account {
    Some(a) => a,
    None => default,
  };

  match client.query(&stmt, &[&i64::try_from(block).unwrap(), 
      &hash, 
      &i64::try_from(time).unwrap(), 
      &account, 
      &gateway,
      &i64::try_from(reward.amount).unwrap(),
      &reward.r#type]).await {
    Ok(v) => Ok(v),
    Err(e) => {
      println!("{}", e);
      Err(error::Error::PgError(e))
    },
  }   
}

