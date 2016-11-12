use data_encoding::base64url;
use lzf;
use error::*;
use rand::{thread_rng,Rng};

pub fn write(text: &str) -> Result<String> {
  trace!("write(text:{:?})", text);
  let bytes = text.as_bytes();
  trace!("Bytes    : {:?}", bytes);
  let compressed = try!(lzf::compress(bytes));
  trace!("Compress : {:?}", compressed);
  let slice = compressed.as_slice();
  let encoded = base64url::encode(slice);
  trace!("Encode   : {:?}", encoded);
  Ok(encoded)
}

pub fn lookup(index: &str) -> Result<String> {
  trace!("lookup(index:{:?})", index);
  let decoded = try!(base64url::decode(index.as_bytes()));
  trace!("Decode     : {:?}", decoded);
  let decomped = try!(lzf::decompress(&decoded, 1000));
  trace!("Decompress : {:?}", decomped);
  let raw = try!(String::from_utf8(decomped));
  trace!("Raw        : {:?}", raw);
  Ok(raw)
}

pub fn gen_rand() {
  let mut rng = thread_rng();
  if rng.gen() { // random bool
      println!("i32: {}, u32: {}", rng.gen::<i32>(), rng.gen::<u32>())
  }
}