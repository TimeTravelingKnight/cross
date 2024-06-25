#![feature(lazy_cell)]
use std::borrow::BorrowMut;
use std::ffi::{c_float, c_int, c_void, OsStr};

use std::io::Read;
use emotionfacesloader::emotions::{setDefaultFalzarFace, setDefaultGregarFace, setVersionFaceFalzar, setVersionFaceGregar, writeNewPointer};
use zip::ZipArchive;
use CrossWindow::detourCrossWindow::install_hooks_cross_check;
use GBASTRUCT::gba;
use std::fs::File;
use std::{slice, thread};
use std::sync::{LazyLock, Mutex};
mod hook_module;
mod helpermacros;
mod bnfunctions;
mod GBASTRUCT;
mod emotionfacesloader;
mod CrossWindow;
use bnfunctions::bn6fun::{self, chip_address, custom_cross_kokoro_change_set, custom_custom_move_cross_select_sub2, custom_move_cross_select_sub, custom_paint_datawindow, drawCrossFaces, getNaviStatus4, getNaviStatus4Falzar, soundRequest, GLOBALGBAREG};
use helpermacros::{memcopy, read_u8, write_u32, write_u8, MutexValue};
use hook_module::hooks;



static BEAST:Mutex<u8>=Mutex::new(0);
const beasticon:usize =0x200;
const beastchipsize:usize =0x540;
const palettesize:usize =0x20;
const addrOfNumcrosses:[u64;2]=[0x143d34a80,0x143d35cbc];
static beasticonlocs:Mutex<[u32;2]>=Mutex::new([0,0]);
static beastchipIcon:Mutex<[u32;2]>=Mutex::new([0,0]);
static beastpallocs:Mutex<[u32;2]>=Mutex::new([0,0]);
static gamever:Mutex<u32>=Mutex::new(0);


const crosspoint:[u8;5]=[0x8,0x18,0x28,0x38,0x48];
const indexinRam:[u8;5]=[0xF,0x10,0x11,0x12,0x13];
const pixelyo:[usize;5]=[0,0x46,0x8C,0xD2,0x118];
const pixely0ForSelected:[usize;5]=[0x15e,0x1a4, 0x1ea,0x230, 0x15e];
const pixelx0ForSelected:[usize;5]=[0,0,0,0,0x14a];
const CrossMax:u8=10;

const windowCount:[fn()->u8;2]=[winDowCurrCount,winDowCurrCountfalzar];
fn winDowCurrCountfalzar()->u8 {
   
    let gbamemorymap= unsafe{*(((  *bn6fun::GLOBALGBAREG.get().unwrap() as u64)+0x48) as *const u64)} as *mut u8;
    let mut differentwindowcount=0;
    let mut newCount=0;
    let crossTest=unsafe{* ((gbamemorymap as u64 + testingCrosses[1] as u64) as *mut u32)}  ;
    let crossTest = unsafe{* ((gbamemorymap.wrapping_add(crossTest as usize)) as *mut u32)};
     while newCount<CrossMax {
        if !testingCross(crossTest, newCount)
        {
           if !funcfortired[1](newCount) {
               
               differentwindowcount+=1;
               
           }
        }
        newCount += 1;
    }
    differentwindowcount  
}


