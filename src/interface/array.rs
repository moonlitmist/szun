use crate::runtime::{
    Reference,
    type_key,
    acquire, release,
    array_length, array_at, array_update
};
use crate::tag;
use super::array;

/// Constant-sized, indexed, ordered collection.
pub struct Array {
    managed:bool,
    addr:Reference,
}
impl Array {
    /// Allocates a new array of a given size and type.
    /// 
    /// # Arguments
    /// 
    /// * `length` - number of elements in the array
    /// * `class` - type identifier of the array contents
    /// 
    /// # Examples
    /// 
    /// ```
    /// // Produces an array of 8 integers.
    /// let int_array = szun::Array::new(8, szun::integer());
    /// ```
    pub fn new(length:usize, class:usize) -> Self
    {
        Self {
            managed:true,
            addr:unsafe {acquire(array(length, class))},
        }
    }

    pub fn from(addr:Reference) -> Result<Self,()>
    {
        return if(unsafe {type_key(addr.class)} == tag::ARRAY) {
            Ok(Self { managed:false, addr:addr })
        }
        else {
            Err(())
        }
    }

    pub fn with(length:usize, class:usize, data:Vec<Reference>) -> Self
    {
        let mut obj = Self::new(length, class);
        for i in 0..usize::min(length, data.len()) {
            obj.set(i, data[i]);
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
        unsafe {array_length(self.addr)}
    }

    pub fn set(&mut self, index:usize, source:Reference)
    {
        unsafe { array_update(self.addr, index, source); }
    }

    pub fn at(&self, index:usize) -> Reference
    {
        unsafe {array_at(self.addr, index)}
    }
}
impl std::ops::Deref for Array {
    type Target = Reference;
    fn deref(&self) -> &Self::Target { return &self.addr; }
}
impl Drop for Array {
    fn drop(&mut self) { if self.managed { unsafe {release(self.addr)}; } }
}
