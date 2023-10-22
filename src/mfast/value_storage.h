// Copyright (c) 2016, Huang-Ming Huang,  Object Computing, Inc.
// All rights reserved.
//
// This file is part of mFAST.
// See the file license.txt for licensing information.
#pragma once

#include "mfast_export.h"
#include <stdint.h>
#include <cstring>
#include <array>
#include <istream>

struct r_value_storage {
    uint64_t reserved1_ = {};
    uint64_t reserved2_ = {};
};

extern "C" {
void set_uint64_defined_bit(r_value_storage* storage, bool defined);
void set_uint64_present(r_value_storage* storage, bool defined);
#if SIZEOF_VOID_P == 4
void set_uint64_set_value(r_value_storage* storage, uint32_t value);
#else
void set_uint64_set_value(r_value_storage* storage, uint64_t value);
#endif
void set_decimal_defined_bit(r_value_storage* storage, bool defined);
void set_decimal_present(r_value_storage* storage, bool defined);
void set_decimal_mantissa_and_exponent(r_value_storage* storage, int64_t mantissa, int16_t exponent);
void set_decimal_mantissa(r_value_storage* storage, int64_t mantissa);
void set_decimal_exponent(r_value_storage* storage, int16_t exponent);
int64_t get_decimal_mantissa(const r_value_storage* storage);
int16_t get_decimal_exponent(const r_value_storage* storage);
}

namespace mfast {
class template_instruction;

template <bool B, typename T = void>
using enable_if_t = typename std::enable_if<B, T>::type;

union MFAST_EXPORT value_storage {
  struct {
    uint32_t present_; ///< indicate if the value is present,
    uint32_t padding_ : 31;
    uint32_t defined_bit_ : 1;
    uint64_t content_;
  } of_uint64;

#if SIZEOF_VOID_P == 4
  struct {
    uint32_t present_; ///< indicate if the value is present,
    uint32_t padding_ : 31;
    uint32_t defined_bit_ : 1;
    uint32_t content_;
  } of_uint32;
#endif

  struct {
    uint32_t present_; ///< indicate if the value is present,
    int16_t exponent_;
    uint16_t padding2_ : 15;
    uint16_t defined_bit_ : 1;
    int64_t mantissa_;
  } of_decimal;

  struct {
    uint32_t present_; ///< indicate if the value is present,
    uint32_t
        own_content_ : 1;  ///< indicate if \a content_ should be deallocated
    uint32_t is_link_ : 1; ///< indicate wheter this is a link so that we
                           ///shouldn't destruct
    /// subfields.
    uint32_t padding_ : 29;
    uint32_t defined_bit_ : 1;
    value_storage *content_;
  } of_group; ///< used for group or template

  struct {
    uint32_t len_; ///< the length+1 of content; it represents null value or
                   ///content is absent
    /// when len==0.
    /// In the case of empty string, len == 1 and content_[0]= '\0'.
    uint32_t capacity_in_bytes_ : 31; ///< used to track the length of memory
                                      ///that has been reserved
    ///< for \a content_. if <tt>.capacity_in_bytes_ == 0</tt> and <tt>len_ >
    ///0</tt>,
    ///< it means the object does not own the memory in \a content_.
    uint32_t defined_bit_ : 1; ///< used by FAST coder/encoder/decoder for
                               ///tracking if a dictionary
                               ///< value is defined or not.
    void *content_;
  } of_array;

  struct {
    union {
      const template_instruction *instruction_;
      uint64_t dummy_; // make sure content_ and instruction_ won't be packed
                       // together in 32
                       // bits environment
    } of_instruction;

    value_storage *content_;
  } of_templateref;

  // construct an undefined value
  value_storage() {
    of_uint64.content_ = 0;
    of_templateref.of_instruction.dummy_ = 0;
  };

  // construct a default numeric value
  value_storage(int) {
    of_uint64.content_ = 0;
    of_uint64.padding_ = 0;
    of_uint64.defined_bit_ = 1;
    of_uint64.present_ = 1;
  };

  // construct a default zero length string value
  value_storage(const char *) {
    of_array.content_ = const_cast<char *>("");
    of_array.len_ = 1;
    of_array.capacity_in_bytes_ = 0;
    of_array.defined_bit_ = 1;
  };

  bool is_defined() const { return of_array.defined_bit_; }
  void defined(bool v) { of_array.defined_bit_ = v; }
  bool is_empty() const { return of_array.len_ == 0; }
  void present(bool p) { of_array.len_ = p; }
  uint32_t array_length() const {
    return of_array.len_ == 0 ? 0 : of_array.len_ - 1;
  };
  void array_length(uint32_t n) { of_array.len_ = n + 1; }
#if SIZEOF_VOID_P == 4
  template <typename T>
  enable_if_t<!std::is_pointer<T>::value && sizeof(T) <= 4, T> get() const {
    return static_cast<T>(of_uint32.content_);
  }