fn winDowCurrCount()->u8 {
   
    let gbamemorymap= unsafe{*(((  *bn6fun::GLOBALGBAREG.get().unwrap() as u64)+0x48) as *const u64)} as *mut u8;
    let mut differentwindowcount=0;
    let mut newCount=0;
    let crossTest=unsafe{* ((gbamemorymap as u64 + testingCrosses[0] as u64) as *mut u32)}  ;
    let crossTest = unsafe{* ((gbamemorymap.wrapping_add(crossTest as usize)) as *mut u32)};
     while newCount<CrossMax {
        if !testingCross(crossTest, newCount)
        {
           if !funcfortired[0](newCount) {
               
               differentwindowcount+=1;
               
           }
        }
        newCount += 1;
    }
    differentwindowcount  
}
fn initForFirstWindow() {
    let regs=&GBASTRUCT::gba.get_or_init(|| GBASTRUCT::init()).registers;
    let gbamemorymap= unsafe{*(((  *bn6fun::GLOBALGBAREG.get().unwrap() as u64)+0x48) as *const u64)} as *mut u8;
    let megaman=unsafe{ *(regs[5] as *mut u32) };
    unsafe{*(gbamemorymap as *mut u8).wrapping_add( (megaman+0x1B) as usize)=2;}
    let crossindexes= (gbamemorymap as *mut u8).wrapping_add(megaman as usize+0x50);
          
        
      unsafe{   *crossindexes.wrapping_add(2)=*crossindexes.wrapping_add(0)};
        

}
const signed:[fn(i8);2]=[goSignedDirectionForCross,goSignedDirectionForCrossFalzar];
fn goSignedDirectionForCrossFalzar(sign :i8) {
    let regs=&gba.get().unwrap().registers;
    let gbamemorymap= unsafe{*(((  *bn6fun::GLOBALGBAREG.get().unwrap() as u64)+0x48) as *const u64)} as *mut u8;
    let megaman=unsafe{ *(regs[5] as *mut u32) };
    let crossindexes= (gbamemorymap as *mut u8).wrapping_add(megaman as usize+0x50);
    let mut window_count=0;
    let mut currCount= unsafe{*(crossindexes.wrapping_add(2))};
    let crossTest=unsafe{* ((gbamemorymap as u64 + testingCrosses[1 as usize] as u64) as *mut u32)}  ;
  
    let crossTest = unsafe{* ((gbamemorymap.wrapping_add(crossTest as usize)) as *mut u32)};
    let mut crossindex=window_count;
    while window_count<3{
        if !testingCross(crossTest, currCount)
        {
           if !funcfortired[1](currCount) {
            if sign >0  {
                crossindex=2+window_count;
            }
            else {
                crossindex =2-window_count;
            }
           
              unsafe{*(crossindexes.wrapping_add(crossindex))=currCount;}
              
            
               window_count+=1;
           }
        }
           if (sign>0){
           currCount = (currCount +1)%10 ;
           }
           else {
            if currCount!=0 {
            currCount = currCount -1;
            }
            else {
                currCount=9;
            }
           }
       

}
}

fn goSignedDirectionForCross(sign :i8) {
    let regs=&gba.get().unwrap().registers;
    let gbamemorymap= unsafe{*(((  *bn6fun::GLOBALGBAREG.get().unwrap() as u64)+0x48) as *const u64)} as *mut u8;
    let megaman=unsafe{ *(regs[5] as *mut u32) };
    let crossindexes= (gbamemorymap as *mut u8).wrapping_add(megaman as usize+0x50);
    let mut window_count=0;
    let mut currCount= unsafe{*(crossindexes.wrapping_add(2))};
    let crossTest=unsafe{* ((gbamemorymap as u64 + testingCrosses[0] as u64) as *mut u32)}  ;
  
    let crossTest = unsafe{* ((gbamemorymap.wrapping_add(crossTest as usize)) as *mut u32)};
    let mut crossindex=window_count;
    while window_count<3{
        if !testingCross(crossTest, currCount)
        {
           if !funcfortired[0](currCount) {
            if sign >0  {
                crossindex=2+window_count;
            }
            else {
                crossindex =2-window_count;
            }
           
              unsafe{*(crossindexes.wrapping_add(crossindex))=currCount;}
              
            
               window_count+=1;
           }
        }
           if (sign>0){
           currCount = (currCount +1)%10 ;
           }
           else {
            if currCount!=0 {
            currCount = currCount -1;
            }
            else {
                currCount=9;
            }
           }
       

}
}


fn scroll() {
    let regs=&gba.get().unwrap().registers;
    let gbamemorymap= unsafe{*(((  *bn6fun::GLOBALGBAREG.get().unwrap() as u64)+0x48) as *const u64)} as *mut u8;
    let game=gamever.lock().unwrap();
        let choice= unsafe{*(regs[4] as *mut u32)} as u8; 
    if windowCount[*game as usize]()>5 {
        let regs=&gba.get().unwrap().registers;
        let gbamemorymap= unsafe{*(((  *bn6fun::GLOBALGBAREG.get().unwrap() as u64)+0x48) as *const u64)} as *mut u8;
        let megaman=unsafe{ *(regs[5] as *mut u32) };
        let crossindexes= (gbamemorymap as *mut u8).wrapping_add(megaman as usize+0x50);
        let nextCross=read_u8!(crossindexes,choice);
      
       
        write_u8!(crossindexes,2,nextCross);
        signed[*game as usize](-1);
        signed[*game as usize](1);

    }
    
    else {
     
        let offset= (unsafe{*(regs[5] as *mut u32)}+0x1b) as usize;
       
        write_u8!(gbamemorymap ,offset,choice);
        
    }

} 

