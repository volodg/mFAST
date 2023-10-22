use std::ffi::c_char;
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

#[no_mangle]
pub extern "C" fn set_decimal_defined_bit(storage: *mut ValueStorage, defined: bool) {
    unsafe {
        (*storage).of_decimal.defined_bit_.set_bit(0, defined);
    }
}

#[no_mangle]
pub extern "C" fn set_decimal_present(storage: *mut ValueStorage, present: bool) {
    unsafe {
        (*storage).of_decimal.present_.set_bit(0, present);
    }
}

#[no_mangle]
pub extern "C" fn set_decimal_mantissa_and_exponent(storage: *mut ValueStorage, mantissa: i64, exponent: i16) {
    unsafe {
        (*storage).of_decimal.mantissa_ = mantissa;
        (*storage).of_decimal.exponent_ = exponent;
    }
}

#[no_mangle]
pub extern "C" fn set_decimal_mantissa(storage: *mut ValueStorage, mantissa: i64) {
    unsafe {
        (*storage).of_decimal.mantissa_ = mantissa;
    }
}

#[no_mangle]
pub extern "C" fn set_decimal_exponent(storage: *mut ValueStorage, exponent: i16) {
    unsafe {
        (*storage).of_decimal.exponent_ = exponent;
    }
}

#[no_mangle]
pub extern "C" fn get_decimal_mantissa(storage: *const ValueStorage) -> i64 {
    unsafe {
        (*storage).of_decimal.mantissa_
    }
}

#[no_mangle]
pub extern "C" fn get_decimal_exponent(storage: *const ValueStorage) -> i16 {
    unsafe {
        (*storage).of_decimal.exponent_
    }
}

#[cfg(target_pointer_width = "32")]
#[no_mangle]
pub extern "C" fn set_uint64_set_value(storage: *mut ValueStorage, value: u32) {
    unsafe {
        (*storage).set(value);
    }
}

#[no_mangle]
pub extern "C" fn set_array_defined_bit(storage: *mut ValueStorage, defined: bool) {
    unsafe {
        (*storage).of_array.set_defined(defined)
    }
}

#[no_mangle]
pub extern "C" fn set_string_value(storage: *mut ValueStorage, value: *const c_char) {
    unsafe {
        (*storage).of_array.set_defined(true);
        (*storage).of_array.len_ = (std::ffi::CStr::from_ptr(value).to_bytes().len() + 1) as u32;
        (*storage).of_array.content_ = value as *mut std::ffi::c_void;
        (*storage).of_array.set_capacity_in_bytes(0);
    }
}

#[no_mangle]
pub extern "C" fn set_string_value_with_size(storage: *mut ValueStorage, value: *const c_char, size: usize) {
    unsafe {
        (*storage).of_array.set_defined(true);
        (*storage).of_array.len_ = (size + 1) as u32;
        (*storage).of_array.content_ = value as *mut std::ffi::c_void;
        (*storage).of_array.set_capacity_in_bytes(0);
    }
}

#[no_mangle]
pub extern "C" fn get_array_is_empty(storage: *mut ValueStorage) -> bool {
    unsafe {
        (*storage).of_array.len_ == 0
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
