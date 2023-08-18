use crate::{
    runtime::{
        Reference,
        kind_hasinner, type_hasinner, type_inner, type_outer,
        type_key, type_innerkey,
    },
    tag,
    Type,
    util::*,
};
use crate::kind;
use crate::{
    Boolean,
    Natural, Integer,
    Block, Sequence,
    Array, List,
    Record,
};
use crate::util;

pub fn encode_tag(addr:Reference) -> Vec<u8>
{
    let mut type_id = addr.class;
    let mut result = Vec::<u8>::new();
    let mut remaining :usize = 1;
    while remaining > 0 {
        result.append(&mut util::pack_natural(unsafe {type_key(type_id)} as u64));

        remaining -= 1;
        if remaining == 0 {
            remaining += unsafe {type_hasinner(type_id)};
        }
        type_id = unsafe {type_inner(type_id)};
    }
    return result;
}

pub fn encode_data(addr:Reference) -> Vec<u8>
{
    let mut result = Vec::<u8>::new();
    match unsafe {type_key(addr.class)} {
        tag::BOOLEAN => {
            result.append(&mut util::pack_natural(Boolean::from(addr).unwrap().get() as u64));
        }
        tag::NATURAL => {
            result.append(&mut util::pack_natural(Natural::from(addr).unwrap().get() as u64));
        }
        tag::INTEGER => {
            result.append(&mut &mut util::pack_integer(Integer::from(addr).unwrap().get() as i64));
        }
        tag::BLOCK => {
            result.append(&mut Block::from(addr).unwrap().get());
        }
        tag::SEQUENCE => { 
            let data = Sequence::from(addr).unwrap();
            result.append(&mut util::pack_natural(data.size() as u64));
            result.append(&mut data.get_raw());
        }
        tag::ARRAY => {
            let data = Array::from(addr).unwrap();
            for i in 0..data.length() {
                if unsafe {type_innerkey(addr.class)} == tag::VARYING {
                    result.append(&mut encode(data.at(i)));
                }
                else {
                    result.append(&mut encode_data(data.at(i)));
                }
            }
        }
        tag::LIST => {
            let data = List::from(addr).unwrap();
            result.append(&mut util::pack_natural(data.length() as u64));
            for i in 0..data.length() {
                if unsafe {type_innerkey(addr.class)} == tag::VARYING {
                    result.append(&mut encode(data.at(i)));
                }
                else {
                    result.append(&mut encode_data(data.at(i)));
                }
            }
        }
        tag::RECORD => {
            let data = Record::from(addr).unwrap();
            for i in 0..data.length() {
                if kind(data.kindof(i)) == tag::VARYING {
                    result.append(&mut encode(data.at(i)));
                }
                else {
                    result.append(&mut encode_data(data.at(i)));
                }
            }
        }
        _ => { }
    }
    return result;
}

pub fn encode(addr:Reference) -> Vec<u8>
{
    let mut result = encode_tag(addr);
    result.append(&mut encode_data(addr));
    return result;
}

pub fn decode_tag(data:&Vec<u8>, index:&mut usize) -> Result<usize,()>
{
    let mut tags = Vec::<usize>::new();
    let mut remaining :usize = 1;
    while remaining > 0 && *index < data.len() {
        let kind = util::unpack_natural(&data, index) as usize;
        tags.push(kind);

        remaining -= 1;
        if remaining == 0 {
            remaining += unsafe {kind_hasinner(kind)};
        }
    }
    let mut type_id = 0;
    for i in (0..tags.len()).rev() {
        type_id = unsafe {type_outer(type_id, tags[i])};
    }

    return if remaining == 0 { Ok(type_id) }
    else { Err(()) }
}

pub fn decode_data(data:&Vec<u8>, type_id:usize, index:&mut usize) -> Result<Type,()>
{
    match unsafe {type_key(type_id)} {
        tag::VARYING => {
            return decode(data, index);
        }
        tag::BOOLEAN => {
            return Ok(Type::Boolean(Boolean::with(unpack_natural(data, index) == 1)));
        }
        tag::NATURAL => {
            return Ok(Type::Natural(Natural::with(unpack_natural(data, index))));
        }
        tag::INTEGER => {
            return Ok(Type::Integer(Integer::with(unpack_integer(data, index))));
        }
        tag::BLOCK => {
            let size = unsafe {type_innerkey(type_id)};
            let mut bytes = Vec::<u8>::with_capacity(size);
            if *index + size <= data.len() {
                for _ in 0..size {
                    bytes.push(data[*index]);
                    *index += 1;
                }
            }
            return Ok(Type::Block(Block::with(size, bytes)));
        }
        tag::SEQUENCE => {
            let size = unpack_natural(data, index) as usize;
            let mut bytes = Vec::<u8>::with_capacity(size);
            if *index + size <= data.len() {
                for _ in 0..size {
                    bytes.push(data[*index]);
                    *index += 1;
                }
            }
            return Ok(Type::Sequence(Sequence::with_raw(bytes)));
        }
        tag::RECORD => {
            return match Record::new(unsafe {type_innerkey(type_id)}) {
                Ok(mut value) => {
                    for i in 0..value.length() {
                        match decode_data(data, value.kindof(i), index) {
                            Ok(refer) => {
                                value.set_at(i, refer.get());
                            }
                            Err(_) => return Err(())
                        }
                    }
                    Ok(Type::Record(value))
                }
                Err(_) => Err(()),
            }
        }
        _ => { }
    }

    return Err(());
}

pub fn decode(data:&Vec<u8>, index:&mut usize) -> Result<Type,()>
{
    return match decode_tag(data, index) {
        Ok(type_id) => decode_data(data, type_id, index),
        _ => Err(()),
    }
}
