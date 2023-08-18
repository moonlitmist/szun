#include "lib.h"
#include "rawlist.cc"

TypeTree DB_TYPE;
NameTree DB_NAME;
SparseList<Type::SchemaBinding> DB_SCHEMA;
Pool<Reference> DB_REFERENCE;

extern "C" void test() { }

extern "C" size_t type_outer(size_t type_id, size_t key)
{
    return DB_TYPE.outer(type_id, key);
}

extern "C" size_t type_inner(size_t type_id)
{
    return DB_TYPE.inner(type_id);
}

extern "C" size_t type_key(size_t type_id)
{
    return DB_TYPE.key(type_id);
}

extern "C" size_t type_innerkey(size_t type_id)
{
    return DB_TYPE.key(DB_TYPE.inner(type_id));
}

extern "C" size_t type_size(size_t type_id)
{
    if(DB_TYPE.has(type_id)) {
        size_t type = DB_TYPE.key(type_id);

        switch(type) {
            case Type::Tag::Null:     return 0;
            case Type::Tag::Varying:  return sizeof(Reference);
            case Type::Tag::Boolean:  return sizeof(Type::Boolean);
            case Type::Tag::Natural:  return sizeof(Type::Natural);
            case Type::Tag::Integer:  return sizeof(Type::Integer);
            case Type::Tag::Significant:  return sizeof(Type::Integer);
            case Type::Tag::Block:    return type_innerkey(type_id);
            case Type::Tag::Sequence: return sizeof(Type::Sequence);

            case Type::Tag::Array: {
                size_t length = DB_TYPE.inner(type_id);
                size_t inner = DB_TYPE.inner(length);
                if(inner == Type::Tag::Null) {
                    return sizeof(Reference);
                }
                else {
                    return static_cast<size_t>(DB_TYPE.key(length)) * type_size(inner);
                }
            }

            case Type::Tag::List:
            case Type::Tag::Record:
            {
                size_t innerkey = type_innerkey(type_id);
                if(innerkey == Type::Tag::Null) {
                    return sizeof(Reference);
                }
                else {
                    switch(type) {
                        case Type::Tag::List: return sizeof(Type::List);
                        case Type::Tag::Record: {
                            auto binding = DB_SCHEMA.get(type_innerkey(type_id));
                            if(binding != nullptr) {
                                return binding->size;
                            }
                        }
                    }
                }
            }

            case Type::Tag::Schema: return sizeof(Type::Schema);
            default: return 0;
        }
    }
    return 0;
}

size_t type_alignment(size_t type_id)
{
    if(DB_TYPE.has(type_id)) {
        size_t type = DB_TYPE.key(type_id);

        switch(type) {
            case Type::Tag::Null:     return 0;
            case Type::Tag::Varying:  return sizeof(size_t);
            case Type::Tag::Boolean:  return sizeof(Type::Boolean);
            case Type::Tag::Natural:  return sizeof(Type::Natural);
            case Type::Tag::Integer:  return sizeof(Type::Integer);
            case Type::Tag::Significant:  return sizeof(Type::Significant);
            case Type::Tag::Block:    return sizeof(uint8_t);
            case Type::Tag::Sequence: return sizeof(size_t);
            case Type::Tag::Array:    return type_inner(type_id);
            case Type::Tag::List:     return sizeof(size_t);
            case Type::Tag::Record: {
                auto binding = DB_SCHEMA.get(type_innerkey(type_id));
                if(binding != nullptr) {
                    return binding->alignment;
                }
            }
            case Type::Tag::Schema: return sizeof(size_t);
            default: return 0;
        }
    }
    return 0;
}

extern "C" size_t kind_hasinner(size_t kind)
{
    switch(kind) {
        case Type::Tag::Array:
            return 2;
        case Type::Tag::Block:
        case Type::Tag::List:
        case Type::Tag::Record:
            return 1;
    }
    return 0;
}

extern "C" size_t type_hasinner(size_t type_id)
{
    return kind_hasinner(type_key(type_id));
}

