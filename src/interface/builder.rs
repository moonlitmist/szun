use crate::tag;
use crate::runtime;

pub fn inner(type_id:usize) -> usize { unsafe {runtime::type_inner(type_id)} }
pub fn kind(type_id:usize) -> usize { unsafe {runtime::type_key(type_id)} }

pub fn varying() -> usize { 0 }
pub fn null() -> usize { unsafe { runtime::type_outer(0, tag::NULL) } }
pub fn boolean() -> usize { unsafe { runtime::type_outer(0, tag::BOOLEAN) } }
pub fn natural() -> usize { unsafe { runtime::type_outer(0, tag::NATURAL) } }
pub fn integer() -> usize { unsafe { runtime::type_outer(0, tag::INTEGER) } }
pub fn significant() -> usize { unsafe { runtime::type_outer(0, tag::SIGNIFICANT) } }

pub fn block(size:usize) -> usize {
    unsafe {
        let inner_node = runtime::type_outer(0, size);
        runtime::type_outer(inner_node, tag::BLOCK)
    }
}

pub fn sequence() -> usize { unsafe { runtime::type_outer(0, tag::SEQUENCE) } }

pub fn array(size:usize, type_id:usize) -> usize {
    unsafe {
        let inner_node = runtime::type_outer(type_id, size);
        runtime::type_outer(inner_node, tag::ARRAY)
    }
}

pub fn list(type_id:usize) -> usize { unsafe { runtime::type_outer(type_id, tag::LIST) } }

pub fn record(schema_id:usize) -> usize {
    unsafe {
        let inner_node = runtime::type_outer(0, schema_id);
        runtime::type_outer(inner_node, tag::RECORD)
    }
}

pub fn schema() -> usize { unsafe { runtime::type_outer(0, tag::SCHEMA) } }
