use std::borrow::Cow;
use std::result;
use std::error::Error as stdError;
use std::str::Utf8Error;
use std::string::FromUtf8Error;
use lzf::LzfError;
use data_encoding::decode::Error as DecodeError;

error_type! {
  #[derive(Debug)]
  pub enum Error {
    Decode(DecodeError) {
      cause;
    },
    Compress(LzfError) {
      desc (_e) "Error with Lzf compression";
    },
    ConvertStr(Utf8Error) {
      cause;
    },
    ConvertString(FromUtf8Error) {
      cause;
    },
    Message(Cow<'static, str>) {
      desc (e) &**e;
      from (s: &'static str) s.into();
      from (s: String) s.into();
    },
    Other(Box<Error>) {
      desc (e) e.description();
      cause (e) Some(&**e);
    },
  }
}

pub type Result<T> = result::Result<T, Error>;
