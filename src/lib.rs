#![feature(lazy_cell)]
use std::ffi::{c_float, c_int, c_void, OsStr};

use std::io::Read;
use zip::ZipArchive;
use GBASTRUCT::gba;
use std::fs::File;
use std::{slice, thread};
use std::sync::{LazyLock, Mutex};
mod hook_module;
mod helpermacros;
mod bnfunctions;
mod GBASTRUCT;
use bnfunctions::bn6fun::{self, chip_address, custom_cross_kokoro_change_set, custom_custom_move_cross_select_sub2, custom_move_cross_select_sub, custom_paint_datawindow, drawCrossFaces, getNaviStatus4, soundRequest, GLOBALGBAREG};
use helpermacros::{memcopy, read_u8, write_u32, write_u8};
use hook_module::hooks;



static BEAST:Mutex<u8>=Mutex::new(0);
const beasticon:usize =0x200;
const beastchipsize:usize =0x540;
const palettesize:usize =0x20;
static beasticonlocs:Mutex<[u32;2]>=Mutex::new([0,0]);
static beastchipIcon:Mutex<[u32;2]>=Mutex::new([0,0]);
static beastpallocs:Mutex<[u32;2]>=Mutex::new([0,0]);
const crosspoint:[u8;5]=[0x8,0x18,0x28,0x38,0x48];
const indexinRam:[u8;5]=[0xF,0x10,0x11,0x12,0x13];
const pixelyo:[usize;5]=[0,0x46,0x8C,0xD2,0x118];
const pixely0ForSelected:[usize;5]=[0x15e,0x1a4, 0x1ea,0x230, 0x15e];
const pixelx0ForSelected:[usize;5]=[0,0,0,0,0x14a];

fn testingCross(crossTest :u32,currCount:u8 )->bool {
   let val= 1<<currCount;
   if crossTest&val ==0 {
    return false; 
   }
    return true
}
fn testForExtremeTired(currCount:u8)->bool {
    let regs=&gba.get().unwrap().registers;
    unsafe{
    * (regs[0] as *mut u32)=0;
    * (regs[1] as *mut u32)=0x17;

    }
    getNaviStatus4(regs[0] as *mut u64);
     let val= unsafe {* (regs[0] as *mut u32) } as u8;

    return val==currCount+1;
}

fn CrossWindowAddCross(gbareg:*mut u64)->c_int{
    let oldr14 =unsafe{ *((gbareg as *mut u32).wrapping_add(14))};
    let crossCount=10 as u8;
    let mut currCount=0;
    let mut windowCount =0;
    let maxCount=5;
    let gbamemorymap= unsafe{*(((  *bn6fun::GLOBALGBAREG.get().unwrap() as u64)+0x48) as *const u64)};
    let crossTest=unsafe{* ((gbamemorymap + 0x46e64) as *mut u32)}  ;
    let crossTest = unsafe{* ((gbamemorymap+ (crossTest as u64)) as *mut u32)};
    let regs=&gba.get().unwrap().registers;
    while currCount<crossCount && windowCount<maxCount {
        if !testingCross(crossTest, currCount)
        {
           if !testForExtremeTired(currCount) {
               let crossindex=0x50+windowCount;
               
               let megaman=unsafe{ *(regs[5] as *mut u32) };
              unsafe{*(gbamemorymap as *mut u8).wrapping_add( (megaman+crossindex) as usize)=currCount;}
              let crossSelected: u32=0x55+currCount as u32; //will need to change this later
              unsafe{ *(gbamemorymap as *mut u8).wrapping_add((megaman+crossSelected ) as usize )=0;}
               windowCount+=1;
           }
        }
        currCount += 1;
    }    
    unsafe { * (regs[0] as *mut u32) =windowCount;}
    return oldr14 as c_int;

}

fn aftersetCross(){
    let mut count= unsafe{*(0x143d34a80 as *mut u8) };
    let mut structOfCrossFaces:*const u32 =0x143d34a90 as *const u32;
    while (count >0) {
        {  
            unsafe {
            let crosstoDraw=*structOfCrossFaces as u8;
            let indextoDraw = *structOfCrossFaces.wrapping_add(1) as u8;
            let currCrossSelected = *structOfCrossFaces.wrapping_add(2) as u8;
            let selected = *structOfCrossFaces.wrapping_add(3) as u8;
            setCrosses(crosstoDraw, indextoDraw, currCrossSelected, selected, 1)
            }
        }

        count-=1;
        structOfCrossFaces=structOfCrossFaces.wrapping_add(4);
    }
    
}


