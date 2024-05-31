use std::sync::{Mutex, OnceLock};

use crate::bnfunctions::bn6fun::{self, GLOBALGBAREG};

#[repr(C)]
pub struct GBASTRUCT {
  pub registers:Vec<u32>

}

impl GBASTRUCT {
   fn init()->GBASTRUCT{
      GBASTRUCT{registers:vec![]}
    }
}

pub fn init()->GBASTRUCT{
let regs=*bn6fun::GLOBALGBAREG.get().unwrap() as u32;

let mut gbabuild=GBASTRUCT::init();


for i in 0..14{
 gbabuild.registers.push((regs as u32).wrapping_add(i*4 as u32));
}

gbabuild

}

pub static gba:OnceLock<GBASTRUCT>=OnceLock::new();