  template <typename T>
  enable_if_t<!std::is_pointer<T>::value && sizeof(T) == 8, T> get() const {
    return static_cast<T>(of_uint64.content_);
  }
#else
  template <typename T> enable_if_t<!std::is_pointer<T>::value, T> get() const {
    return static_cast<T>(of_uint64.content_);
  }
#endif
  template <typename T> enable_if_t<std::is_pointer<T>::value, T> get() const {
    return reinterpret_cast<T>(of_array.content_);
  }

#if SIZEOF_VOID_P == 4
  template <typename T>
  enable_if_t<std::is_integral<T>::value && (sizeof(T) <= 4)> set(T v) {
    of_uint32.content_ = static_cast<uint32_t>(v);
  }

  template <typename T>
  enable_if_t<std::is_integral<T>::value && (sizeof(T) > 4)> set(T v) {
    of_uint64.content_ = static_cast<uint64_t>(v);
  }
#else
  template <typename T> enable_if_t<!std::is_pointer<T>::value> set(T v) {
    of_uint64.content_ = static_cast<uint64_t>(v);
  }
#endif

  template <typename T> enable_if_t<std::is_pointer<T>::value> set(void *v) {
    of_array.content_ = v;
  }

 value_storage(r_value_storage s) {
    memcpy(this, &s, sizeof(s));
 }
};

static_assert(sizeof(value_storage) == 16, "");

static_assert(sizeof(r_value_storage) == sizeof(value_storage), "");

template <typename IntType> struct int_value_storage {
  r_value_storage storage_;

  int_value_storage() {
      set_uint64_defined_bit(&storage_, true);
  }
  int_value_storage(value_storage s) {
      memcpy(&storage_, &s, sizeof(s));
  }
  int_value_storage(IntType v) {
    set_uint64_defined_bit(&storage_, true);
    set_uint64_present(&storage_, true);
    set_uint64_set_value(&storage_, v);
  }
};

struct decimal_value_storage {
  r_value_storage storage_;

  decimal_value_storage() {
      set_decimal_defined_bit(&storage_, true);
  }
  explicit decimal_value_storage(value_storage s) {
      memcpy(&storage_, &s, sizeof(s));
  }
  decimal_value_storage(int64_t mantissa, int16_t exponent) {
    set_decimal_defined_bit(&storage_, true);
    set_decimal_present(&storage_, true);
    set_decimal_mantissa_and_exponent(&storage_, mantissa, exponent);
  }

  int64_t mantissa() const {
      return get_decimal_mantissa(&storage_);
  }
  int16_t exponent() const {
      return get_decimal_exponent(&storage_);
  }
  void mantissa(int64_t m) {
      set_decimal_mantissa(&storage_, m);
  }
  void exponent(int16_t e) {
      set_decimal_exponent(&storage_, e);
  }
};

struct string_value_storage {
  value_storage storage_;

  string_value_storage() { storage_.of_array.defined_bit_ = 1; }
  explicit string_value_storage(value_storage s) : storage_(s) {}
  string_value_storage(const char *v) {
    storage_.of_array.defined_bit_ = 1;
    storage_.of_array.len_ = static_cast<uint32_t>(std::strlen(v) + 1);
    storage_.of_array.content_ = const_cast<char *>(v);
    storage_.of_array.capacity_in_bytes_ = 0;
  }

  string_value_storage(const char *v, std::size_t n) {
    storage_.of_array.defined_bit_ = 1;
    storage_.of_array.len_ = static_cast<uint32_t>(n + 1);
    storage_.of_array.content_ = const_cast<char *>(v);
    storage_.of_array.capacity_in_bytes_ = 0;
  }
};

struct byte_vector_value_storage : string_value_storage {
  byte_vector_value_storage() {}
  explicit byte_vector_value_storage(const value_storage &s)
      : string_value_storage(s) {}
  byte_vector_value_storage(const unsigned char *v, std::size_t n)
      : string_value_storage(reinterpret_cast<const char *>(v), n) {}

  byte_vector_value_storage(const char *v, std::size_t n)
      : string_value_storage(reinterpret_cast<const char *>(v), n) {}
};

template <unsigned SIZE>
struct value_storage_array : std::array<value_storage, SIZE> {};

template <> struct value_storage_array<0> {
  value_storage *data() { return reinterpret_cast<value_storage *>(this); }
  const value_storage *data() const {
    return reinterpret_cast<const value_storage *>(this);
  }
};

MFAST_EXPORT std::istream &operator>>(std::istream &strm,
                                      decimal_value_storage &storage);
MFAST_EXPORT std::ostream &operator<<(std::ostream &os,
                                      const decimal_value_storage &storage);
}
