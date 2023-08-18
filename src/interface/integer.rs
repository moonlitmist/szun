use crate::runtime::{
    Reference,
    acquire, release,
    type_key,
    integer_get, integer_set
};
use crate::tag;
use super::integer;

pub struct Integer {
    managed:bool,
    addr:Reference,
}
impl Integer {
    pub fn new() -> Self
    {
        Self {
            managed:true,
            addr:unsafe {acquire(integer())},
        }
    }

    pub fn from(addr:Reference) -> Result<Self,()>
    {
        return if(unsafe {type_key(addr.class)} == tag::INTEGER) {
            Ok(Self { managed:false, addr:addr })
        }
        else {
            Err(())
        }
    }

    pub fn with(value:i64) -> Self
    {
        let mut obj = Self::new();
        obj.set(value);
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

    pub fn set(&mut self, value:i64)
    {
        unsafe { integer_set(self.addr, value) };
    }

    pub fn get(&self) -> i64
    {
        unsafe { integer_get(self.addr) }
    }
}
impl std::ops::Deref for Integer {
    type Target = Reference;
    fn deref(&self) -> &Self::Target { return &self.addr; }
}
impl Drop for Integer {
    fn drop(&mut self) { if self.managed { unsafe {release(self.addr)}; } }
}
