#include <cstdint>
#include <cstdlib>
#include <cstring>

#include "type.h"
#include "typetree.h"
#include "nametree.h"
#include "sparselist.h"
#include "rawlist.h"
#include "pool.h"

extern "C" struct Str {
    uint8_t* bytes;
    size_t length;
};

extern TypeTree DB_TYPE;
extern NameTree DB_NAME;
extern SparseList<Type::SchemaBinding> DB_SCHEMA;
extern Pool<Reference> DB_REFERENCE;

extern "C" void test();

extern "C" size_t type_outer(size_t id, size_t key);
extern "C" size_t type_inner(size_t id);
extern "C" size_t type_key(size_t id);
extern "C" size_t type_innerkey(size_t id);
extern "C" size_t type_size(size_t type_id);
size_t type_alignment(size_t type_id);
extern "C" size_t kind_hasinner(size_t kind);
extern "C" size_t type_hasinner(size_t type_id);


extern "C" size_t name_indexof(const uint8_t* bytes, size_t length);
std::string name_keyof_internal(size_t index);
extern "C" Str name_keyof(size_t index);

uint8_t* allocate(size_t type_id, size_t count);
extern "C" Reference acquire(size_t type_id);
extern "C" void release(Reference id);
extern "C" bool copy(Reference dst, Reference src);
extern "C" bool transfer(Reference dst, Reference src);

Reference resolve_addr(Reference addr);

// Varying //
extern "C" Reference varying_get(Reference addr);
extern "C" void varying_set(Reference addr, Reference source);
extern "C" void varying_clear(Reference addr);

// Boolean //
extern "C" void bool_set(Reference addr, Type::Boolean value);
extern "C" Type::Boolean bool_get(Reference addr);

// Natural //p
extern "C" void natural_set(Reference addr, Type::Natural value);
extern "C" Type::Natural natural_get(Reference addr);

// Integer //
extern "C" void integer_set(Reference addr, Type::Integer value);
extern "C" Type::Integer integer_get(Reference addr);

// Significant //
extern "C" void significant_set(Reference addr, Type::Significant value);
extern "C" Type::Significant significant_get(Reference addr);

// Block //
extern "C" size_t block_length(Reference addr);
extern "C" uint8_t block_get(Reference addr, size_t index);
extern "C" void block_set(Reference addr, size_t index, uint8_t value);

// String //
extern "C" size_t sequence_capacity(Reference addr);
extern "C" size_t sequence_length(Reference addr);
extern "C" uint8_t sequence_get(Reference addr, size_t index);
extern "C" void sequence_clear(Reference addr);
extern "C" void sequence_set(Reference addr, size_t index, uint8_t value);
extern "C" void sequence_insert(Reference addr, size_t index, uint8_t value);
extern "C" void sequence_reserve(Reference addr, size_t capacity);

// Array //
extern "C" size_t array_length(Reference addr);
Reference array_cell(Reference addr, size_t index);
extern "C" Reference array_at(Reference addr, size_t index);
extern "C" void array_update(Reference addr, size_t index, Reference source);

// List //
extern "C" size_t list_capacity(Reference addr);
extern "C" size_t list_length(Reference addr);
Reference list_cell(Reference addr, size_t index);
extern "C" Reference list_at(Reference addr, size_t index);
//extern "C" Reference list_first(Reference addr);
//extern "C" Reference list_last(Reference addr);
extern "C" void list_clear(Reference addr);
extern "C" void list_insert(Reference addr, size_t index, Reference source);
extern "C" void list_prepend(Reference addr, Reference source);
extern "C" void list_append(Reference addr, Reference source);
extern "C" void list_update(Reference addr, size_t index, Reference source);
//extern "C" void list_truncate(Reference addr, size_t maximum);
//extern "C" void list_shift(Reference addr);
extern "C" void list_remove(Reference addr, size_t index);
extern "C" void list_reserve(Reference addr, size_t capacity);
//extern "C" void list_resize(Reference addr, size_t length);

// Record //
extern "C" size_t record_length(Reference addr);
extern "C" size_t record_type(Reference addr, size_t index);
Reference record_cell(Reference addr, size_t index);
extern "C" Reference record_at(Reference addr, size_t index);
extern "C" void record_update(Reference addr, size_t index, Reference source);
extern "C" size_t record_keyof(Reference addr, size_t index);
extern "C" size_t record_indexof(Reference addr, size_t key);

// Schema //
extern "C" size_t schema_length(Reference addr);
extern "C" size_t schema_insert(Reference addr, size_t index, size_t type_id);
extern "C" void schema_update(Reference addr, size_t index, size_t type_id);
extern "C" size_t schema_get(Reference addr, size_t index);
extern "C" void schema_remove(Reference addr, size_t index);
//extern "C" void schema_reorder(Reference addr, size_t index_from, size_t index_to);
extern "C" void schema_map(Reference addr, size_t key, size_t index);
extern "C" void schema_unmap(Reference addr, size_t key);
extern "C" size_t schema_indexof(Reference addr, size_t key);
extern "C" size_t schema_keyof(Reference addr, size_t index);
extern "C" size_t schema_bind(Reference addr, size_t key);
extern "C" size_t schema_binding(size_t key);
extern "C" bool schema_has(size_t id);
