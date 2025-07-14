use aes::cipher::{Block, block_padding::NoPadding, BlockDecryptMut, 
    generic_array::GenericArray, BlockEncryptMut, KeyIvInit};

use crate::arch_get_cpu_time;

use crate::service::register_mmkapi;
use crate::{nkapi, nkapi_return_err, nkapi_return_ok};
use typenum::{UInt, UTerm, B1, B0};

use lazy_static::lazy_static;  
use spin::Mutex;
use alloc::vec::Vec; 
use alloc::boxed::Box;

type Aes256CbcEnc = cbc::Encryptor<aes::Aes256>;
type Aes256CbcDec = cbc::Decryptor<aes::Aes256>;

pub fn app_init(){
    register_mmkapi(22, app_handler as usize);
}

struct Aes_s {
    enc: Aes256CbcEnc,
    dec: Aes256CbcDec,
    id: usize,
}

impl Aes_s {
    pub fn new(id: usize, key: &[u8; 32], iv: &[u8; 16]) -> Self {

        // little-endian.
        Aes_s {
            id: id,
            enc: Aes256CbcEnc::new(key.into(), iv.into()),
            dec: Aes256CbcDec::new(key.into(), iv.into())
        }
        
    }
    pub fn id(&self ) -> usize {
        return self.id;
    }
    pub fn encrypt(&mut self, data: &mut [u8], data_len: usize) -> usize {
        if data_len % 16 != 0 {
            panic!("Data is not padded in AESCBC.");
        }
        // return time usage for evaluation
        let time_start: usize = arch_get_cpu_time();

        let mut tmp_buf: [u8; 16] = [0; 16];
        let blks_out = Block::<Aes256CbcDec>::from_mut_slice(&mut tmp_buf);
    
        for i in 0..(data_len/16) {
            let slice = &mut data[(i*16)..((i+1)*16)];
            let blks_in = Block::<Aes256CbcDec>::from_mut_slice(slice);
            
            //let arr: Block<Aes256CbcDec> = [0 as u8; 16].into();
            self.enc.encrypt_block_b2b_mut(blks_in, blks_out);
            blks_in.copy_from_slice(blks_out);
        }
        //self.enc.clone_from(&oper);

        let time_end: usize = arch_get_cpu_time();
        return time_end - time_start;
    }
    pub fn decrypt(&mut self, data: &mut [u8], data_len: usize) -> usize {
        if data_len % 16 != 0 {
            panic!("Data is not padded in AESCBC.");
        }

        // return time usage for evaluation
        let time_start: usize = arch_get_cpu_time();

        let mut tmp_buf: [u8; 16] = [0; 16];
        let blks_out = Block::<Aes256CbcDec>::from_mut_slice(&mut tmp_buf);
        
        for i in 0..(data_len/16) {
            let slice = &mut data[(i*16)..((i+1)*16)];
            let blks_in = Block::<Aes256CbcDec>::from_mut_slice(slice);
            
            //let arr: Block<Aes256CbcDec> = [0 as u8; 16].into();
            self.dec.decrypt_block_b2b_mut(blks_in, blks_out);
            blks_in.copy_from_slice(blks_out);
        }

        let time_end: usize = arch_get_cpu_time();
        return time_end - time_start;

        //self.dec.clone_from(&oper);
    }

}

lazy_static! {
    static ref AES_SESSION_LIST: Mutex<Vec<Aes_s>> = Mutex::new(Vec::<Aes_s>::new());
}

macro_rules! aes_operate {
    ($handle:expr, $target:ident, $oper:block) => {
        let mut _find = false;
        let mut ptlist = AES_SESSION_LIST.lock();
        for tar in ptlist.iter_mut(){
            if tar.id() == $handle {
                _find = true;

                //let $target: Aes_s = ptlist.remove(tar);
                
                let $target: &mut Aes_s = tar;

                $oper

                //ptlist.push($target);
            }
        }
        if !_find {
            nkapi_return_err!(2 as usize);
        }
    };
}


nkapi!{
    fn app_handler(id: usize, session: usize, buf_n: usize, siz: usize) -> usize {
        debug_info_level!(3,"ekcapi_aescbc({:x}, {:x}, {:x}, {:x})", id, session, buf_n, siz);

        unsafe{
            //let mut out_buf_tmp: &mut [u8] = &mut *(&mut out_buf_tmp_alloc as *mut [u8; 4096] as usize as *mut [u8; 256]);;
            //TODO: check the validity of these pointer.
            //TODO: check size of the buffer.

            match id {
                0 => {
                    let mut ptlist = AES_SESSION_LIST.lock();
                    for tar in 0..ptlist.len(){
                        if ptlist[tar].id() == session {
                            ptlist.remove(tar);
                            break;
                        }
                    }

                    let key_buf: &[u8; 32] = &*(buf_n as *mut [u8; 32]);
                    let iv_buf: &[u8; 16] = &*(siz as *mut [u8; 16]);

                    //big/small-endian key construct.
                    let mut e_key: [u8; 32] = [0;32];
                    for word in 0..8 {
                        for bt in 0..4 {

                            //TODO: choose a kind of key.

                            //big endian:
                            e_key[word*4 + bt] = key_buf[word*4 + 3 - bt];

                            //small endian:
                            //e_key[word*4 + bt] = key_buf[word*4 + bt];
                        }
                    }

                    ptlist.push(
                        Aes_s::new(session, &e_key, iv_buf)
                    );
                    
                    
                    // debug_info!("key is:");

                    // for i in 0..32 {
                    //     print!("{:x} ", key_buf[i]);
                    // }
                    // debug_info!("\n iv is: ");
                    // for i in 0..16 {
                    //     print!("{:x} ", iv_buf[i]);
                    // }
                    // debug_info!("\n");
                    nkapi_return_ok!(session);
                }
                1 => {
                    if siz % 16 != 0 || siz < 0 || siz > 32768 {
                        debug_info_level!(0,"Invalid data input size in AES: {:x}", siz);
                        nkapi_return_err!(1);
                    }
                    let buf: &mut [u8] = &mut *(buf_n as *mut [u8; 65539]);
                    let mut time: usize = 0;
                    aes_operate!(session, target_aes, {
                        time = target_aes.encrypt(buf, siz);
                    });

                    print!("aes enc time cost: {}\n", time);
                    print!("warn: if you are evaluating SSH time cost, please comment this message.\n");
                
                    // aes256_cbc_encrypt(mod_buf, siz, &key, &iv);
                    // debug_info!("Encrypted: ");
                    // for i in 0..siz {
                    //     print!("{:x} ", buf[i]);
                    // }
                    // print!("\n");
                }
                2 => {
                    if siz % 16 != 0 || siz < 0 || siz > 32768 {
                        debug_info_level!(0,"Invalid data input size in AES: {:x}", siz);
                        nkapi_return_err!(1);
                    }
                    let buf: &mut [u8] = &mut *(buf_n as *mut [u8; 65539]);
                    let mut time: usize = 0;
                    aes_operate!(session, target_aes, {
                        time = target_aes.decrypt(buf, siz);
                    });
                    //aes256_cbc_decrypt(mod_buf, siz, &key, &iv);
                    // debug_info!("Decrypted: ");
                    // for i in 0..siz {
                    //     print!("{:x} ", buf[i]);
                    // }
                    // print!("\n");
                    
                    //print!("aes dec time cost: {}\n", time);
                    //print!("warn: if you are evaluating SSH time cost, please comment this message.\n");
                }
                _ => {

                }
            }

            nkapi_return_ok!(siz);
        }
        
    }
}