extern "C" size_t name_indexof(const uint8_t* bytes, size_t length)
{
    std::string str(reinterpret_cast<const char*>(bytes), length);
    if(str.size() > 0) {
        return DB_NAME.indexof(str);
    }
    return 0;
}

extern "C" Str name_keyof(size_t index)
{
    Str result {0};
    std::string str = DB_NAME.keyof(index);
    if(str.length() > 0) {
        result.bytes = new uint8_t[str.length()];
    }
    for(size_t i = 0; i < str.length(); ++i) {
        result.bytes[i] = str[i];
    }
    result.length = str.length();
    return result;
}

std::string name_keyof_internal(size_t index)
{
    return DB_NAME.keyof(index);
}

extern "C" void name_release(Str data)
{
    if(data.bytes != nullptr) {
        delete[] data.bytes;
    }
}

uint8_t* allocate(size_t type_id, size_t count)
{
    uint8_t* mem = nullptr;
    size_t size = type_size(type_id) * count;
    if(size > 0) {
        mem = reinterpret_cast<uint8_t*>(malloc(size));
        if(mem != nullptr) {
            memset(mem, 0, size);
        }
        return mem;
    }
    return nullptr;
}

extern "C" Reference acquire(size_t type_id)
{
    Reference addr {0};
    addr.address = allocate(type_id, 1);
    if(addr.address != nullptr) {
        addr.type = type_id;
    }
    return addr;
 }

void drop(Reference addr)
{
    if(addr.address != nullptr) {
        switch(type_key(addr.type)) {
            case Type::Tag::Varying: {
                auto& var = *reinterpret_cast<Reference*>(addr.address);
                if(var.address != nullptr) {
                    drop(var);
                }
                var.type = 0;
                var.address = nullptr;
            } break;

            case Type::Tag::Sequence: {
                auto& seq = *reinterpret_cast<Type::Sequence*>(addr.address);
                rawlist_clear(seq.data);
            } break;

            case Type::Tag::List: {
                Type::List& list = *reinterpret_cast<Type::List*>(addr.address);
                for(size_t i = 0; i < list.data.length; ++i) {
                    drop(list_cell(addr, i));
                }
                rawlist_clear(list.data);
            } break;

            case Type::Tag::Record: {
                size_t schema_id = type_innerkey(addr.type);
                auto binding = DB_SCHEMA.get(schema_id);
                if(binding != nullptr) {
                    for(size_t i = 0; i < binding->data.size(); ++i) {
                        auto cell = record_cell(addr, i);
                        drop(cell);
                    }
                }
            } break;

            case Type::Tag::Schema: {
                auto& object = (*reinterpret_cast<Type::Schema*>(addr.address));
                rawlist_clear(object.data);
                rawlist_clear(object.map);
            } break;
        }
        memset(addr.address, 0, type_size(addr.type));
    }
}

extern "C" void release(Reference addr)
{
    if(addr.address != nullptr) {
        drop(addr);
        free(addr.address);
    }
}

