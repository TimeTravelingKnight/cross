
macro_rules! MutexValue{
    ($mutexlock:ident,$value:literal) => {
      thread::spawn(move || {
        let mut data=$mutexlock.lock().unwrap();
        *data=$value;
      } ).join().unwrap();
    };
    ($mutexlock:ident) => {
      thread::spawn(move || {
        let data=$mutexlock.lock().unwrap();
         *data
      } ).join().unwrap()
    };
    ($mutexlock:ident,$value:ident) => {
      thread::spawn(move || {
        let data=$mutexlock.lock().unwrap();
        *data[$value]
      } ).join().unwrap()
    };
}

macro_rules! memcopy {
  ($ram:ident,$offset:ident,$bytes:ident,$src:ident) => {
       
      let slice=unsafe{slice::from_raw_parts_mut(($ram+$offset) as *mut u8, $bytes )};
      let _ = &slice.copy_from_slice($src); 


  };
}
pub(crate) use memcopy; 


macro_rules! grab_u32{
  ($ram:ident,$offset:literal) => {
     unsafe{*$ram.wrapping_add($offset as usize)}
};
  ($ram:ident,$offset:ident) => {
  unsafe{*$ram.wrapping_add($offset as usize)}
}

}
macro_rules! write_u32{
  ($ram:ident,$offset:literal,$value:ident) => {
   unsafe {*($ram.wrapping_add($offset as usize) as *mut u32) =$value};
  };
  ($ram:ident,$offset:literal,$value:literal) => {
    unsafe {*($ram.wrapping_add($offset as usize) as *mut u32 ) =$value};
  };
  ($ram:ident,$offset:ident,$value:ident) => {
    unsafe {*($ram.wrapping_add($offset as usize) as *mut u32) =($value as u32)};
  };
  ($ram:ident,$offset:ident,$value:literal) => {
    unsafe {*($ram.wrapping_add($offset as usize) as *mut u32) =$value};
  }; 

}

macro_rules! write_u8{
  ($ram:ident,$offset:literal,$value:ident) => {
   unsafe {*($ram.wrapping_add($offset as usize) as *mut u8) =$value};
  };
  ($ram:ident,$offset:literal,$value:literal) => {
    unsafe {*($ram.wrapping_add($offset as usize) as *mut u8 ) =$value};
  };
  ($ram:ident,$offset:ident,$value:ident) => {
    unsafe {*($ram.wrapping_add($offset as usize) as *mut u8) =$value};
  };
  ($ram:ident,$offset:ident,$value:literal) => {
    unsafe {*($ram.wrapping_add($offset as usize) as *mut u8) =$value};
  }; 

}


macro_rules! read_u8{
  ($ram:ident,$offset:literal) => {
   unsafe {*($ram.wrapping_add($offset as usize) as *mut u8)};
  };
  ($ram:ident,$offset:literal) => {
    unsafe {*($ram.wrapping_add($offset as usize) as *mut u8 )};
  };
  ($ram:ident,$offset:ident) => {
    unsafe {*($ram.wrapping_add($offset as usize) as *mut u8)};
  };
  ($ram:ident,$offset:ident) => {
    unsafe {*($ram.wrapping_add($offset as usize) as *mut u8)};
  }; 

}



pub(crate) use MutexValue;
pub(crate) use grab_u32;
pub(crate) use write_u32;
pub(crate) use write_u8; 
pub(crate) use read_u8;