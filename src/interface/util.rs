use crate::runtime;

pub fn acquire(type_id:usize) -> runtime::Reference
{
    unsafe {runtime::acquire(type_id)}
}

pub fn release(addr:runtime::Reference)
{
    unsafe {runtime::release(addr)}
}

pub fn copy(dst:runtime::Reference, src:runtime::Reference) -> Result<(),()>
{
    if unsafe {runtime::copy(dst, src)} { Ok(()) }
    else { Err(()) }
}

pub fn transfer(dst:runtime::Reference, src:runtime::Reference) -> Result<(),()>
{
    if unsafe {runtime::transfer(dst, src)} { Ok(()) }
    else { Err(()) }
}