fn testingCross(crossTest :u32,currCount:u8 )->bool {
   let val= 1<<currCount;
   if crossTest&val ==0 {
    return false; 
   }
    return true
}
fn testForExtremeTiredFalzar(currCount:u8) ->bool {
    let regs=&gba.get().unwrap().registers;
    
   
    
   
    unsafe{
    * (regs[0] as *mut u32)=0;
    * (regs[1] as *mut u32)=0x17;

    }
    getNaviStatus4Falzar(regs[0] as *mut u64);

    let val= unsafe {* (regs[0] as *mut u32) } as u8;
    
   val==(currCount+1)
    

}
fn testForExtremeTired(currCount:u8)->bool {
    let regs=&gba.get().unwrap().registers;
    
   
    
   
    unsafe{
    * (regs[0] as *mut u32)=0;
    * (regs[1] as *mut u32)=0x17;

    }
    
   
     getNaviStatus4(regs[0] as *mut u64);

     let val= unsafe {* (regs[0] as *mut u32) } as u8;
     
    val==(currCount+1)
}

const testingCrosses:[u64;2]=[0x46e64, 0x46574];
const funcfortired:[fn(u8)->bool;2]=[testForExtremeTired,testForExtremeTiredFalzar];
fn CrossWindowAddCross(gbareg:*mut u64)->c_int{
    let oldr14 =unsafe{ *((gbareg as *mut u32).wrapping_add(14))};
    let crossCount=10 as u8;
    let mut currCount=0;
    let mut window_count =0;
    let maxCount=5;
  
    let gbamemorymap= unsafe{*(((  *bn6fun::GLOBALGBAREG.get().unwrap() as u64)+0x48) as *const u64)};
    //let game=gamever.lock().unwrap(); 
    let crossTest=unsafe{* ((gbamemorymap + testingCrosses[0]) as *mut u32)}  ;
    let crossTest = unsafe{* ((gbamemorymap+ (crossTest as u64)) as *mut u32)};
    let regs=&GBASTRUCT::gba.get_or_init(|| GBASTRUCT::init()).registers;
   

    while currCount<crossCount && window_count<maxCount {
        if !testingCross(crossTest, currCount)
        {
           
           if !funcfortired[0](currCount) {
               let crossindex=0x50+window_count;
               
               let megaman=unsafe{ *(regs[5] as *mut u32) };
              unsafe{*(gbamemorymap as *mut u8).wrapping_add( (megaman+crossindex) as usize)=currCount;}
              let crossSelected: u32=0x55+window_count as u32; //will need to change this later
              unsafe{ *(gbamemorymap as *mut u8).wrapping_add((megaman+crossSelected ) as usize )=0;}
               window_count+=1;
           }
        }
        currCount += 1;
    }    
   
   
    if  windowCount[0]()>5 {
        
       initForFirstWindow();
       signed[0](-1);
       signed[0](1);
    }
    unsafe { * (regs[0] as *mut u32) =window_count;}
    oldr14 as c_int
    
}

fn CrossWindowAddCrossFalzar(gbareg:*mut u64)->c_int{
    let oldr14 =unsafe{ *((gbareg as *mut u32).wrapping_add(14))};
    let crossCount=10 as u8;
    let mut currCount=0;
    let mut window_count =0;
    let maxCount=5;
  
    let gbamemorymap= unsafe{*(((  *bn6fun::GLOBALGBAREG.get().unwrap() as u64)+0x48) as *const u64)};
    //let game=gamever.lock().unwrap(); 
    let crossTest=unsafe{* ((gbamemorymap + testingCrosses[1]) as *mut u32)}  ;
    let crossTest = unsafe{* ((gbamemorymap+ (crossTest as u64)) as *mut u32)};
    let regs=&GBASTRUCT::gba.get_or_init(|| GBASTRUCT::init()).registers;
   

    while currCount<crossCount && window_count<maxCount {
        if !testingCross(crossTest, currCount)
        {
           
           if !funcfortired[1](currCount) {
               let crossindex=0x50+window_count;
               
               let megaman=unsafe{ *(regs[5] as *mut u32) };
              unsafe{*(gbamemorymap as *mut u8).wrapping_add( (megaman+crossindex) as usize)=currCount;}
              let crossSelected: u32=0x55+window_count as u32; //will need to change this later
              unsafe{ *(gbamemorymap as *mut u8).wrapping_add((megaman+crossSelected ) as usize )=0;}
               window_count+=1;
           }
        }
        currCount += 1;
    }    
   
   
    if  windowCount[1]()>5 {
        
       initForFirstWindow();
       signed[1](-1);
       signed[1](1);
    }
    unsafe { * (regs[0] as *mut u32) =window_count;}
    oldr14 as c_int
    
}


