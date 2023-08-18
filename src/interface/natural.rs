use crate::runtime::{
    Reference,
    acquire, release,
    type_key,
    natural_get, natural_set
};
use crate::tag;
use super::natural;

pub struct Natural {
    managed:bool,
    addr:Reference,
}
impl Natural {
    pub fn new() -> Self
    {
        Self {
            managed:true,
            addr:unsafe {acquire(natural())},
        }
    }

    pub fn from(addr:Reference) -> Result<Self,()>
    {
        return if(unsafe {type_key(addr.class)} == tag::NATURAL) {
            Ok(Self { managed:false, addr:addr })
        }
        else {
            Err(())
        }
    }

    pub fn with(value:u64) -> Self
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

    pub fn set(&mut self, value:u64)
    {
        unsafe { natural_set(self.addr, value) };
    }

    pub fn get(&self) -> u64
    {
        unsafe { natural_get(self.addr) }
    }
}
impl std::ops::Deref for Natural {
    type Target = Reference;
    fn deref(&self) -> &Self::Target { return &self.addr; }
}
impl Drop for Natural {
    fn drop(&mut self) { if self.managed { unsafe {release(self.addr)}; } }
}