fn setCrosses(crosstoDraw:u8,indextoDraw:u8,currCrossSelected:u8,selected:u8, amountToDraw:u8) {
    let mut y0=pixelyo[(crosstoDraw%5) as usize];
    let mut x0=0x14a;
    if indextoDraw==currCrossSelected {
        if (selected==0) {
            x0=0;
        }
        else {
            y0=pixely0ForSelected[(crosstoDraw%5) as usize];
            x0=pixelx0ForSelected[(crosstoDraw%5) as usize];
    
        }
    }
    x0+=640*((crosstoDraw<5) as usize);
   unsafe{ 
    drawCrossFaces( *(0x143d33bc0 as *mut u64),
    0,
    3,
    indexinRam[indextoDraw as usize], //index in ram
    3,
    ((x0 as c_float)/1280.0)*100.0,
    ((y0 as c_float)/640.0)*100.0,
    ((288.0+(x0 as c_float))/1280.0)*100.0,
    ((64.0+ (y0 as c_float))/640.0)*100.0,
    7,
    crosspoint[indextoDraw as usize] as i16, //pointtodraw
    0x48,
    0x10,
    0,
    0,
    0x3f800000,
    0x3f800000,
    (*(0x143b3d46c as *const u32 ) | 0xffffff00) << 8 | (*(0x143b3d46c as *const u32 ) |  & 0xff) << 8 | (*(0x143b3d46c as *const u32 )& 0xff)  ,
    amountToDraw as u32
);


    



}
}


fn setfaces(){
    let mut curraddress=0x8000000;
    let gbamemorymap= unsafe{*(((  *bn6fun::GLOBALGBAREG.get().unwrap() as u64)+0x48) as *const u64)};
   {
    let archive=File::open("./data/exe6.dat").unwrap();
    let mut archive_zip=ZipArchive::new(archive).unwrap();
  
    {
        let mut rom:Vec<u8> =vec!();   
    for i in 0..archive_zip.len(){
    let mut romindirectory=archive_zip.by_index(i).unwrap();
    let name=romindirectory.enclosed_name();
    if let Some(name)=name {
   
        if name.file_name() == Some(OsStr::new("rom.srl")) {
            _=romindirectory.read_to_end(&mut rom);
            {
             let icon=&rom[0x708374..0x708374+beasticon];
             memcopy!(gbamemorymap ,curraddress ,beasticon,icon);
            
           _= thread::spawn(move || {
           
                let mut data = beasticonlocs.lock().unwrap();
                  data[0]=curraddress as u32;
                
                }).join(); 
            curraddress+=beasticon as u64;
            let beastchiptech =&rom[0x74550C..0x74550C+beastchipsize];
            memcopy!(gbamemorymap,curraddress,beastchipsize,beastchiptech);
            _= thread::spawn(move || {
           
                let mut data = beastchipIcon.lock().unwrap();
                  data[0]=curraddress as u32;
                
                }).join(); 
            curraddress+=beastchipsize as u64;
            }
            let beastpal =&rom[0x747E8C ..0x747E8C +palettesize];
            memcopy!(gbamemorymap,curraddress,palettesize,beastpal);
            _= thread::spawn(move || {
           
                let mut data = beastpallocs.lock().unwrap();
                  data[0]=curraddress as u32;
                
                }).join(); 

            curraddress+=palettesize as u64;
           break;
            
         
         }
    }
    }
    }

}
{
    let archive=File::open("./data/exe6f.dat").unwrap();
    let mut archive_zip=ZipArchive::new(archive).unwrap();
  
    {
        let mut rom:Vec<u8> =vec!();   
    for i in 0..archive_zip.len(){
    let mut romindirectory=archive_zip.by_index(i).unwrap();
    let name=romindirectory.enclosed_name();
    if let Some(name)=name {
   
        if name.file_name() == Some(OsStr::new("rom_f.srl")) {
            _=romindirectory.read_to_end(&mut rom);
            {
             let icon=&rom[0x70A414..0x70A414+beasticon];
             memcopy!(gbamemorymap ,curraddress ,beasticon,icon);
            
           _= thread::spawn(move || {
           
                let mut data = beasticonlocs.lock().unwrap();
                  data[1]=curraddress as u32;
                
                }).join(); 
            curraddress+=beasticon as u64;

              let beastchiptech =&rom[0x7475D8..0x7475D8+beastchipsize];
            memcopy!(gbamemorymap,curraddress,beastchipsize,beastchiptech);
            _= thread::spawn(move || {
           
                let mut data = beastchipIcon.lock().unwrap();
                  data[1]=curraddress as u32;
                
                }).join(); 
            curraddress+=beastchipsize as u64; 

            let beastpal =&rom[0x749F58 ..0x749F58 +palettesize];
            memcopy!(gbamemorymap,curraddress,palettesize,beastpal);
            _= thread::spawn(move || {
           
                let mut data = beastpallocs.lock().unwrap();
                  data[1]=curraddress as u32;
                
                }).join(); 

            curraddress+=palettesize as u64;





            }
           break;
            
         
         }
    }
    }
    }

}


}
fn gregarnewgame(gbareg:*mut u64)->c_int{
    bn6fun::GLOBALGBAREG.get_or_init(|| gbareg as u64);
    GBASTRUCT::gba.get_or_init(|| GBASTRUCT::init());
    setfaces();
    let val=bn6fun::NEWGAMEINIT(gbareg);
    val
}
fn gregarcontinue(gbareg: *mut u64 ) {
    bn6fun::GLOBALGBAREG.get_or_init(|| gbareg as u64);
    GBASTRUCT::gba.get_or_init(|| GBASTRUCT::init());
    setfaces();
    //let gbamemorymap= unsafe{*(((gbareg as u64)+0x48) as *const u64)};
    bn6fun::IDKWHATTHIS(gbareg,gbareg as *mut u32,1);

}

