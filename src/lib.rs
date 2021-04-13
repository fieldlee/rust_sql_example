use crate::dao::Database;
use std::sync::{Arc, Mutex};

pub mod config;
pub mod controllers;
pub mod dao;
pub mod models;

pub struct AppState<'a> {
    pub connections: Mutex<u32>,
    pub context: Arc<Database<'a>>,
}
