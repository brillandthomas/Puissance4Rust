mod ai;
mod client;

pub use ai::{ai_action as action, DEFAULT_DEPTH};
pub use client::play_against;