fn customscreen_effects() {
   let gbareg= &GBASTRUCT::gba.get().unwrap().registers;
   let pointertoregs: *mut u64=*bn6fun::GLOBALGBAREG.get().unwrap() as *mut u64;
   let curvalue= thread::spawn(move || {
           
    let mut data = BEAST.lock().unwrap();
    *data
    
    }).join(); 

 if curvalue.unwrap()==1 {
    _=thread::spawn(move || {
           
        let mut data = BEAST.lock().unwrap();
        *data=0
        
        }).join(); 
        let ram=unsafe{*( (gbareg[0] as *mut u32).wrapping_add(0x48>>2) as *mut u64)} as *mut u8;
        let mut beastoffset=0x46B44;
        let beasticonInLabels=0x42C7C>>2;
        let beasticonInLabels2 =0x466F8>>2;
        let beastchipicon =0x40950>>2;
        let beastchipicon2 = 0x46810>>2;
        let currbeast=read_u8!(ram,beastoffset);
        let mut sound=0x92;
        if (currbeast==0xB) {
            write_u8!(ram,beastoffset,0xC);
            beastoffset+=4;
            write_u8!(ram,beastoffset,0x18);
            sound =0x94;

            
        }
        else {
            write_u8!(ram,beastoffset,0xB);
            beastoffset+=4;
            write_u8!(ram,beastoffset,0x17);
        }
        
       let value=thread::spawn(move || {
           
            let data = beasticonlocs.lock().unwrap();
            let index=(currbeast%0xB+1)%2;
            data[index as usize] 
          
            
            }).join().unwrap();
        let ram= ram as *mut u32;
        write_u32!(ram,beasticonInLabels,value);
        write_u32!(ram,beasticonInLabels2,value);
        let value=thread::spawn(move || {
           
            let data = beastchipIcon.lock().unwrap();
            let index=(currbeast%0xB+1)%2;
            data[index as usize] 
          
            
        }).join().unwrap();
        write_u32!(ram,beastchipicon,value);
        write_u32!(ram,beastchipicon2,value);
        let beastpal=beastchipicon+1;
        let beastpal2=beastchipicon2+1;
        let value=thread::spawn(move || {
           
            let data = beastpallocs.lock().unwrap();
            let index=(currbeast%0xB+1)%2;
            data[index as usize] 
          
            
        }).join().unwrap();
        write_u32!(ram,beastpal,value);
        write_u32!(ram,beastpal2,value);
        
        
        unsafe{*(0x141f976aA as *mut u8)=sound; }
        unsafe{*(gbareg[0] as *mut u32)=(sound as u32)+0xFF};
        soundRequest(pointertoregs);
        
 }


    
  
  else {
  let offset=  unsafe{*(gbareg[5] as *mut u32)}+0x1A;   
    let ram=unsafe{*( (gbareg[0] as *mut u32).wrapping_add(0x48>>2) as *mut u64)} as *mut u8;
    let crossbyte=read_u8!(ram,offset);
    unsafe {
    *((gbareg[0]) as *mut u32)=crossbyte as u32;   
    }
    custom_cross_kokoro_change_set( pointertoregs);
    custom_custom_move_cross_select_sub2(pointertoregs);
    custom_move_cross_select_sub(pointertoregs);
    custom_paint_datawindow(pointertoregs);
    unsafe {
        *((gbareg[0]) as *mut u32)=0x92;   
        }
    soundRequest(pointertoregs);
    }
}
fn beastCheck2() {
    unsafe { *(0x143D34A80 as *mut u8)=0}
  /*     let curvalue= thread::spawn(move || {
           
        let mut data = BEAST.lock().unwrap();
        *data
        
        }).join(); 
        
    */
    /* if curvalue.unwrap()==0 {
        functiontosetfaces();
        otherfunctiontosetfaces();
        otherfunctiontosetfaces2(0);
     } */


}
fn beastCheck() ->c_int {
    let gbareg= &GBASTRUCT::gba.get().unwrap().registers;
    let pointertoregs: *mut u64=*bn6fun::GLOBALGBAREG.get().unwrap() as *mut u64;
    let currentChip= unsafe {
        *((gbareg[0]) as *mut u32) as u8  
        };

    let ram=unsafe{*( (gbareg[0] as *mut u32).wrapping_add(0x48>>2) as *mut u64)} as *mut u8;   
    if currentChip==0xB{
       _=  thread::spawn(move || {
           
            let mut data = BEAST.lock().unwrap();
            *data = 1;
            
        }).join();
        let mut offset=  unsafe{*(gbareg[5] as *mut u32)}+1;  
            write_u8!(ram,offset,0x5C);
        offset+=1;
           write_u8!(ram,offset,0x0);    
          unsafe{ *(0x143d34a80 as *mut u8)=0;}
    }
     chip_address(pointertoregs)
}

