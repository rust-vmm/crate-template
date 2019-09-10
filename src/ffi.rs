// Copyright (C) 2019 Alibaba Cloud. All rights reserved.
// Copyright 2019 Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use std::mem;
use std::ptr;

#[doc(hidden)]
pub use serde_bytes::ByteBuf;

/// Trait to get size information about an FFI object.
pub trait SizeofFamStruct {
    /// Get actual size of an FFI object.
    fn size_of(&self) -> usize;
}

/// Serialize an FFI object into `ByteBuf`.
pub fn serialize_ffi<T: SizeofFamStruct>(something: &T) -> ByteBuf {
    let mut serialized_self: Vec<u8> = vec![0; something.size_of()];
    unsafe {
        ptr::copy(
            something as *const T as *const u8,
            serialized_self.as_mut_ptr() as *mut u8,
            something.size_of(),
        );
    }
    ByteBuf::from(serialized_self)
}

/// Deserialize an FFI object from `ByteBuf`.
pub fn deserialize_ffi<T>(serialized: ByteBuf) -> std::result::Result<T, (usize, usize)>
where
    T: SizeofFamStruct + Default + Clone,
{
    let data = serialized.into_vec();
    if data.len() != mem::size_of::<T>() {
        Err((mem::size_of::<T>(), data.len()))
    } else {
        Ok(unsafe { ptr::read_unaligned(data.as_ptr() as *const T) })
    }
}

/// Deserialize an FFI object with flexible array as the last field from `ByteBuf`.
pub fn deserialize_ffi_fam<T>(serialized: ByteBuf) -> std::result::Result<Vec<T>, (usize, usize)>
where
    T: SizeofFamStruct + Default,
{
    let data = serialized.into_vec();
    if data.len() < mem::size_of::<T>() {
        Err((mem::size_of::<T>(), data.len()))
    } else {
        let obj = unsafe { ptr::read_unaligned(data.as_ptr() as *const T) };
        if obj.size_of() == mem::size_of::<T>() {
            if data.len() != mem::size_of::<T>() {
                Err((mem::size_of::<T>(), data.len()))
            } else {
                Ok(vec![obj])
            }
        } else {
            if obj.size_of() != data.len() {
                Err((obj.size_of(), data.len()))
            } else {
                let entries = (obj.size_of() + mem::size_of::<T>() - 1) / mem::size_of::<T>();
                let mut buf = Vec::with_capacity(entries);
                for _ in 0..entries {
                    buf.push(Default::default());
                }
                //let mut buf: Vec<T> = vec![Default::default(); entries];
                unsafe {
                    ptr::copy(
                        data.as_ptr(),
                        &mut buf[0] as *mut T as *mut u8,
                        obj.size_of(),
                    )
                };
                Ok(buf)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::*;

    #[allow(dead_code)]
    #[test]
    fn ffi_test_serde() {
        #[derive(Default, Clone, SerializeFfi, DeserializeFfi)]
        pub struct FfiStruct1 {
            f1: u32,
        }

        #[derive(Default, Clone, SerializeFfi, DeserializeFfiFam)]
        pub struct FfiStruct2 {
            f1: u32,
        }

        impl SizeofFamStruct for FfiStruct2 {
            fn size_of(&self) -> usize {
                mem::size_of::<Self>()
            }
        }
    }

    #[test]
    fn ffi_test_kvm_structs() {
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone, PartialEq, SerializeFfi, DeserializeFfi)]
        pub struct kvm_memory_alias {
            pub slot: u32,
            pub flags: u32,
            pub guest_phys_addr: u64,
            pub memory_size: u64,
            pub target_phys_addr: u64,
        }

        #[repr(C)]
        #[derive(Default, Debug, SerializeFfi, DeserializeFfi)]
        pub struct __IncompleteArrayField<T>(::std::marker::PhantomData<T>, [T; 0]);
        impl<T> __IncompleteArrayField<T> {
            #[inline]
            pub fn new() -> Self {
                __IncompleteArrayField(::std::marker::PhantomData, [])
            }
        }

        #[repr(C)]
        #[derive(Debug, Default, SerializeFfi, DeserializeFfiFam)]
        pub struct kvm_msrs {
            pub nmsrs: u32,
            pub pad: u32,
            pub entries: __IncompleteArrayField<u64>,
        }

        impl SizeofFamStruct for kvm_msrs {
            fn size_of(&self) -> usize {
                self.nmsrs as usize * std::mem::size_of::<u64>() + std::mem::size_of::<Self>()
            }
        }
    }
}