const listOfCrosses:[usize;2]=[0x143d34a90 ,0x143d35cc0];


fn aftersetCrossGregar(){

   

    let mut count= unsafe{  *(addrOfNumcrosses[0] as *mut u8) };
    let mut structOfCrossFaces:*const u32 =listOfCrosses[0] as *const u32;
    while (count >0) {
        {  
            unsafe {
            let crosstoDraw=*structOfCrossFaces as u8;
            let indextoDraw = *structOfCrossFaces.wrapping_add(1) as u8;
            let currCrossSelected = *structOfCrossFaces.wrapping_add(2) as u8;
            let selected = *structOfCrossFaces.wrapping_add(3) as u8;
            setCrosses(crosstoDraw, indextoDraw, currCrossSelected, selected, 0)
            }
        }

        count-=1;
        structOfCrossFaces=structOfCrossFaces.wrapping_add(4);
    }
    unsafe{ 
        let pointertoregs=*bn6fun::GLOBALGBAREG.get().unwrap() as *mut u32;
        *pointertoregs=8;
        *(0x143b3d431 as *mut u8)=0;
    }

}

fn aftersetCrossFalzar(){

    let mut count= unsafe{  *(addrOfNumcrosses[1] as *mut u8) };
    let mut structOfCrossFaces:*const u32 =listOfCrosses[1] as *const u32;
    while (count >0) {
        {  
            unsafe {
            let crosstoDraw=*structOfCrossFaces as u8;
            let indextoDraw = *structOfCrossFaces.wrapping_add(1) as u8;
            let currCrossSelected = *structOfCrossFaces.wrapping_add(2) as u8;
            let selected = *structOfCrossFaces.wrapping_add(3) as u8;
            setCrosses(crosstoDraw, indextoDraw, currCrossSelected, selected, 0)
            }
        }

        count-=1;
        structOfCrossFaces=structOfCrossFaces.wrapping_add(4);
    }
    unsafe{ 
        let pointertoregs=*bn6fun::GLOBALGBAREG.get().unwrap() as *mut u32;
        *pointertoregs=8;
        *(0x143b3d557 as *mut u8)=0;
    }

}


const facesinram:[u64;2]= [0x143d33bc0,0x143d35040];


fn gregarsetCross(notusefuldata:u64,indextoDraw:u8){
    let mut structOfCrossFaces:*const u32=listOfCrosses[0] as *const u32;
    structOfCrossFaces=structOfCrossFaces.wrapping_add((indextoDraw<<2) as usize);
    unsafe {
        
    setCrosses(*structOfCrossFaces as u8, *structOfCrossFaces.wrapping_add(1) as u8, *structOfCrossFaces.wrapping_add(2) as u8, *structOfCrossFaces.wrapping_add(3) as u8,1);
    let update= (((  *bn6fun::GLOBALGBAREG.get().unwrap() as u64)+0x34) as *mut u32);
    *update=*update-4;

    }
        
}

fn falzarsetCross(notusefuldata:u64,indextoDraw:u8){
    let mut structOfCrossFaces:*const u32=listOfCrosses[1] as *const u32;
    structOfCrossFaces=structOfCrossFaces.wrapping_add((indextoDraw<<2) as usize);
    unsafe {
        
    setCrosses(*structOfCrossFaces as u8, *structOfCrossFaces.wrapping_add(1) as u8, *structOfCrossFaces.wrapping_add(2) as u8, *structOfCrossFaces.wrapping_add(3) as u8,1);
    let update= (((  *bn6fun::GLOBALGBAREG.get().unwrap() as u64)+0x34) as *mut u32);
    *update=*update-4;

    }
        
}

