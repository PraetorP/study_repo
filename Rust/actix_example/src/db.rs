use std::io;

use deadpool_postgres::Client;
use itertools::{any, Itertools};
use tokio_pg_mapper::FromTokioPostgresRow;

use crate::models::{ToDoItem, ToDoList};
use eyre::{eyre, Report, Result};

pub async fn get_todos(client: &Client) -> Result<Vec<ToDoList>> {
    let statement = client.prepare("select * from todo_list").await?;

    client
        .query(&statement, &[])
        .await?
        .into_iter()
        .map(|row| ToDoList::from_row(row).map_err(eyre::Error::from))
        .collect()
}

pub async fn get_items(client: &Client, list_id: i32) -> Result<Vec<ToDoItem>> {
    let statement = client
        .prepare("select * from todo_item where list_id = $1 order by id")
        .await?;

    client
        .query(&statement, &[&list_id])
        .await?
        .into_iter()
        .map(|row| ToDoItem::from_row(row).map_err(eyre::Error::from))
        .collect()
}

pub async fn create_todo(client: &Client, title: String) -> Result<ToDoList> {
    let statement = client
        .prepare("insert into todo_list (title) values($1) returning id, title")
        .await?;

    client
        .query(&statement, &[&title])
        .await?
        .into_iter()
        .map(|row| ToDoList::from_row(row).map_err(eyre::Error::from))
        .collect::<Result<Vec<_>, _>>()?
        .pop()
        .ok_or(eyre!("failed to insert"))
}

pub async fn check_todo(client: &Client, list_id: i32, item_id: i32) -> Result<()> {
    let statement = client
        .prepare("update todo_item set checked = true where list_id = $1 and id = $2 and checked = false")
        .await?;

    match client.execute(&statement, &[&list_id, &item_id]).await? {
        1 => Ok(()),
        _ => Err(eyre!("already changed!!!"))
    }
}
