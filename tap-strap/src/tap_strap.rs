#![no_std]
#![no_main]

use core::cell::RefCell;

use bleps::{self};



pub struct TapStrap {

}

impl TapStrap {
    pub async fn new(ble: &bleps::asynch::Ble) -> Self {
        ble.
    }
}