#[no_mangle]
pub extern "C" fn luaopen_make(_:c_void)-> c_int{

    hooks::GM_HOOK!(0x141edebcb,gregarnewgame,16);
    
    hooks::GM_HOOK!(0x141edec8e,gregarcontinue,14);
    hooks::GM_HOOK!(0x141f9a1ab,customscreen_effects,14);
    unsafe {
      *(0x141F9A1B9 as *mut u8)=0xEB;
      *(0x141F9A1BA as *mut u8)=0x7B;
                           //jump
    }
     hooks::GM_HOOK!(0x1423ac324,beastCheck,15);   
     hooks::GM_HOOK!(0x141f8c960,setCrosses,16 );
      unsafe{
     *(0x141f8c96f as *mut u8)  = 0xC3;       
 
}
     hooks::GM_HOOK!(0x141f8cbc0,aftersetCross,15);
     unsafe{
        *(0x141f8cbcf as *mut u8)  = 0xC3;       
    
   }
   //load headcross palette
    unsafe {

        *(0x1423bd077 as *mut u8)=0x0;
        *(0x1423bcd13 as *mut u8)=0xBA;
        *(0x1423bcd14 as *mut u8)=0x0;
        *(0x1423bcd15 as *mut u8)=0x0;
        *(0x1423bcd16 as *mut u8)=0x0;
        *(0x1423bcd17 as *mut u8)=0x0;
        *(0x1423bcd18 as *mut u8)=0x90;

    }
    hooks::GM_HOOK!(0x1423bd7d0,CrossWindowAddCross,13);
    unsafe
     {
        *(0x1423bd7dd as *mut u8)  = 0xC3;   
     }
 //   hooks::GM_HOOK!(0x141f9a160,beastCheck2,27);
    0
}