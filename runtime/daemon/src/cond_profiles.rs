#![allow(dead_code)]
//! Conditional build profiles
#[derive(Debug, Clone, Copy)]
pub enum Profile { Dev, Release, Test }

pub fn select_profile(name: &str) -> Profile {
    match name.to_lowercase().as_str() {
        "release" => Profile::Release,
        "test" => Profile::Test,
        _ => Profile::Dev,
    }
}
