use crate::runtime::{
    Reference,
    acquire, release,
    type_key,
    sequence_capacity, sequence_length,
    sequence_clear, sequence_reserve,
    sequence_get,
    sequence_insert, sequence_set,
};
use crate::tag;
use super::sequence;

pub struct Sequence {
    managed:bool,
    addr:Reference,
}
impl Sequence {
    pub fn new() -> Self
    {
        Self {
            managed:true,
            addr:unsafe {acquire(sequence())},
        }
    }

    pub fn from(addr:Reference) -> Result<Self,()>
    {
        return if(unsafe {type_key(addr.class)} == tag::SEQUENCE) {
            Ok(Self { managed:false, addr:addr })
        }
        else {
            Err(())
        }
    }

    pub fn with_raw(data:Vec<u8>) -> Self
    {
        let mut obj = Self::new();
        obj.set_raw(data);
        return obj;
    }

    pub fn with(data:&str) -> Self
    {
        let mut obj = Self::new();
        obj.set(data);
        return obj;
    }

    pub fn detach(&mut self)
    {
        self.managed = false;
    }

    pub fn release(mut self)
    {
        self.detach();
        unsafe { release(self.addr); }
    }

    pub fn capacity(&self) -> usize
    {
        unsafe {sequence_capacity(self.addr)}
    }

    pub fn size(&self) -> usize
    {
        unsafe {sequence_length(self.addr)}
    }

    pub fn set_at(&mut self, index:usize, value:u8)
    {
        unsafe { sequence_set(self.addr, index, value); }
    }

    pub fn set_raw(&mut self, data:Vec<u8>)
    {
        unsafe { sequence_clear(self.addr); }
        if data.len() > 0 {
            unsafe { sequence_reserve(self.addr, data.len()); }
            for i in 0..data.len() {
                unsafe { sequence_insert(self.addr, i, data[i]); }
            }
        }
    }

    pub fn set(&mut self, data:&str)
    {
        self.set_raw(Vec::from(data.as_bytes()));
    }

    pub fn get_at(&self, index:usize) -> u8
    {
        unsafe {sequence_get(self.addr, index)}
    }

    pub fn get_raw(&self) -> Vec<u8>
    {
        let length = unsafe {sequence_length(self.addr)};
        let mut result = Vec::<u8>::new();

        if length > 0 {
            result.reserve_exact(length);
            for i in 0..length {
                result.push(unsafe {sequence_get(self.addr, i)});
            }
        }
        return result;
    }

    pub fn get(&self) -> String
    {
        match String::from_utf8(self.get_raw()) {
            Ok(s) => s,
            Err(_) => String::new(),
        }
    }
}
impl std::ops::Deref for Sequence {
    type Target = Reference;
    fn deref(&self) -> &Self::Target { return &self.addr; }
}
impl Drop for Sequence {
    fn drop(&mut self) { if self.managed { unsafe {release(self.addr)}; } }
}
