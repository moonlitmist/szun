use crate::runtime::{
    Reference,
    acquire, release,
    type_key,
    varying_get, varying_set,
    varying_clear,
};
use crate::tag;
use super::varying;

pub struct Varying {
    managed:bool,
    addr:Reference,
}
impl Varying {
    pub fn new() -> Self
    {
        Self {
            managed:true,
            addr:unsafe {acquire(varying())},
        }
    }

    pub fn from(addr:Reference) -> Result<Self,()>
    {
        return if(unsafe {type_key(addr.class)} == tag::VARYING) {
            Ok(Self { managed:false, addr:addr })
        }
        else {
            Err(())
        }
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

    pub fn is_null(&self) -> bool
    {
        (unsafe {varying_get(self.addr)}).address == 0
    }

    pub fn set(&mut self, source:Reference)
    {
        unsafe { varying_set(self.addr, source); }
    }

    pub fn clear(&mut self)
    {
        unsafe { varying_clear(self.addr); }
    }

    pub fn get(&self) -> Reference
    {
        unsafe { varying_get(self.addr) }
    }
}
impl std::ops::Deref for Varying {
    type Target = Reference;
    fn deref(&self) -> &Self::Target { return &self.addr; }
}
impl Drop for Varying {
    fn drop(&mut self) { if self.managed { unsafe {release(self.addr)}; } }
}
