use data_encoding::base64url;
use error::*;
use rand::thread_rng;
use text_generator::TextGenerator;
use std;
use redux;
use redux::model::{Parameters,AdaptiveTreeModel};

pub fn encode(text: &str) -> Result<String> {
    trace!("write(text:{:?})", text);
    let bytes = text.as_bytes();
    trace!("Bytes     : {:?}", bytes);
    let compressed = compress(bytes);
    let slice = compressed.as_slice();
    let encoded = base64url::encode(slice);
    trace!("Encode    : {:?}", encoded);
    Ok(encoded)
}

pub fn decode(index: &str) -> Result<String> {
    trace!("lookup(index:{:?})", index);
    let decoded = try!(base64url::decode(index.as_bytes()));
    trace!("Decode     : {:?}", decoded);
    let decomped = decompress(&decoded);
    let raw = try!(String::from_utf8(decomped));
    trace!("Raw        : {:?}", raw);
    Ok(raw)
}

pub fn gen_rand_text<'a>() -> String {
    let mut rng = thread_rng();
    let tg = TextGenerator::base_28(&mut rng);
    let text: String = tg.take(20).collect();
    text
}

fn compress(data: &[u8]) -> Vec<u8> {
    let mut cursor = std::io::Cursor::new(&data);
    let mut compressed = Vec::<u8>::new();
    let cnt_in_out = redux::compress(
        &mut cursor,
        &mut compressed,
        AdaptiveTreeModel::new(Parameters::new(8, 14, 16).unwrap()));
    trace!("Compressed {:?}", cnt_in_out);
    compressed
}

fn decompress(data: &[u8]) -> Vec<u8> {
    let mut cursor = std::io::Cursor::new(&data);
    let mut decompressed = Vec::<u8>::new();
    let cnt_in_out = redux::decompress(
        &mut cursor,
        &mut decompressed,
        AdaptiveTreeModel::new(Parameters::new(8, 14, 16).unwrap()));
    trace!("Decompressed {:?}", cnt_in_out);
    decompressed
}

// 0-9 == 48-57
// a-z == 97-122
// A-Z == 65-90
// ' ' == 32; '.' == 46; '!' == 33; '?' == 63
// printable == 32-126

#[test]
fn gen_text_produces() {
    let actual = gen_rand_text();
    assert!(actual.len() > 0);
}

#[test]
fn encode_decode_should_invert() {
    let text = "hello world this is a test of the compression algorithm and how well it can deal \
                with text and stuff so we write a whole lot of things and say hello to it and \
                then cast it out into the world to see how that compression works and if it can \
                shrink this up by much and unshrink it and so on and on and on and on and on \
                and then some";
    let index = encode(text).unwrap();
    let result = decode(&index).unwrap();
    assert_eq!(result, text);
}

// #[test]
// fn test_compression_formats() {
//     use std;
//     use redux;
//     use redux::model::*;
//     use lzma;

//     let mut rng = thread_rng();
//     let tg = TextGenerator::base_28(&mut rng);
//     let text: String = tg.take(3200).collect();
//     let data = text.as_bytes();

//     // Encode
//     let mut cursor1 = std::io::Cursor::new(&data);
//     let mut red_compressed = Vec::<u8>::new();
//     let rd_out = redux::compress(&mut cursor1,
//                     &mut red_compressed,
//                     AdaptiveTreeModel::new(Parameters::new(8, 14, 16).unwrap()));
//     let red_encoded = base64url::encode(red_compressed.as_slice());

//     let lz_compressed = lzma::compress(data, 9 | lzma::EXTREME_PRESET).unwrap();
//     let lz_encoded = base64url::encode(lz_compressed.as_slice());

//     println!("RD:{:?}", rd_out);
//     println!("RD:{}:{}",red_encoded.len(),&red_encoded[0..10]);
//     println!("LZ:{}:{}",lz_encoded.len(),&lz_encoded[0..10]);
// }

//helloworldhelloworldhelloworld
//Z_w-Sp2r-kH8ZD-vozuv7SjftH5Ct4yF9FM2AA==
//hellohellohellohelloworldworldworldworldhelloworldhelloworldhelloworld
//Z_w-Sp2Jxn6yOUt5RyL65DQ9JrMxyoU0Kv9K3UnOmh-kMmEDh3seAGNy6qv28RlwG5W8JHyA
//the quick brown fox jumped over the lazy dog
//c_MsYzHFEBI-xNYGFXWDrAXXc6l3FRMRbLtbzz5oPgo286ZqyV7wdJgXBA==