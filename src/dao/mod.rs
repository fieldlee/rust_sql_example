use super::models::{Group,User};

pub mod db_context;
mod group_dao;
mod user_dao;
mod user_join_group_dao;


pub type Table<'c, T> = db_context::Table<'c, T>;
pub type Database<'c> = db_context::Database<'c>;
pub type JoinTable<'c, T1, T2> = db_context::JoinTable<'c, T1, T2>;