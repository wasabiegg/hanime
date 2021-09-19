use aes::Aes128;
use block_modes::block_padding::NoPadding;
use block_modes::{BlockMode, Cbc};
use bytes::BytesMut;
use std::error::Error;

type Aes128Cbc = Cbc<Aes128, NoPadding>;
pub fn decrypt<'a>(src: &'a mut BytesMut) -> Result<&[u8], Box<dyn Error>> {
    // let key = hex!("00010203040506070001020304050607");
    // let iv = hex!("00010203040506070001020304050607");

    let key = b"0123456701234567";
    let iv = b"0123456701234567";

    let cipher = Aes128Cbc::new_from_slices(key, iv).unwrap();
    match cipher.decrypt(&mut src[..]) {
        Ok(i) => {
            let length = i.len();
            let un_padding = i[length - 1] as usize;
            return Ok(&i[..length - un_padding]);
        }
        Err(e) => {
            eprintln!("dd {:#?}", e);
            return Err(Box::new(e));
        }
    }
}