fn setCrosses(crosstoDraw:u8,indextoDraw:u8,currCrossSelected:u8,selected:u8, amountToDraw:u8) {
   let gameversion=gamever.lock().unwrap();
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
    drawCrossFaces( *(facesinram[*gameversion as usize] as *mut u64),
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
            
          
           
                let mut data = beasticonlocs.lock().unwrap();
                  data[0]=curraddress as u32;
                
               
            curraddress+=beasticon as u64;
            let beastchiptech =&rom[0x74550C..0x74550C+beastchipsize];
            memcopy!(gbamemorymap,curraddress,beastchipsize,beastchiptech);
            
           
                let mut data = beastchipIcon.lock().unwrap();
                  data[0]=curraddress as u32;
                
            
            curraddress+=beastchipsize as u64;
            }
            let beastpal =&rom[0x747E8C ..0x747E8C +palettesize];
            memcopy!(gbamemorymap,curraddress,palettesize,beastpal);
            
           
                let mut data = beastpallocs.lock().unwrap();
                  data[0]=curraddress as u32;
                
               

            curraddress+=palettesize as u64;

            curraddress=writeNewPointer(0,curraddress, rom);
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
            
           
           
                let mut data = beasticonlocs.lock().unwrap();
                  data[1]=curraddress as u32;
                
             
            curraddress+=beasticon as u64;

              let beastchiptech =&rom[0x7475D8..0x7475D8+beastchipsize];
            memcopy!(gbamemorymap,curraddress,beastchipsize,beastchiptech);
           
           
                let mut data = beastchipIcon.lock().unwrap();
                  data[1]=curraddress as u32;
                
             
            curraddress+=beastchipsize as u64; 

            let beastpal =&rom[0x749F58 ..0x749F58 +palettesize];
            memcopy!(gbamemorymap,curraddress,palettesize,beastpal);
          
           
                let mut data = beastpallocs.lock().unwrap();
                  data[1]=curraddress as u32;
                
               

            curraddress+=palettesize as u64;

            
            curraddress=writeNewPointer(1,curraddress, rom);


            }
           break;
            
         
         }
    }
    }
    }

}


}

fn resetSound(){
    unsafe{*(0x141f976aA as *mut u8)=0x92;}
    unsafe{ *(0x142985ddA as *mut u8 )=0x94;}
}

fn falzarnewgame(gbareg:*mut u64)->c_int{
    bn6fun::GLOBALGBAREG.get_or_init(|| gbareg as u64);
    GBASTRUCT::gba.get_or_init(|| GBASTRUCT::init());
    
    let mut curr= gamever.lock().unwrap();
    *curr=1;

    resetSound();
    
    setfaces();
    let gbamemorymap= unsafe{*(((  *bn6fun::GLOBALGBAREG.get().unwrap() as u64)+0x48) as *const u64)} as *mut u8;
    write_u8!(gbamemorymap,0x4625c,0x1);   //Don't Reselect cross
    write_u8!(gbamemorymap,0x46260,0xD); //BeastOut
    write_u8!(gbamemorymap, 0x465e8,0x1);
    let val=bn6fun::NEWGAMEINITFALZAR(gbareg);
    val
}
fn falzarcontinue(gbareg: *mut u64 ) {
    bn6fun::GLOBALGBAREG.get_or_init(|| gbareg as u64);
    GBASTRUCT::gba.get_or_init(|| GBASTRUCT::init());
    
  
    let mut curr= gamever.lock().unwrap();
    *curr=1;
    resetSound();

    setfaces();
    let gbamemorymap= unsafe{*(((  *bn6fun::GLOBALGBAREG.get().unwrap() as u64)+0x48) as *const u64)} as *mut u8;
    write_u8!(gbamemorymap,0x4625c,0x1);   
    write_u8!(gbamemorymap,0x46260,0xD);
    write_u8!(gbamemorymap, 0x465e8,0x1);
    //let gbamemorymap= unsafe{*(((gbareg as u64)+0x48) as *const u64)};
    bn6fun::IDKWHATTHIS(gbareg,gbareg as *mut u32,1);

}

