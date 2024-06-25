use std::ffi::c_int;

use retour::static_detour;

use crate::{CrossWindowAddCross, CrossWindowAddCrossFalzar};

static_detour! {
  static mmbnlc_enable_cross_check_gregar: unsafe extern "system" fn(
   *mut  u64
  ) ->c_int;
  static  mmbnlc_enable_cross_check_falzar: unsafe extern "system" fn(
   *mut u64
)->c_int;
}

pub fn install_hooks_cross_check(){
 unsafe{ mmbnlc_enable_cross_check_gregar.initialize(   std::mem::transmute(0x1423bd7d0 as *mut u8 ), 
  move |a| CrossWindowAddCross(a)).unwrap().enable().unwrap();
  mmbnlc_enable_cross_check_falzar.initialize(   std::mem::transmute(0x142d45e30 as *mut u8), 
  move |a| CrossWindowAddCrossFalzar(a)).unwrap().enable().unwrap();

 }
}