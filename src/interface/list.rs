use crate::runtime::{
    Reference,
    acquire, release,
    type_key,
    list_capacity, list_length,
    list_at,
    list_clear,
    list_insert, list_prepend, list_append, list_update,
    list_remove,
    list_reserve,
};
use crate::tag;
use super::list;

pub struct List {
    managed:bool,
    addr:Reference,
}
impl List {
    pub fn new(class:usize) -> Self
    {
        Self {
            managed:true,
            addr:unsafe {acquire(list(class))},
        }
    }

    pub fn from(addr:Reference) -> Result<Self,()>
    {
        return if(unsafe {type_key(addr.class)} == tag::LIST) {
            Ok(Self { managed:false, addr:addr })
        }
        else {
            Err(())
        }
    }

    pub fn with(class:usize, data:Vec<Reference>) -> Self
    {
        let mut obj = Self::new(class);
        for item in data {
            obj.insert(obj.length(), item);
        }
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
        unsafe {list_capacity(self.addr)}
    }

    pub fn length(&self) -> usize
    {
        unsafe {list_length(self.addr)}
    }

    pub fn at(&self, index:usize) -> Reference
    {
        unsafe {list_at(self.addr, index)}
    }

    pub fn clear(&mut self)
    {
        unsafe{list_clear(self.addr)};
    }

    pub fn insert(&mut self, index:usize, source:Reference)
    {
        unsafe{list_insert(self.addr, index, source)};
    }

    pub fn prepend(&mut self, source:Reference)
    {
        unsafe{list_prepend(self.addr, source)};
    }

    pub fn append(&mut self, source:Reference)
    {
        unsafe{list_append(self.addr, source)};
    }

    pub fn set(&mut self, index:usize, source:Reference)
    {
        unsafe{list_update(self.addr, index, source)};
    }

    pub fn remove(&mut self, index:usize)
    {
        unsafe{list_remove(self.addr, index)};
    }

    pub fn reserve(&mut self, length:usize)
    {
        unsafe{list_reserve(self.addr, length)};
    }
}
impl std::ops::Deref for List {
    type Target = Reference;
    fn deref(&self) -> &Self::Target { return &self.addr; }
}
impl Drop for List {
    fn drop(&mut self) { if self.managed { unsafe {release(self.addr)}; } }
}