fn gregarnewgame(gbareg:*mut u64)->c_int{
    bn6fun::GLOBALGBAREG.get_or_init(|| gbareg as u64);
    GBASTRUCT::gba.get_or_init(|| GBASTRUCT::init());
    
    let mut curr= gamever.lock().unwrap();
    *curr=0;

    resetSound();
    
    setfaces();
    
    let val=bn6fun::NEWGAMEINIT(gbareg);
    val
}
fn gregarcontinue(gbareg: *mut u64 ) {
    bn6fun::GLOBALGBAREG.get_or_init(|| gbareg as u64);
    GBASTRUCT::gba.get_or_init(|| GBASTRUCT::init());
    
  
    let mut curr= gamever.lock().unwrap();
    *curr=0;
    resetSound();

    setfaces();
    //let gbamemorymap= unsafe{*(((gbareg as u64)+0x48) as *const u64)};
    bn6fun::IDKWHATTHIS(gbareg,gbareg as *mut u32,1);

}
const beastoffsetver:[u32;2]=[0x46B44,0x46254];
const beasticonInLabelsVer:[u32;2]=[0x42C7C,0x42564];
const beasticonInLabels2Ver:[u32;2]=[0x466F8,0x45E08];
const beastchipIconVer:[u32;2]=[0x40950,0x40238];
const beastchipicon2Ver:[u32;2]=[0x46810,0x45F20];
const beastSoundVer:[u32;2]=[0x46ec0,0x465D0];
fn customscreen_effects() {
   let gbareg= &GBASTRUCT::gba.get_or_init(|| GBASTRUCT::init()).registers;
   let pointertoregs: *mut u64=*bn6fun::GLOBALGBAREG.get().unwrap() as *mut u64;
 {
   let mut curvalue= BEAST.lock().unwrap();
  
 if *curvalue==1 {
    let gameversion=gamever.lock().unwrap();
         *curvalue=0;   
        let ram=unsafe{*( (gbareg[0] as *mut u32).wrapping_add(0x48>>2) as *mut u64)} as *mut u8;
        let mut beastoffset=beastoffsetver[*gameversion as usize];
        let beasticonInLabels=beasticonInLabelsVer[*gameversion as usize]>>2;
        let beasticonInLabels2 =beasticonInLabels2Ver[*gameversion as usize]>>2;
        let beastchipicon =beastchipIconVer[*gameversion as usize]>>2;
        let beastchipicon2 = beastchipicon2Ver[*gameversion as usize]>>2;
        let currbeast=read_u8!(ram,beastoffset);
        let beastOthervar=beastSoundVer[*gameversion as usize];
        let mut sound=0x92;
        if (currbeast==0xB) {
            write_u8!(ram,beastoffset,0xC);
            beastoffset+=4;
            write_u8!(ram,beastoffset,0x18);
            sound =0x94;
            write_u8!(ram,beastOthervar,0xC);
            
        }
        else {
            write_u8!(ram,beastoffset,0xB);
            beastoffset+=4;
            write_u8!(ram,beastoffset,0x17);
            write_u8!(ram,beastOthervar,0xB);
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
        unsafe{ *(0x142985ddA as *mut u8 )=sound;}
        unsafe{*(gbareg[0] as *mut u32)=(sound as u32)+0xFF};
        soundRequest[*gameversion as usize](pointertoregs);
        
 }


    
  
  else {
  let offset=  unsafe{*(gbareg[5] as *mut u32)}+0x1A;   
    let ram=unsafe{*( (gbareg[0] as *mut u32).wrapping_add(0x48>>2) as *mut u64)} as *mut u8;
    let crossbyte=read_u8!(ram,offset);
    unsafe {
    *((gbareg[0]) as *mut u32)=crossbyte as u32;   
    }
    {
        
    custom_cross_kokoro_change_set[ {*gamever.lock().unwrap()} as usize]( pointertoregs);
    custom_custom_move_cross_select_sub2[ {*gamever.lock().unwrap()} as usize](pointertoregs);
    custom_move_cross_select_sub[ {*gamever.lock().unwrap()} as usize](pointertoregs);
    custom_paint_datawindow[ {*gamever.lock().unwrap()} as usize](pointertoregs);
    }
    unsafe {
        *((gbareg[0]) as *mut u32)=0x92;   
        }
    soundRequest[{*gamever.lock().unwrap()} as usize](pointertoregs);
    }
}
}

fn beastCheck() ->c_int {
    let gbareg=    &GBASTRUCT::gba.get_or_init(|| GBASTRUCT::init()).registers;

    let pointertoregs: *mut u64=*bn6fun::GLOBALGBAREG.get().unwrap() as *mut u64;
    let currentChip= unsafe {
        *((gbareg[0]) as *mut u32) as u8  
        };
    let gameversion=MutexValue!(gamever) as usize;
    let ram=unsafe{*( (gbareg[0] as *mut u32).wrapping_add(0x48>>2) as *mut u64)} as *mut u8;   
    if currentChip==0xB{
     
        let  check1= unsafe{*(gbareg[5] as *mut u32)}+8;
        let check2= unsafe{*(gbareg[4] as *mut u32)}+7;  
        let check1= read_u8!(ram,check1);
        let check2 = read_u8!(ram,check2);
        if check2==0 && check1<5 {
            _=  thread::spawn(move || {
           
                let mut data = BEAST.lock().unwrap();
                *data = 1;
                
            }).join();
        let mut offset=  unsafe{*(gbareg[5] as *mut u32)}+1;  
            write_u8!(ram,offset,0x5C);
        offset+=1;
           write_u8!(ram,offset,0x0);    
          unsafe{ *(addrOfNumcrosses[gameversion] as *mut u8)=0;}
        }
    }
     chip_address[gameversion](pointertoregs)
}

#[no_mangle]
pub  extern "C" fn luaopen_make(_:c_void)-> c_int{

    hooks::GM_HOOK!(0x141edebcb,gregarnewgame,16);
    hooks::GM_HOOK!(0x1428e191b,falzarnewgame,16);
    
    hooks::GM_HOOK!(0x141edec8e,gregarcontinue,14);
    hooks::GM_HOOK!(0x1428e19de,falzarcontinue,14);

    hooks::GM_HOOK!(0x141f9a1ab,customscreen_effects,14);
    hooks::GM_HOOK!(0x1429887ab,customscreen_effects,14);
    unsafe {
      *(0x141F9A1B9 as *mut u8)=0xEB;
     *(0x141F9A1BA as *mut u8)=0x7B;
      *( 0x1429887B9 as *mut u8)=0xEB;
      *( 0x1429887BA as *mut u8)=0x7B;

                           //jump
  }
     hooks::GM_HOOK!(0x1423ac324,beastCheck,15);   
     hooks::GM_HOOK!(0x142d36f04,beastCheck,15);


    hooks::GM_HOOK!( 0x141f991e8,gregarsetCross,15 );//0x141f8c960,
     hooks::GM_HOOK!(0x142987918,falzarsetCross,15);
     
     hooks::GM_HOOK!(0x141f99ff3,aftersetCrossGregar,25);  //0x141f8cbc0
    hooks::GM_HOOK!(0x1429885f3,aftersetCrossFalzar,25);
    
   //load headcross palette
    unsafe {

        *(0x1423bd077 as *mut u8)=0x0;
        
       *(0x142d45697 as *mut u8)=0x0;

        *(0x1423bcd13 as *mut u8)=0xBA;
        *(0x1423bcd14 as *mut u8)=0x0;
        *(0x1423bcd15 as *mut u8)=0x0;
        *(0x1423bcd16 as *mut u8)=0x0;
        *(0x1423bcd17 as *mut u8)=0x0;
        *(0x1423bcd18 as *mut u8)=0x90;

        *(0x142d45333 as *mut u8)=0xBA;
        *(0x142d45334 as *mut u8)=0x0;
        *(0x142d45335 as *mut u8)=0x0;
        *(0x142d45336 as *mut u8)=0x0;
        *(0x142d45337 as *mut u8)=0x0;
       *(0x142d45338 as *mut u8)=0x90;



    }
    install_hooks_cross_check();
    //hooks::GM_HOOK!(0x1423bd7d0,CrossWindowAddCross,13);
    //hooks::GM_HOOK!(0x142d45e30,CrossWindowAddCrossFalzar,13);
    //unsafe
   // {
       // *(0x1423bd7dd as *mut u8)  = 0xC3;   
     //   *(0x142d45e3d as *mut u8 )=0xC3;
   //  }
   hooks::GM_HOOK!(0x1423abc43,scroll,22);
   hooks::GM_HOOK!(0x142d36813,scroll,22);
   hooks::GM_HOOK!(0x14245cf66,setVersionFaceGregar,21);
   hooks::GM_HOOK!(0x142dd0d76 ,setVersionFaceFalzar,21);
   hooks::GM_HOOK!(0x14245d484,setDefaultGregarFace,17);
   hooks::GM_HOOK!(0x142dd1294,setDefaultFalzarFace,17);
     
     unsafe{*(0x142d3d24A as *mut u8)=1;}
    0
}