extern "C" bool copy(Reference dst, Reference src)
{
    if(src.address != dst.address) {
        Reference source = src;
        Reference destination = dst;
        
        // dereference varying data
        if(type_key(src.type) == Type::Tag::Varying) {
            source = *reinterpret_cast<Reference*>(src.address);
        }

        // prepare destination for varying data
        if(type_key(dst.type) == Type::Tag::Varying) {
            auto& dest_ref = *reinterpret_cast<Reference*>(dst.address);
            
            // determine if memory can be reused, otherwise free and reallocate
            if(source.type != dest_ref.type) {
                if(dest_ref.address != nullptr) {
                    free(dest_ref.address);
                    dest_ref.type = Type::Tag::Null;
                    dest_ref.address = nullptr;
                }
                dest_ref = acquire(source.type);
            }
        }

        // copy data into destination
        if(source.type == destination.type) {
            drop(destination);

            switch(type_key(destination.type)) {
                case Type::Tag::Null: { } break;

                case Type::Tag::Sequence: {
                    auto& src_seq = *reinterpret_cast<Type::Sequence*>(source.address);
                    auto& dst_seq = *reinterpret_cast<Type::Sequence*>(destination.address);

                    rawlist_clear(dst_seq.data);
                    rawlist_reserve(dst_seq.data, sizeof(uint8_t), src_seq.data.length);
                    memcpy(dst_seq.data.data, src_seq.data.data, sizeof(uint8_t) * src_seq.data.length);
                    dst_seq.data.length = src_seq.data.length;
                } break;

                case Type::Tag::Array: {
                    for(size_t i = 0; i < array_length(source); ++i) {
                        copy(array_cell(destination, i), array_cell(source, i));
                    }
                } break;
                
                case Type::Tag::List: {
                    auto& src_list = *reinterpret_cast<Type::List*>(source.address);
                    auto& dst_list = *reinterpret_cast<Type::List*>(destination.address);

                    rawlist_reserve(dst_list.data, type_size(type_inner(source.type)), src_list.data.capacity);
                    dst_list.data.length = src_list.data.length;

                    for(size_t i = 0; i < src_list.data.length; ++i) {
                        copy(list_at(destination, i), list_at(source, i));
                    }
                } break;

                case Type::Tag::Record: {
                    size_t schema_id = type_innerkey(source.type);
                    auto binding = DB_SCHEMA.get(schema_id);
                    if(binding != nullptr) {
                        for(size_t i = 0; i < binding->data.size(); ++i) {
                            auto src_cell = record_cell(source, i);
                            auto dst_cell = record_cell(destination, i);
                            copy(dst_cell, src_cell);
                        }
                    }
                } break;

                case Type::Tag::Schema: {
                    auto& src_schema = *reinterpret_cast<Type::Schema*>(source.address);
                    auto& dst_schema = *reinterpret_cast<Type::Schema*>(destination.address);

                    rawlist_clear(dst_schema.data);
                    rawlist_clear(dst_schema.map);
                    rawlist_reserve(dst_schema.data, sizeof(size_t), src_schema.data.length);
                    rawlist_reserve(dst_schema.map, sizeof(size_t), src_schema.map.length);

                    for(size_t i = 0; i < src_schema.data.length; ++i) {
                        auto src_cell = reinterpret_cast<size_t*>(rawlist_cell(src_schema.data, sizeof(size_t), i));
                        auto dst_cell = reinterpret_cast<size_t*>(rawlist_cell(dst_schema.data, sizeof(size_t), i));
                        if(src_cell != nullptr && dst_cell != nullptr) {
                            *dst_cell = *src_cell;
                        }
                    }

                    for(size_t i = 0; i < src_schema.map.length; ++i) {
                        auto src_cell = reinterpret_cast<size_t*>(rawlist_cell(src_schema.map, sizeof(size_t), i));
                        auto dst_cell = reinterpret_cast<size_t*>(rawlist_cell(dst_schema.map, sizeof(size_t), i));
                        if(src_cell != nullptr && dst_cell != nullptr) {
                            *dst_cell = *src_cell;
                        }
                    }
                } break;

                default: {
                    memcpy(destination.address, source.address, type_size(source.type));
                } break;
            }

            return true;
        }
    }
    return false;
}

extern "C" bool transfer(Reference dst, Reference src)
{
    if(src.address != dst.address && src.type != 0 && dst.type != 0) {
        Reference source = src;
        Reference destination = dst;
        
        // dereference varying data
        if(type_key(src.type) == Type::Tag::Varying) {
            source = *reinterpret_cast<Reference*>(src.address);
        }

        // prepare destination for varying data
        if(type_key(dst.type) == Type::Tag::Varying) {
            auto& dest_ref = *reinterpret_cast<Reference*>(dst.address);
            
            // determine if memory can be reused, otherwise free and reallocate
            if(source.type != dest_ref.type) {
                if(dest_ref.address != nullptr) {
                    free(dest_ref.address);
                    dest_ref.type = Type::Tag::Null;
                    dest_ref.address = nullptr;
                }
                dest_ref = acquire(source.type);
            }
        }

        // copy data into destination
        if(source.type == destination.type) {
            drop(destination);
            memcpy(destination.address, source.address, type_size(source.type));
            memset(source.address, 0, type_size(source.type));
            return true;
        }
    }
    return false;
}

