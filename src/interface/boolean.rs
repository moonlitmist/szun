use crate::runtime::{
    Reference,
    acquire, release,
    type_key,
    bool_get, bool_set
};
use crate::tag;
use super::boolean;

pub struct Boolean {
    managed:bool,
    addr:Reference,
}
impl Boolean {
    pub fn new() -> Self
    {
        Self {
            managed:true,
            addr:unsafe {acquire(boolean())},
        }
    }

    pub fn from(addr:Reference) -> Result<Self,()>
    {
        return if(unsafe {type_key(addr.class)} == tag::BOOLEAN) {
            Ok(Self { managed:false, addr:addr })
        }
        else {
            Err(())
        }
    }

    pub fn with(value:bool) -> Self
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

    pub fn set(&mut self, value:bool)
    {
        unsafe { bool_set(self.addr, value) };
    }

    pub fn get(&self) -> bool
    {
        unsafe { bool_get(self.addr) }
    }
}
impl std::ops::Deref for Boolean {
    type Target = Reference;
    fn deref(&self) -> &Self::Target { return &self.addr; }
}
impl Drop for Boolean {
    fn drop(&mut self) { if self.managed { unsafe {release(self.addr)}; } }
}
