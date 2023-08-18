use crate::runtime::{
    Reference,
    acquire, release,
    type_key,
    significant_get, significant_set
};
use crate::tag;
use super::significant;

pub struct Significant {
    managed:bool,
    addr:Reference,
}
impl Significant {
    pub fn new() -> Self
    {
        Self {
            managed:true,
            addr:unsafe {acquire(significant())},
        }
    }

    pub fn from(addr:Reference) -> Result<Self,()>
    {
        return if(unsafe {type_key(addr.class)} == tag::SIGNIFICANT) {
            Ok(Self { managed:false, addr:addr })
        }
        else {
            Err(())
        }
    }

    pub fn with(value:f64) -> Self
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

    pub fn set(&mut self, value:f64)
    {
        unsafe { significant_set(self.addr, value) };
    }

    pub fn get(&self) -> f64
    {
        unsafe { significant_get(self.addr) }
    }
}
impl std::ops::Deref for Significant {
    type Target = Reference;
    fn deref(&self) -> &Self::Target { return &self.addr; }
}
impl Drop for Significant {
    fn drop(&mut self) { if self.managed { unsafe {release(self.addr)}; } }
}