Reference resolve_addr(Reference addr)
{
    Reference result = addr;
    if(result.type == Type::Tag::Null) {
        if(result.address != nullptr) {
            result = *reinterpret_cast<Reference*>(result.address);
        }
        else {
            result = Reference {0};
        }
    }
    return result;
}


// Varying //

extern "C" Reference varying_get(Reference addr)
{
    Reference result {0};
    result = *reinterpret_cast<Reference*>(addr.address);
    return result;
}

extern "C" void varying_set(Reference addr, Reference source)
{
    Reference& var = *reinterpret_cast<Reference*>(addr.address);
    
    if(var.address != nullptr) {
        drop(var);
    }

    if(var.type != source.type || var.address == nullptr) {
        if(var.address != nullptr) {
            free(var.address);
        }
        var.type = source.type;
        var.address = allocate(source.type, 1);
    }

    copy(var, source);
}

extern "C" void varying_clear(Reference addr)
{
    Reference& var = *reinterpret_cast<Reference*>(addr.address);
    
    if(var.address != nullptr) {
        drop(var);
        free(var.address);
        var.type = 0;
        var.address = nullptr;
    }
}


// Boolean //

extern "C" void bool_set(Reference addr, Type::Boolean value)
{
    *(reinterpret_cast<Type::Boolean*>(addr.address)) = value;
}

extern "C" Type::Boolean bool_get(Reference addr)
{
    return *(reinterpret_cast<Type::Boolean*>(addr.address));
}


// Natural //

extern "C" void natural_set(Reference addr, Type::Natural value)
{
    *(reinterpret_cast<Type::Natural*>(addr.address)) = value;
}

extern "C" Type::Natural natural_get(Reference addr)
{
    return *(reinterpret_cast<Type::Natural*>(addr.address));
}


// Integer //

extern "C" void integer_set(Reference addr, Type::Integer value)
{
    *(reinterpret_cast<Type::Integer*>(addr.address)) = value;
}

extern "C" Type::Integer integer_get(Reference addr)
{
    return *(reinterpret_cast<Type::Integer*>(addr.address));
}


// Significant //

extern "C" void significant_set(Reference addr, Type::Significant value)
{
    *(reinterpret_cast<Type::Significant*>(addr.address)) = value;
}

extern "C" Type::Significant significant_get(Reference addr)
{
    return *(reinterpret_cast<Type::Significant*>(addr.address));
}


// Block //

extern "C" size_t block_length(Reference addr)
{
    return type_innerkey(addr.type);
}

extern "C" uint8_t block_get(Reference addr, size_t index)
{
    size_t length = type_innerkey(addr.type);
    if(index < length) {
        return reinterpret_cast<uint8_t*>(addr.address)[index];
    }
    return 0;
}

extern "C" void block_set(Reference addr, size_t index, uint8_t value)
{
    size_t length = type_innerkey(addr.type);
    if(index < length) {
        reinterpret_cast<uint8_t*>(addr.address)[index] = value;
    }
}


// Sequence //

extern "C" size_t sequence_capacity(Reference addr)
{
    Type::Sequence& seq = *reinterpret_cast<Type::Sequence*>(addr.address);
    return seq.data.capacity;
}

extern "C" size_t sequence_length(Reference addr)
{
    Type::Sequence& seq = *reinterpret_cast<Type::Sequence*>(addr.address);
    return seq.data.length;
}

