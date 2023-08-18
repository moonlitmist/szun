use crate::runtime::{
    util::{name_indexof, name_keyof},
    Reference,
    acquire, release,
    type_key,
    schema_length,
    schema_insert, schema_update, schema_get,
    schema_remove,
    schema_map, schema_unmap, schema_indexof, schema_keyof,
    schema_bind,
};
use crate::tag;
use super::schema;

pub struct Schema {
    managed:bool,
    addr:Reference,
}
impl Schema {
    pub fn new() -> Self
    {
        Self {
            managed:true,
            addr:unsafe {acquire(schema())},
        }
    }

    pub fn from(addr:Reference) -> Result<Self,()>
    {
        return if(unsafe {type_key(addr.class)} == tag::SCHEMA) {
            Ok(Self { managed:false, addr:addr })
        }
        else {
            Err(())
        }
    }

    pub fn with(members:Vec<(&str, usize)>) -> Self
    {
        let mut obj = Self::new();
        for binding in members {
            let (key, type_id) = binding;
            obj.assign(key, type_id);
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
    
    pub fn length(&self) -> usize
    {
        unsafe {schema_length(self.addr)}
    }

    pub fn add(&mut self, type_id:usize) -> usize
    {
        unsafe {schema_insert(self.addr, usize::MAX, type_id)}
    }

    pub fn assign(&mut self, key:&str, type_id:usize)
    {
        if key.len() > 0 {
            let key_index = name_indexof(key);
            unsafe {
                schema_map(
                    self.addr,
                    key_index,
                    schema_insert(self.addr, usize::MAX, type_id)
                );
            }
        }
        else {
            self.add(type_id);
        }
    }

    pub fn set(&mut self, index:usize, type_id:usize)
    {
        unsafe { schema_update(self.addr, index, type_id); }
    }

    pub fn get(&self, index:usize) -> Option<usize>
    {
        match unsafe {schema_get(self.addr, index)} {
            0 => None,
            value => Some(value),
        }
    }

    pub fn remove(&mut self, index:usize)
    {
        unsafe { schema_remove(self.addr, index); }
    }

    pub fn map(&mut self, key:&str, index:usize)
    {
        let key_index = name_indexof(key);
        unsafe { schema_map(self.addr, key_index, index); }
    }

    pub fn unmap(&mut self, key:&str)
    {
        let key_index = name_indexof(key);
        unsafe { schema_unmap(self.addr, key_index); }
    }

    pub fn indexof(&mut self, key:&str) -> Option<usize>
    {
        let key_index = name_indexof(key);
        let result = unsafe {schema_indexof(self.addr, key_index)};
        if result != self.length() {
            Some(result)
        }
        else {
            None
        }
    }

    pub fn keyof(&mut self, index:usize) -> Option<String>
    {
        let result = unsafe {schema_keyof(self.addr, index)};
        if result != 0 {
            Some(name_keyof(result))
        }
        else {
            None
        }
    }

    pub fn bind(&self, id:usize) -> usize
    {
        unsafe {schema_bind(self.addr, id)}
    }
}
impl std::ops::Deref for Schema {
    type Target = Reference;
    fn deref(&self) -> &Self::Target { return &self.addr; }
}
impl Drop for Schema {
    fn drop(&mut self) { if self.managed { unsafe {release(self.addr)}; } }
}
