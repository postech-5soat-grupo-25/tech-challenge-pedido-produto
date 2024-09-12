#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

pub mod api;
pub mod rabbit;
mod adapters;
mod base;
mod controllers;
mod entities;
pub mod external;
pub mod gateways;
pub mod traits;
mod use_cases;