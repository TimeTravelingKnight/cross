macro_rules! GM_HOOK {
  ($address:literal,$func_name:ident,$bytes:literal) => {
      let slice=unsafe{slice::from_raw_parts_mut($address as *mut u8, $bytes)};
      let _ = &slice[0..2].copy_from_slice(&[0x48,0xB8]); 


      let _ = &slice[2..10].copy_from_slice(&($func_name as u64).to_le_bytes());
   
      let _ = &slice[10..12].copy_from_slice(&[0xFF,0xD0]);
      if ($bytes>12) {
      let _ = &slice[12..].copy_from_slice(&[0x90;($bytes-12)]);    
      }
  };
}
pub(crate) use GM_HOOK; 