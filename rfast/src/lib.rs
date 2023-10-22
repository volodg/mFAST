use crate::value_storage::ValueStorage;

mod value_storage;
mod instructions;

use intbits::Bits;

#[no_mangle]
pub extern "C" fn set_uint64_defined_bit(storage: *mut ValueStorage, defined: bool) {
    unsafe {
        (*storage).of_uint64.defined_bit_.set_bit(0, defined);
    }
}

#[no_mangle]
pub extern "C" fn set_uint64_present(storage: *mut ValueStorage, present: bool) {
    unsafe {
        (*storage).of_uint64.present_.set_bit(0, present);
    }
}

#[cfg(target_pointer_width = "64")]
#[no_mangle]
pub extern "C" fn set_uint64_set_value(storage: *mut ValueStorage, value: u64) {
    unsafe {
        (*storage).set(value);
    }
}

#[cfg(target_pointer_width = "32")]
#[no_mangle]
pub extern "C" fn set_uint64_set_value(storage: *mut ValueStorage, value: u32) {
    unsafe {
        (*storage).set(value);
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
