use crate::model::{
    id::{BookId, CheckoutId},
    user::{BookOwner, CheckoutUser},
};
use chrono::{DateTime, Utc};

pub mod event;

#[derive(Debug)]
pub struct Book {
    pub id: BookId,
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub description: String,
    pub owner: BookOwner,
    pub checkout: Option<Checkout>,
}

#[derive(Debug)]
pub struct BookListOptions {
    pub limit: i64,
    pub offset: i64,
}

// model::checkout側でも同名の型を定義しているが、
// それとは異なるモジュールにあるので別の型として扱われるa。
// 実際、上記`Book`方のcheckoutフィールドとしてのみ使用する。
#[derive(Debug)]
pub struct Checkout {
    pub checkout_id: CheckoutId,
    pub checked_out_by: CheckoutUser,
    pub checked_out_at: DateTime<Utc>,
}