extern "C" uint8_t sequence_get(Reference addr, size_t index)
{
    Type::Sequence& seq = *reinterpret_cast<Type::Sequence*>(addr.address);
    if(index < seq.data.length) {
        return *reinterpret_cast<uint8_t*>(rawlist_cell(seq.data, 1, index));
    }
    return 0;
}

extern "C" void sequence_clear(Reference addr)
{
    Type::Sequence& seq = *reinterpret_cast<Type::Sequence*>(addr.address);
    rawlist_clear(seq.data);
}

extern "C" void sequence_set(Reference addr, size_t index, uint8_t value)
{
    Type::Sequence& seq = *reinterpret_cast<Type::Sequence*>(addr.address);
    auto cell = reinterpret_cast<uint8_t*>(rawlist_cell(seq.data, 1, index));
    if(cell != nullptr) {
        *cell = value;
    }
}

extern "C" void sequence_insert(Reference addr, size_t index, uint8_t value)
{
    Type::Sequence& seq = *reinterpret_cast<Type::Sequence*>(addr.address);
    auto cell = reinterpret_cast<uint8_t*>(rawlist_insert(seq.data, 1, index));
    if(cell != nullptr) {
        *cell = value;
    }
}

extern "C" void sequence_reserve(Reference addr, size_t capacity)
{
    Type::Sequence& seq = *reinterpret_cast<Type::Sequence*>(addr.address);
    rawlist_reserve(seq.data, 1, capacity);
}


// Array //

extern "C" size_t array_length(Reference addr)
{
    return type_innerkey(addr.type);
}

Reference array_cell(Reference addr, size_t index)
{
    Reference result {0};
    size_t length_n = type_inner(addr.type);
    size_t length = type_key(length_n);
    size_t type = type_inner(length_n);
    size_t offset = type_size(type);

    // validate for overflow
    if(addr.address != nullptr && offset > 0 && index < length) {
        result.type = type;
        result.address = addr.address + (offset * index);
    }
    return result;
}

extern "C" Reference array_at(Reference addr, size_t index)
{
    Reference result {0};
    Reference cell = array_cell(addr, index);
    if(cell.address != nullptr) {
        if(type_key(cell.type) == Type::Tag::Varying) {
            result = varying_get(cell);
        }
        else {
            result = cell;
        }
    }
    return result;
}

extern "C" void array_update(Reference addr, size_t index, Reference source)
{
    Reference cell = array_cell(addr, index);
    if(type_key(cell.type) == Type::Tag::Varying) {
        varying_set(cell, source);
    }
    else {
        copy(cell, source);
    }
}


// List //

extern "C" size_t list_capacity(Reference addr)
{
    return (*reinterpret_cast<Type::List*>(addr.address)).data.capacity;
}

extern "C" size_t list_length(Reference addr)
{
    return (*reinterpret_cast<Type::List*>(addr.address)).data.length;
}

Reference list_cell(Reference addr, size_t index)
{
    Reference result {0};
    Type::List& list = *reinterpret_cast<Type::List*>(addr.address);
    size_t inner = type_inner(addr.type);
    size_t offset = type_size(inner);

    // validate for overflow
    if(list.data.data != nullptr && offset > 0 && index < list.data.capacity) {
        result.type = inner;
        result.address = rawlist_cell(list.data, offset, index);
    }

    return result;
}

extern "C" Reference list_at(Reference addr, size_t index)
{
    Reference result {0};
    Reference cell = list_cell(addr, index);

    if(cell.address != nullptr) {
        if(type_key(cell.type) == Type::Tag::Varying) {
            result = *reinterpret_cast<Reference*>(cell.address);
        } else {
            result = cell;
        }
    }
    return result;
}

/*extern "C" Reference list_first(Reference addr)
{
    Reference result {0};
    if(type_key(addr.type) == Type::Tag::List) {
        auto& list = (*reinterpret_cast<Type::List*>(addr.address));
        if(list.length > 0) {
            result = list_at(addr, 0);
        }
    }
    return result;
}*/

