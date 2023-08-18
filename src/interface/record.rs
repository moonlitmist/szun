use crate::runtime::{
    util::{name_indexof, name_keyof},
    Reference,
    acquire, release,
    schema_has,
    type_key,
    record_length,
    record_at, record_update,
    record_indexof, record_keyof,
    record_type,
};
use crate::tag;
use super::record;

pub struct Record {
    managed:bool,
    addr:Reference,
}
impl Record {
    pub fn new(schema:usize) -> Result<Self,()>
    {
        if unsafe {schema_has(schema)} {
            Ok(Self {
                managed:true,
                addr:unsafe {acquire(record(schema))},
            })
        } else {
            Err(())
        }
    }

    pub fn from(addr:Reference) -> Result<Self,()>
    {
        return if(unsafe {type_key(addr.class)} == tag::RECORD) {
            Ok(Self { managed:false, addr:addr })
        } else {
            Err(())
        }
    }
    
    pub fn with(schema:usize, data:Vec<(&str, Reference)>) -> Result<Self,()>
    {
        match Self::new(schema) {
            Ok(mut obj) => {
                for (key, value) in data {
                    obj.set(key, value);
                }
                Ok(obj)
            }
            Err(_) => Err(())
        }
    }

    pub fn with_values(schema:usize, data:Vec<Reference>) -> Result<Self,()>
    {
        match Self::new(schema) {
            Ok(mut obj) => {
                for index in 0..data.len() {
                    obj.set_at(index, data[index]);
                }
                Ok(obj)
            }
            Err(_) => Err(())
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

    pub fn length(&self) -> usize
    {
        unsafe {record_length(self.addr)}
    }

    pub fn set_at(&mut self, index:usize, source:Reference)
    {
        unsafe { record_update(self.addr, index, source); }
    }

    pub fn set(&mut self, key:&str, source:Reference)
    {
        match self.indexof(key) {
            Some(index) => {
                unsafe { record_update(self.addr, index, source); }
            }
            None => { }
        }
    }

    pub fn at(&self, index:usize) -> Reference
    {
        unsafe {record_at(self.addr, index)}
    }

    pub fn get(&self, key:&str) -> Reference
    {
        match self.indexof(key) {
            Some(index) => {
                unsafe {record_at(self.addr, index)}
            }
            None => Reference::null()
        }
    }

    pub fn indexof(&self, key:&str) -> Option<usize>
    {
        let key_index = name_indexof(key);
        let result = unsafe {record_indexof(self.addr, key_index)};
        if result != self.length() {
            Some(result)
        }
        else {
            None
        }
    }

    pub fn keyof(&self, index:usize) -> Option<String>
    {
        let result = unsafe {record_keyof(self.addr, index)};
        if result != 0 {
            Some(name_keyof(result))
        }
        else {
            None
        }
    }

    pub fn kindof(&self, index:usize) -> usize
    {
        unsafe {record_type(self.addr, index)}
    }
}
impl std::ops::Deref for Record {
    type Target = Reference;
    fn deref(&self) -> &Self::Target { return &self.addr; }
}
impl Drop for Record {
    fn drop(&mut self) { if self.managed { unsafe {release(self.addr)}; } }
}
