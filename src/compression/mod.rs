// #![allow(non_upper_case_globals)]
// #![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
mod bindings;

#[derive(Debug)]
pub enum Error {
    InputTooLarge,
    OutputTooLarge,
    ReturnCode(i32),
}

pub fn unpack_to_buf(src_data: &[u8], dest_data: &mut [u8]) -> Result<(), Error> {
    if src_data.len() >= u32::MAX as usize {
        return Err(Error::InputTooLarge);
    }

    if dest_data.len() >= u32::MAX as usize {
        return Err(Error::OutputTooLarge);
    }

    let return_code = unsafe {
        let src: *const u8 = src_data.as_ptr();
        let src_size = src_data.len() as u32;

        let mut dest: *mut u8 = dest_data.as_mut_ptr();
        let mut dest_size: u32 = dest_data.len() as u32;

        bindings::Unpack(src, src_size, &mut dest, &mut dest_size)
    };

    if return_code == 0 {
        Ok(())
    } else {
        Err(Error::ReturnCode(return_code))
    }
}

pub fn unpack_sized(src_data: &[u8], size: usize) -> Result<Vec<u8>, Error> {
    let mut dest_data = vec![0u8; size as usize];

    unpack_to_buf(src_data, &mut dest_data)?;

    Ok(dest_data)
}

pub fn pack(src_data: &[u8]) -> Result<Vec<u8>, Error> {
    if src_data.len() >= u32::MAX as usize {
        return Err(Error::InputTooLarge);
    }

    let result_vec = unsafe {
        let src: *const u8 = src_data.as_ptr();
        let src_size = src_data.len() as u32;

        let mut dest: *mut u8 = std::ptr::null_mut();
        let mut dest_size: u32 = 0;

        // The C++ code does not expect any case where this can fail.
        // Its return code does not indicate success or failure.
        bindings::Pack(src, src_size, &mut dest, &mut dest_size);

        let mut target_vec = vec![0u8; dest_size as usize];
        std::ptr::copy_nonoverlapping(dest, target_vec.as_mut_ptr(), dest_size as usize);

        bindings::PackFree(dest);

        target_vec
    };

    Ok(result_vec)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pack_matches_unpack() {
        let original_data = (0..1_000_000)
            .map(|x| (x & 0xFF) as u8)
            .collect::<Vec<u8>>();

        let compressed = match pack(&original_data) {
            Ok(x) => x,
            Err(why) => {
                assert!(false, "Compression error: {:?}", why);
                return;
            }
        };

        let uncompressed = match unpack_sized(&compressed, original_data.len()) {
            Ok(x) => x,
            Err(why) => {
                assert!(false, "Decompression error: {:?}", why);
                return;
            }
        };

        assert_eq!(
            original_data, uncompressed,
            "Original data does not match uncompressed result"
        );
    }
}
