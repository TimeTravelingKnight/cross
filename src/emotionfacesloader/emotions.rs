use std::{sync::Mutex, thread,slice};

use crate::{bnfunctions::bn6fun, gamever, helpermacros::{grab_u32, memcopy, write_u32, MutexValue}, GBASTRUCT::gba};



static EMOTIONPOINTERS:Mutex<[u32;2]>=Mutex::new([0,0]);
static PalettePointers:Mutex<[u32;2]>=Mutex::new([0,0]);
const FACE_LOCIN_ROM:u32=0x1D12C;
const SIZE_OF_FACE:usize=0x100;
const labels_LOC_OFFSET:[u32;2]=[0x41F10,0x417F8];
const labels_LOC_OFFSETPAL:[u32;2]=[0x41F80,0x41868];
const rom_palettes:[u32;2]=[0x75178C,0x753858];
fn WhichVersion(currForm:u8)->usize {
  if (0<currForm && currForm<6) || (currForm==0xB) || (currForm>0xC && currForm<(0xC+6)) || currForm==0x17 {
   return 0;
  }
  1
}
pub fn setDefaultFace(){
  let regs=&gba.get().unwrap().registers;
  let gameversion=MutexValue!(gamever) as usize;
  let gbamemorymap= unsafe{*(((  *bn6fun::GLOBALGBAREG.get().unwrap() as u64)+0x48) as *const u64)} as *const u8;
  let r1=regs[1] as *mut u32;
  let currMegman=grab_u32!(r1,0);
  let ver=WhichVersion(currMegman as u8);
  let offset=labels_LOC_OFFSET[gameversion];
  let value=thread::spawn(move || {
   let mut data=EMOTIONPOINTERS.lock().unwrap();
   data[ver]
 } ).join().unwrap();
  write_u32!(gbamemorymap,offset,value);
 let offset=labels_LOC_OFFSETPAL[gameversion];
 let value =thread::spawn(move || {
   let mut data=PalettePointers.lock().unwrap();
   data[ver]
 } ).join().unwrap();
 write_u32!(gbamemorymap,offset,value);
 
 
 unsafe{*(regs[4] as *mut u32)=currMegman;}

}
pub fn setVersionFace(){
  let gameversion=MutexValue!(gamever) as usize;
  let regs=&gba.get().unwrap().registers;
  let gbamemorymap= unsafe{*(((  *bn6fun::GLOBALGBAREG.get().unwrap() as u64)+0x48) as *const u64)} as *const u8;
  let r0=regs[0] as *mut u32;
  let currMegman=grab_u32!(r0,0);
  let ver=WhichVersion(currMegman as u8);
  let offset=labels_LOC_OFFSET[gameversion];
 let value=thread::spawn(move || {
  let mut data=EMOTIONPOINTERS.lock().unwrap();
  data[ver]
} ).join().unwrap();
 write_u32!(gbamemorymap,offset,value);
 let offset=labels_LOC_OFFSETPAL[gameversion];
let value =thread::spawn(move || {
  let mut data=PalettePointers.lock().unwrap();
  data[ver]
} ).join().unwrap();
write_u32!(gbamemorymap,offset,value);


 unsafe{*(regs[4] as *mut u32)=currMegman;}
}


pub fn writeNewPointer(version:u8,curraddress:u64, rom:Vec<u8>)->u64 {
  let gbamemorymap= unsafe{*(((  *bn6fun::GLOBALGBAREG.get().unwrap() as u64)+0x48) as *const u64)} as *const u32;

  let mut initpointer=curraddress;
println!("{:x?}",initpointer);
  let mut facelocs=curraddress+(0x19<<2);
let mut pointersin_rom =FACE_LOCIN_ROM;
_= thread::spawn(move || {
  let mut data=EMOTIONPOINTERS.lock().unwrap();
  data[version as usize]=initpointer as u32;
} ).join();
for i in 0..0x19 {
let gbamemorymap3=gbamemorymap as *const u8;

write_u32!(gbamemorymap3,initpointer,facelocs);
initpointer+=4;
let mut faceloc :usize=0;
for j in 0..3{
    faceloc+=((rom[(pointersin_rom+j) as usize] as u32 )<<(j*8) )as usize;
}

pointersin_rom+=4;
let face =&rom[faceloc..faceloc+SIZE_OF_FACE];
let gbamemorymap2=gbamemorymap as usize;
let facelocs2=facelocs as usize;

memcopy!(gbamemorymap2,facelocs2,SIZE_OF_FACE,face);

facelocs+=SIZE_OF_FACE as u64;

}
_= thread::spawn(move || {
  let mut data=PalettePointers.lock().unwrap();
  data[version as usize]=facelocs as u32;
} ).join();
let loc=rom_palettes[version as usize] as usize;
let pal=&rom[loc..(loc+0x19*0x20)];
let gbamemorymap3=gbamemorymap as usize;
let sizeOfBytes=0x19*0x20;
let facelocs4=facelocs as usize; 
memcopy!(gbamemorymap3,facelocs4,sizeOfBytes,pal);
facelocs+=0x19*0x20;
facelocs 
}