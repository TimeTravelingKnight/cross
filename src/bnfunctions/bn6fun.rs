use std::ffi::c_char;
use std::ffi::c_float;
use std::ffi::c_short;
use std::fmt::Pointer;
use std::sync::OnceLock;
use std::sync::LazyLock;
use std::ffi::c_int;


macro_rules! BN_FUNCTION{
  ($func_name:ident,$offset:literal) => {
      pub static $func_name:LazyLock<fn(*mut u64)->c_int> =LazyLock::new (|| {
          let gbamemorymap= unsafe{*(((GLOBALGBAREG.get().unwrap())+0x48) as *mut u64) };
          unsafe {std::mem::transmute(*((gbamemorymap+$offset) as *mut u64) as *const())}}); 
      
  };
}



pub static GLOBALGBAREG:OnceLock<u64>=OnceLock::new();
pub const mapaddress:*const u32=0x143d34c08 as *const u32;
BN_FUNCTION!(NEWGAMEINIT,0x5b628);
BN_FUNCTION!(GREGARSAVE,0xa6488);
BN_FUNCTION!(custom_cross_kokoro_change_set,0x46ec8);
BN_FUNCTION!(custom_custom_move_cross_select_sub2,0x46288);
BN_FUNCTION!( custom_move_cross_select_sub,0x46278);
BN_FUNCTION!(custom_paint_datawindow,0x46758);
BN_FUNCTION!(chip_address,0x464e8);
BN_FUNCTION!(getNaviStatus4 ,0x75638);


pub static drawCrossFaces:LazyLock<fn(
u64,//param1
u64,//param2
u8, // param3
u8, // param4
u8, // param5
c_float, //param6
c_float, //param7
c_float, //param8
c_float, //param9
c_short, //param10
c_short, //param11
u8, //param12
u8, //param13
c_char, //param14
 u32, //param15
 u32, //param16
u32, //param17
u32, //param18
u32 //param19










)>=LazyLock::new(|| unsafe{std::mem::transmute(0x1431970b0 as *const()) });

pub static IDKWHATTHIS:LazyLock<fn(*mut u64,*mut u32, u32)>=LazyLock::new(|| unsafe{std::mem::transmute(0x14002a1a0 as *const() )});

pub static convert_rom_address_to_mpak:LazyLock<fn (*const u32, u32)->*const u64>=LazyLock::new(|| unsafe{std::mem::transmute(0x143162520 as *const())});

pub static soundRequest:LazyLock<fn(*mut u64)-> u32>= LazyLock::new(|| unsafe{std::mem::transmute(0x141f3cae0 as *const())});