#![no_std]
#![allow(unused_attributes)]

extern crate eng_wasm;
extern crate eng_wasm_derive;
extern crate rustc_hex;

use eng_wasm::*;
use eng_wasm::String;
use eng_wasm_derive::eth_contract;
use eng_wasm_derive::pub_interface;
use rustc_hex::ToHex;

#[eth_contract("Sample.json")]
struct EthContract;

#[pub_interface]
pub trait ContractInterface {
  fn toggle_bool(String) -> ();
}

pub struct Contract;
impl ContractInterface for Contract {
  fn toggle_bool(address: String) -> () {
    let c = EthContract::new(&address);
    c.toggleBool();
  }
}
