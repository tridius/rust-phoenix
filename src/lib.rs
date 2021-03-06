#![allow(warnings)]
#![cfg_attr(all(test, feature = "nightly"), feature(test))]

//! Phoenix is a Phoenix client library written in Rust.
//!
//! 

#[cfg(all(feature = "nightly", test))]
extern crate test;
extern crate websocket;

pub mod channel;
pub mod socket;

#[cfg(test)]
mod tests {
    use super::socket;

    #[test]
    fn can_construct_and_use_socket() {
        let mut socket_builder = socket::SocketBuilder::new(String::from("url"))
        .add_on_open(|| println!{"Hello there!"})
        .add_on_message(|message| println!("{}", message));
        let mut socket = socket_builder.finish();
        socket.process_events();
    }
}