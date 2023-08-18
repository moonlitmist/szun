pub mod util;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Reference {
    pub(crate) class:usize,
    pub(crate) address:usize,
}
impl Reference {
    pub fn null() -> Self
    {
        Self {
            class:0,
            address:0,
        }
    }

    pub fn is_null(&self) -> bool
    {
        self.address != 0
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Str {
    pub bytes:*mut u8,
    pub length:usize,
}

extern "C" {
    pub fn test();

    pub fn acquire(type_id:usize) -> Reference;
    pub fn release(addr:Reference);

    pub fn copy(dst:Reference, src:Reference) -> bool;
    pub fn transfer(dst:Reference, src:Reference) -> bool;

    pub fn type_outer(type_id:usize, key:usize) -> usize;
    pub fn type_inner(type_id:usize) -> usize;
    pub fn type_key(type_id:usize) -> usize;
    pub fn type_innerkey(type_id:usize) -> usize;
    pub fn kind_hasinner(kind:usize) -> usize;
    pub fn type_hasinner(type_id:usize) -> usize;

    pub fn type_size(type_id:usize) -> usize;

    pub fn name_indexof(data:*const u8, length:usize) -> usize;
    pub fn name_keyof(index:usize) -> Str;
    pub fn name_release(data:Str);

    pub fn varying_get(addr:Reference) -> Reference;
    pub fn varying_set(addr:Reference, source:Reference);
    pub fn varying_clear(addr:Reference);

    pub fn bool_set(addr:Reference, data:bool);
    pub fn bool_get(addr:Reference) -> bool;
    
    pub fn natural_set(addr:Reference, data:u64);
    pub fn natural_get(addr:Reference) -> u64;

    pub fn integer_set(addr:Reference, data:i64);
    pub fn integer_get(addr:Reference) -> i64;

    pub fn significant_set(addr:Reference, data:f64);
    pub fn significant_get(addr:Reference) -> f64;
    
    pub fn block_length(addr:Reference) -> usize;
    pub fn block_set(addr:Reference, index:usize, data:u8);
    pub fn block_get(addr:Reference, index:usize) -> u8;

    pub fn sequence_capacity(addr:Reference) -> usize;
    pub fn sequence_length(addr:Reference) -> usize;
    pub fn sequence_get(addr:Reference, index:usize) -> u8;
    pub fn sequence_clear(addr:Reference);
    pub fn sequence_set(addr:Reference, index:usize, value:u8);
    pub fn sequence_insert(addr:Reference, index:usize, value:u8);
    pub fn sequence_reserve(addr:Reference, capacity:usize);
    
    pub fn array_length(addr:Reference) -> usize;
    pub fn array_at(addr:Reference, index:usize) -> Reference;
    pub fn array_update(addr:Reference, index:usize, source:Reference);

    pub fn list_capacity(addr:Reference) -> usize;
    pub fn list_length(addr:Reference) -> usize;
    //pub fn list_first(addr:Reference) -> Reference;
    //pub fn list_last(addr:Reference) -> Reference;
    pub fn list_at(addr:Reference, index:usize) -> Reference;
    pub fn list_clear(addr:Reference);
    pub fn list_insert(addr:Reference, index:usize, src:Reference);
    pub fn list_prepend(addr:Reference, src:Reference);
    pub fn list_append(addr:Reference, src:Reference);
    pub fn list_update(addr:Reference, index:usize, src:Reference);
    //pub fn list_truncate(addr:Reference, maximum:usize);
    //pub fn list_shift(addr:Reference, maximum:usize);
    pub fn list_remove(addr:Reference, index:usize);
    pub fn list_reserve(addr:Reference, capacity:usize);
    //pub fn list_resize(addr:Reference, length:usize);
    //pub fn list_cut(addr:Reference, index:usize, length:usize) -> Reference;
    //pub fn list_find(addr:Reference, target:Reference, start:usize) -> usize;
    //pub fn list_count(addr:Reference, target:Reference, start:usize) -> usize;

    pub fn record_length(addr:Reference) -> usize;
    pub fn record_type(addr:Reference, index:usize) -> usize;
    pub fn record_at(addr:Reference, index:usize) -> Reference;
    pub fn record_update(addr:Reference, index:usize, source:Reference);
    pub fn record_keyof(addr:Reference, index:usize) -> usize;
    pub fn record_indexof(addr:Reference, key:usize) -> usize;

    pub fn schema_length(addr:Reference) -> usize;
    pub fn schema_insert(addr:Reference, index:usize, type_id:usize) -> usize;
    pub fn schema_update(addr:Reference, index:usize, type_id:usize);
    pub fn schema_get(addr:Reference, index:usize) -> usize;
    pub fn schema_remove(addr:Reference, index:usize);
    //pub fn schema_reorder(addr:Reference, from:usize, to:usize);
    pub fn schema_map(addr:Reference, key:usize, index:usize);
    pub fn schema_unmap(addr:Reference, key:usize);
    pub fn schema_indexof(addr:Reference, key:usize) -> usize;
    pub fn schema_keyof(addr:Reference, index:usize) -> usize;
    pub fn schema_bind(addr:Reference, id:usize) -> usize;
    pub fn schema_has(id:usize) -> bool;
}