/*extern "C" Reference list_last(Reference addr)
{
    Reference result {0};
    if(type_key(addr.type) == Type::Tag::List) {
        auto& list = (*reinterpret_cast<Type::List*>(addr.address));
        if(list.length > 0) {
            result = list_at(addr, list.length - 1);
        }
    }
    return result;
}*/

extern "C" void list_clear(Reference addr)
{
    drop(addr);
}

extern "C" void list_insert(Reference addr, size_t index, Reference source)
{
    auto& list = (*reinterpret_cast<Type::List*>(addr.address));
    size_t inner = type_inner(addr.type);
    size_t offset = type_size(inner);

    if(index > list.data.length) { index = list.data.length; }

    void* cell = rawlist_insert(list.data, offset, index);

    if(type_key(inner) == Type::Tag::Varying) {
        varying_set(list_cell(addr, index), source);
    }
    else {
        copy(list_cell(addr, index), source);
    }
}

extern "C" void list_prepend(Reference addr, Reference source)
{
    list_insert(addr, 0, source);
}

extern "C" void list_append(Reference addr, Reference source)
{
    auto& list = (*reinterpret_cast<Type::List*>(addr.address));
    list_insert(addr, list.data.length, source);
}

extern "C" void list_update(Reference addr, size_t index, Reference source)
{
    auto& list = (*reinterpret_cast<Type::List*>(addr.address));
    if(index < list.data.length) {
        size_t inner = type_inner(addr.type);
        size_t offset = type_size(inner);

        if(type_key(inner) == Type::Tag::Varying) {
            varying_set(list_cell(addr, index), source);
        }
        else {
            copy(list_cell(addr, index), source);
        }
    }
}

/*extern "C" void list_truncate(Reference addr, size_t maximum)
{
    if(type_key(addr.type) == Type::Tag::List) {
        //auto& list = (*reinterpret_cast<Type::List*>(addr.address));
        
    }
}*/

/*extern "C" void list_shift(Reference addr)
{
    if(type_key(addr.type) == Type::Tag::List) {
        //auto& list = (*reinterpret_cast<Type::List*>(addr.address));
        
    }
}*/

extern "C" void list_remove(Reference addr, size_t index)
{
    auto& list = (*reinterpret_cast<Type::List*>(addr.address));
    size_t inner = type_inner(addr.type);
    size_t offset = type_size(inner);
    if(index < list.data.length) {
        drop(list_at(addr, index));
        rawlist_remove(list.data, offset, index);
    }
}

extern "C" void list_reserve(Reference addr, size_t capacity)
{
    auto& list = (*reinterpret_cast<Type::List*>(addr.address));
    size_t inner = type_inner(addr.type);
    size_t offset = type_size(inner);

    rawlist_reserve(list.data, offset, capacity);
}

/*extern "C" void list_resize(Reference addr, size_t length)
{
    if(type_key(addr.type) == Type::Tag::List) {
        //auto& list = (*reinterpret_cast<Type::List*>(addr.address));
        
    }
}*/


// Record //

extern "C" size_t record_length(Reference addr)
{
    auto binding = DB_SCHEMA.get(type_innerkey(addr.type));
    if(binding != nullptr) {
        return binding->data.size();
    }
    return 0;
}

extern "C" size_t record_type(Reference addr, size_t index)
{
    auto binding = DB_SCHEMA.get(type_innerkey(addr.type));
    if(binding != nullptr && index < binding->data.size()) {
        return binding->data[index].type;
    }
    return 0;
}

Reference record_cell(Reference addr, size_t index)
{
    Reference result {0};
    auto binding = DB_SCHEMA.get(type_innerkey(addr.type));
    if(binding != nullptr) {
        if(index < binding->data.size()) {
            result.type = binding->data[index].type;
            result.address = addr.address + binding->data[index].offset;
        }
    }
    return result;
}

