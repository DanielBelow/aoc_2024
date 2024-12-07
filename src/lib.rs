#![deny(unsafe_code)]
#![warn(nonstandard_style, rust_2018_idioms)]
#![allow(
    clippy::implicit_hasher,
    clippy::similar_names,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;

aoc_runner_derive::aoc_lib! { year = 2024 }
