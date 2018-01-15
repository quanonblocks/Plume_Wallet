extern crate rand;
extern crate base64;
extern crate bip39;
extern crate ring_pwhash;
extern crate serde;
extern crate serde_json;
/// TODO: Perhaps, serde_toml
#[macro_use] 
extern crate serde_derive;

use bip39::{ Mnemonic, MnemonicType, Language, Seed };
use rand::Rng;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::fs::OpenOptions;

extern crate Plume_wallet;

use Plume_wallet::{ Salt, LightWallet };

fn main() {
    let wallet = LightWallet::default();

    let json = serde_json::to_vec(&wallet).unwrap();

    match OpenOptions::new().read(true).write(true).truncate(true).create(true).open("keystore.json") {
        Ok(mut f) => f.write_all(&json),
        _ => panic!("error! error! this should not have occurred"),
    };

    let wallet2 = LightWallet::from_file("keystore.json");

    println!("{:?}\n{:?}", wallet, wallet2);
}

fn prompt(message: &str) -> String {
    println!("{}", &message);

    let mut input = String::new();

    io::stdin().read_line(&mut input);
    let res = String::from(input.trim());
    res
}