extern "C" Reference record_at(Reference addr, size_t index)
{
    Reference result = record_cell(addr, index);
    if(type_key(result.type) == Type::Tag::Varying) {
        result = varying_get(result);
    }
    return result;
}

extern "C" void record_update(Reference addr, size_t index, Reference source)
{
    Reference destination {0};
    auto binding = DB_SCHEMA.get(type_innerkey(addr.type));
    if(binding != nullptr) {
        if(index < binding->data.size()) {
            destination.type = binding->data[index].type;
            destination.address = addr.address + binding->data[index].offset;
            copy(destination, source);
        }
    }
}

extern "C" size_t record_keyof(Reference addr, size_t index)
{
    Reference destination {0};
    auto binding = DB_SCHEMA.get(type_innerkey(addr.type));
    if(binding != nullptr) {
        if(index < binding->data.size()) {
            return binding->data[index].key;
        }
    }
    return 0;
}

extern "C" size_t record_indexof(Reference addr, size_t key)
{
    Reference destination {0};
    auto binding = DB_SCHEMA.get(type_innerkey(addr.type));
    if(binding != nullptr) {
        auto result = binding->map.get(key);
        if(result != nullptr) {
            return *result;
        }
        return binding->data.size();
    }
    return 0;
}


// Schema //

extern "C" size_t schema_length(Reference addr)
{
    auto& object = (*reinterpret_cast<Type::Schema*>(addr.address));
    return object.data.length;
}

extern "C" size_t schema_insert(Reference addr, size_t index, size_t type_id)
{
    auto& object = (*reinterpret_cast<Type::Schema*>(addr.address));
    if(index >= object.data.length) {
        index = object.data.length;
    }
    void* cell = rawlist_insert(object.data, sizeof(size_t), index);
    *reinterpret_cast<size_t*>(cell) = type_id;
    return index;
}

extern "C" void schema_update(Reference addr, size_t index, size_t type_id)
{
    auto& object = (*reinterpret_cast<Type::Schema*>(addr.address));
    if(index < object.data.length) {
        auto cell = reinterpret_cast<size_t*>(rawlist_cell(object.data, sizeof(size_t), index));
        if(cell != nullptr) {
            *cell = type_id;
        }
    }
}

extern "C" size_t schema_get(Reference addr, size_t index)
{
    auto& object = (*reinterpret_cast<Type::Schema*>(addr.address));
    if(index < object.data.length) {
        auto cell = reinterpret_cast<size_t*>(rawlist_cell(object.data, sizeof(size_t), index));
        if(cell != nullptr) {
            return *cell;
        }
    }
    return 0;
}

extern "C" void schema_remove(Reference addr, size_t index)
{
    auto& object = (*reinterpret_cast<Type::Schema*>(addr.address));
    if(index < object.data.length) {
        rawlist_remove(object.data, sizeof(size_t), index);

        // remove mapping
        for(size_t i = 0; i < object.map.length; i++) {
            auto cell = reinterpret_cast<Type::Schema::Mapping*>(rawlist_cell(object.data, sizeof(Type::Schema::Mapping), i));
            if(cell != nullptr) {
                if(cell->index == index) {
                    rawlist_remove(object.map, sizeof(Type::Schema::Mapping), i);
                    return;
                }
            }
        }
    }
}

extern "C" void schema_map(Reference addr, size_t key, size_t index)
{
    auto& object = (*reinterpret_cast<Type::Schema*>(addr.address));
    if(index < object.data.length) {
        size_t find_index = 0;
        for(; find_index < object.map.length; find_index++) {
            auto cell = reinterpret_cast<Type::Schema::Mapping*>(rawlist_cell(object.map, sizeof(Type::Schema::Mapping), find_index));
            if(cell != nullptr) {
                if(cell->key == key) { break; }
            }
        }

        // if key is not found, add new mapping
        if(find_index == object.map.length) {
            auto cell = reinterpret_cast<Type::Schema::Mapping*>(rawlist_insert(object.map, sizeof(Type::Schema::Mapping), object.map.length));
            cell->key = key;
            cell->index = index;
        }
        // otherwise, update existing key
        else {
            auto cell = reinterpret_cast<Type::Schema::Mapping*>(rawlist_cell(object.map, sizeof(Type::Schema::Mapping), find_index));
            cell->index = index;
        }
    }
}

