use crate::runtime::{
    Reference,
    acquire, release,
    type_inner, type_key,
    block_length,
    block_set, block_get,
};
use crate::tag;
use super::block;

pub struct Block {
    managed:bool,
    addr:Reference,
}
impl Block {
    pub fn new(size:usize) -> Self
    {
        Self {
            managed:true,
            addr:unsafe {acquire(block(size))},
        }
    }

    pub fn from(addr:Reference) -> Result<Self,()>
    {
        return if(unsafe {type_key(addr.class)} == tag::BLOCK) {
            Ok(Self { managed:false, addr:addr })
        }
        else {
            Err(())
        }
    }

    pub fn with(size:usize, data:Vec<u8>) -> Self
    {
        let mut obj = Self::new(size);
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

    pub fn size(&self) -> usize
    {
        unsafe {block_length(self.addr)}
    }

    pub fn set(&mut self, data:Vec<u8>)
    {
        let length = unsafe {type_key(type_inner(self.addr.class))};
        for index in 0..usize::min(data.len(), length) {
            unsafe {block_set(self.addr, index, data[index])};
        }
    }

    pub fn get(&self) -> Vec<u8>
    {
        let mut result = Vec::<u8>::new();
        let length = unsafe {type_key(type_inner(self.addr.class))};
        if length > 0 {
            result.resize(length, 0);
            for index in 0..length {
                result[index] = unsafe {block_get(self.addr, index)};
            }
        }
        return result;
    }
}
impl std::ops::Deref for Block {
    type Target = Reference;
    fn deref(&self) -> &Self::Target { return &self.addr; }
}
impl Drop for Block {
    fn drop(&mut self) { if self.managed { unsafe {release(self.addr)}; } }
}
