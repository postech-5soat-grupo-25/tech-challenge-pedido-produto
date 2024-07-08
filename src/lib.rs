#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

pub mod api;
mod base;
mod controllers;
mod entities;
mod external;
mod gateways;
mod traits;
mod use_cases;