extern "C" void schema_unmap(Reference addr, size_t key)
{
    auto& object = (*reinterpret_cast<Type::Schema*>(addr.address));
    for(size_t i; i < object.map.length; i++) {
        auto cell = reinterpret_cast<Type::Schema::Mapping*>(rawlist_cell(object.map, sizeof(Type::Schema::Mapping), i));
        if(cell != nullptr) {
            if(cell->key == key) {
                rawlist_remove(object.map, sizeof(Type::Schema::Mapping), i);
                return;
            }
        }
    }
}

extern "C" size_t schema_indexof(Reference addr, size_t key)
{
    auto& object = (*reinterpret_cast<Type::Schema*>(addr.address));
    for(size_t i = 0; i < object.map.length; i++) {
        auto cell = reinterpret_cast<Type::Schema::Mapping*>(rawlist_cell(object.map, sizeof(Type::Schema::Mapping), i));
        if(cell != nullptr) {
            if(cell->key == key) {
                return cell->index;
            }
        }
    }
    return object.data.length;
}

extern "C" size_t schema_keyof(Reference addr, size_t index)
{
    auto& object = (*reinterpret_cast<Type::Schema*>(addr.address));
    for(size_t i = 0; i < object.map.length; i++) {
        auto cell = reinterpret_cast<Type::Schema::Mapping*>(rawlist_cell(object.map, sizeof(Type::Schema::Mapping), i));
        if(cell != nullptr) {
            if(cell->index == index) {
                return cell->key;
            }
        }
    }
    return object.data.length;
}

extern "C" size_t schema_bind(Reference addr, size_t id)
{
    if(id > 0) {
        Type::SchemaBinding binding {0};
        auto& object = (*reinterpret_cast<Type::Schema*>(addr.address));

        // prepare binding
        binding.binding = id;
        for(size_t i = 0; i < object.data.length; ++i) {
            Type::SchemaBinding::Row row {0};

            size_t type_id = *reinterpret_cast<size_t*>(rawlist_cell(object.data, sizeof(size_t), i));
            size_t size = type_size(type_id);
            size_t alignment = type_alignment(type_id);
            binding.alignment = std::max(alignment, binding.alignment);
            size_t position = ((binding.size + (alignment - 1)) & ~(alignment - 1));
            binding.size = size + position;

            row.type = type_id;
            row.offset = position;
            binding.data.push_back(row);
        }
        binding.size = ((binding.size + (binding.alignment - 1)) & ~(binding.alignment - 1));
        binding.references = 0;

        for(size_t i = 0; i < object.map.length; ++i) {
            auto cell = reinterpret_cast<Type::Schema::Mapping*>(rawlist_cell(object.map, sizeof(Type::Schema::Mapping), i));
            binding.map.set(cell->key, cell->index);
            binding.data[cell->index].key = cell->key;
        }

        /* printf("[Binding]\n");
        printf("  Id:    %zu\n", binding.binding);
        printf("  Size:  %zu\n", binding.size);
        printf("  Align: %zu\n", binding.alignment);
        printf("  Data:\n");
        for(size_t i = 0; i < binding.data.size(); ++i) {
            printf("   - %zu {%#x} (%zu)\n",
                binding.data[i].type,
                type_key(binding.data[i].type),
                binding.data[i].offset
            );
        }
        printf("  Map:\n");
        for(size_t key : binding.map.indices()) {
            printf("   - %zu -> %zu\n",
                key,
                *binding.map.get(key)
            );
        } */

        // add binding to pool
        DB_SCHEMA.set(id, binding);
    }

    return id;
}

extern "C" bool schema_has(size_t id)
{
    return DB_SCHEMA.has(id);
}
