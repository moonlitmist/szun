//mod typetree; pub use typetree::TypeTree;
//mod memory; pub use memory::Memory;

fn pack_count_leading_ones(data:u8) -> usize
{
    (data == 0xff) as usize
    + (data >= 0xfe) as usize
    + (data >= 0xfc) as usize
    + (data >= 0xf8) as usize
    + (data >= 0xf0) as usize
    + (data >= 0xe0) as usize
    + (data >= 0xc0) as usize
    + (data >= 0x80) as usize
}

fn pack_encode_size(data:u64, signed:bool) -> usize
{
    let sign_bit = signed as u32;
    1 + (data >= 1 << (7 - sign_bit)) as usize
    + (data >= 1 << (14 - sign_bit)) as usize
    + (data >= 1 << (21 - sign_bit)) as usize
    + (data >= 1 << (28 - sign_bit)) as usize
    + (data >= 1 << (35 - sign_bit)) as usize
    + (data >= 1 << (42 - sign_bit)) as usize
    + (data >= 1 << (49 - sign_bit)) as usize
    + (data >= 1 << (56 - sign_bit)) as usize
    + (signed && data >= 1 << 63) as usize
}

fn pack_data(size:usize, data:u64, sign:bool) -> Vec<u8>
{
    let mut data = data;
    let mut buffer :Vec<u8> = vec![0];
    let mut header_size = size - 1;
    let mut buffer_size = size;

    if header_size >= 8 {
        buffer.append(&mut pack_natural(header_size as u64));
        buffer_size += buffer.len() - 1;
        header_size = 8;
    }
    buffer.resize(buffer_size, 0);
    buffer[0] = ((0xFF00 >> header_size) & 0xFF) as u8;

    for i in 1..buffer_size + 1 {
        buffer[buffer_size - i] |= (data & 0xFF) as u8;
        data >>= 8;
    }

    if sign {
        match header_size {
            8 => { buffer[buffer_size - (size - 1)] |= 0x80; }
            7 => { buffer[1] |= 0x80; }
            _ => { buffer[0] |= 1 << 6 - header_size; }
        }
    }

    return buffer;
}

pub fn pack_natural(data:u64) -> Vec<u8>
{
    pack_data(pack_encode_size(data, false), data, false)
}

pub fn pack_integer(data:i64) -> Vec<u8>
{
    let mut udata :u64 = data as u64;
    let negative :bool = data < 0;
    if negative { udata = !udata; }

    pack_data(pack_encode_size(udata, true), udata, negative)
}

fn unpack_data(data:&Vec<u8>, index:&mut usize, signed:bool) -> u64
{
    let mut result :u64 = 0;
    let mut negative = false;
    if *index < data.len() {
        let mut pack_size = pack_count_leading_ones(data[*index]);

        if pack_size < 7 {
            result = (((data[*index] as u64) << pack_size) & 0xFF) >> pack_size;
            if signed {
                let sign_mask = 1 << (6 - pack_size);
                negative = (result & sign_mask) != 0;
                result &= !sign_mask;
            }
            *index += 1;
        }
        else {
            *index += 1;
            if pack_size == 8 {
                pack_size = unpack_natural(&data, index) as usize;
            }
            pack_size -= 1;

            result = data[*index] as u64;
            if signed {
                negative = (result & 0x80) != 0;
                result &= 0x7F;
            }
            *index += 1;
        }

        for _ in 1..pack_size + 1 {
            result <<= 8;
            result += data[*index] as u64;
            *index += 1;
        }

        if negative { result = !result; }
    }
    return result;
}

pub fn unpack_natural(data:&Vec<u8>, index:&mut usize) -> u64
{
    unpack_data(data, index, false)
}

pub fn unpack_integer(data:&Vec<u8>, index:&mut usize) -> i64
{
    unpack_data(data, index, true) as i64
}
