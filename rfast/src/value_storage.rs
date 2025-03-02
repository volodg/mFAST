use std::ffi::c_uint;
use crate::instructions::template_instruction::TemplateInstruction;
use intbits::Bits;
use num::Integer;

#[repr(C)]
pub union ValueStorage {
    pub of_uint64: ValueStorageUint64,
    #[cfg(target_pointer_width = "32")]
    of_uint32: ValueStorageUint32,
    pub of_decimal: ValueStorageDecimal,
    of_group: ValueStorageGroup,
    pub of_array: ValueStorageArray,
    of_templateref: ValueStorageTemplateRef,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ValueStorageUint64 {
    pub present_: u32,
    pub defined_bit_: u32, // least significant bit
    content_: u64,
}

#[repr(C)]
#[cfg(target_pointer_width = "32")]
#[derive(Clone, Copy)]
pub struct ValueStorageUint32 {
    present_: u32,
    defined_bit_: u32, // least significant bit
    content_: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ValueStorageDecimal {
    pub present_: u32,
    pub exponent_: i16,
    pub defined_bit_: u16, // least significant bit
    pub mantissa_: i64,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ValueStorageGroup {
    ///< indicate if the value is present,
    present_: u32,
    /// own_content_ : 1;  ///< indicate if \a content_ should be deallocated
    /// is_link_ : 1; ///< indicate whether this is a link so that we shouldn't destruct
    own_content_and_is_link_and_defined_bit_: u32, // two most significant bits and one least significant bit
    content_: *mut ValueStorage,
} ///< used for group or template

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ValueStorageArray {
    ///< the length+1 of content; it represents null value or
    ///content is absent
    /// when len==0.
    /// In the case of empty string, len == 1 and content_[0]= '\0'.
    pub len_: u32,
    ///"capacity_in_bytes" < used to track the length of memory
    ///that has been reserved
    ///< for \a content_. if <tt>.capacity_in_bytes_ == 0</tt> and <tt>len_ >
    ///0</tt>,
    ///< it means the object does not own the memory in \a content_.

    ///"defined_bit" < used by FAST coder/encoder/decoder for
    ///tracking if a dictionary
    ///< value is defined or not.
    capacity_in_bytes_and_defined_bit_: u32, // defined_bit_ is least significant bit
    pub content_: *mut std::ffi::c_void,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ValueStorageTemplateRef {
    of_instruction: ValueStorageTemplateRefInstruction,
    content_: *mut ValueStorage,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union ValueStorageTemplateRefInstruction {
    instruction_: *const TemplateInstruction,
    dummy_: u64,
}

impl Default for ValueStorage {
    fn default() -> Self {
        ValueStorage {
            of_uint64: ValueStorageUint64 {
                present_: 0,
                defined_bit_: 0,
                content_: 0,
            },
        }
    }
}

impl ValueStorage {
    fn new_numeric(value: c_uint) -> Self {
        let mut storage = Self::default();
        unsafe {
            storage.of_uint64.content_ = value as u64;
            storage.of_uint64.defined_bit_ = 1;
            storage.of_uint64.present_ = 1;
        }
        storage
    }

    fn new_string(value: *const std::os::raw::c_char) -> Self {
        let mut storage = Self::default();
        unsafe {
            storage.of_array.content_ = value as *mut std::ffi::c_void;
            storage.of_array.len_ = 1;
            storage.of_array.capacity_in_bytes_and_defined_bit_ = 1;
        }
        storage
    }

    fn is_defined(&self) -> bool {
        unsafe {
            self.of_array.capacity_in_bytes_and_defined_bit_.bit(0)
        }
    }

    fn defined(&mut self, v: bool) {
        unsafe {
            self.of_array.capacity_in_bytes_and_defined_bit_.set_bit(0, v)
        }
    }

    fn is_empty(&self) -> bool {
        unsafe {
            self.of_array.len_ == 0
        }
    }

    fn present(&mut self, p: bool) {
        unsafe {
            self.of_array.len_ = if p { 1 } else { 0 };
        }
    }

    fn array_length(&self) -> u32 {
        unsafe {
            if self.of_array.len_ == 0 {
                0
            } else {
                self.of_array.len_ - 1
            }
        }
    }

    fn set_array_length(&mut self, n: u32) {
        unsafe {
            self.of_array.len_ = n + 1;
        }
    }
}

#[cfg(target_pointer_width = "32")]
impl ValueStorage {
    fn get<T>(&self) -> T
        where
            T: Copy,
    {
        if std::mem::size_of::<T>() <= 4 {
            self.of_uint32.content_ as T
        } else {
            self.of_uint64.content_ as T
        }
    }

    pub fn set<T>(&mut self, v: T)
        where
            T: Copy,
    {
        if std::mem::size_of::<T>() <= 4 {
            self.of_uint32.content_ = v as u32;
        } else {
            self.of_uint64.content_ = v as u64;
        }
    }
}

#[cfg(target_pointer_width = "64")]
impl ValueStorage {
    fn get<T>(&self) -> T
        where
            u64: Into<T>,
            T: Copy + Integer,
    {
        unsafe {
            self.of_uint64.content_.into()
        }
    }

    pub fn set<T>(&mut self, v: T)
        where
            u64: From<T>,
            T: Copy + Integer,
    {
        self.of_uint64.content_ = v.into();
    }
}

impl ValueStorageArray {
    pub fn set_defined(&mut self, defined: bool) {
        self.capacity_in_bytes_and_defined_bit_.set_bit(31, defined);
    }

    pub fn set_capacity_in_bytes(&mut self, capacity: u32) {
        self.capacity_in_bytes_and_defined_bit_.set_bits(0..=30, capacity);
    }
}
