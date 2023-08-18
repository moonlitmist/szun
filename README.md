# Suzu Runtime / Notation
Szun is a library for defining, manipulating, and formatting structured, dynamically-typed data.

> Version 0.0.1 is for reference purposes and contains unimplemented features, bugs, and documentation errors.

## Runtime
The Szun runtime provides an interface for constructing and modfying data objects.

## Encoding
Szun uses a tag-prefixed, hierarchical, binary encoding with variable-sized values to produce serializations of reduced size.

## Language
Szun notation provides a human-readable format for structures and data to be defined.
The notation may be parsed and loaded at runtime or compiled directly into encoded format.


# Runtime Documentation

## Conventions

### Namespacing
Examples in this documentation will assume inclusion of the szun namespace.
Practical examples should use `szun::` or `use szun::{...};` in limited scopes.

### Interface Usage
While the runtime provides methods for directly acquring and releasing memory, it is recommended that developers use typed interfaces to ensure memory is freed upon leaving scope.


## Data Types

|Type|Code|Parameters|Description|
|---|---|---|---|
|Undefined|x00|--|Data is not defined|
|Varying|x01|--|Stores any data type|
|Boolean|x02|--|True or false|
|Natural|x10|--|Non-negative integers|
|Integer|x11|--|Integers|
|Significant|x13|--|Fixed-precision, variable-magnitude numbers|
|Block|x1e|size|Constant-sized series of bytes|
|Sequence|x1f|--|Variable-sized series of bytes|
|Array|x22|size, type|Constant-sized, ordered collection|
|List|x23|type|Variable-sized, ordered collection|
|Record|x7e|schema|Instance of a schema|
|Schema|x7f|--|Definition of abstract structure|

### Type Building
Type building functions are used to generate identifiers used in the construction of complex data types.

* `varying()`
* `boolean()`
* `natural()`
* `integer()`
* `significant()`
* `block(size)`
* `sequence()`
* `array(size, type)`
* `list(type)`
* `record(schema)`
* `schema()`

#### Example
This example produces an identifier representing a block of 4 bytes, which can be used to construct a list containing that type.
```
let type_id = block(4);
let list = List::new(type_id);
```

## Code Examples

### Example 1
```
const VEC3F :usize = 0x200;
const VEC3U :usize = 0x201;
const MESH  :usize = 0x202;

fn vec3f(x:f64, y:f64, z:f64) -> szun::Record
{
    use szun::*;
    Record::with_values(VEC3F, vec![
        *Significant::with(x),
        *Significant::with(y),
        *Significant::with(z),
    ]).unwrap()
}

fn vec3u(x:u64, y:u64, z:u64) -> szun::Record
{
    use szun::*;
    Record::with_values(VEC3U, vec![
        *Natural::with(x),
        *Natural::with(y),
        *Natural::with(z),
    ]).unwrap()
}

fn main()
{
    use szun::*;

    // Vec3f
    Schema::with(vec![
        ("x", significant()),
        ("y", significant()),
        ("z", significant()),
    ]).bind(VEC3F);

    // Vec3u
    Schema::with(vec![
        ("x", natural()),
        ("y", natural()),
        ("z", natural()),
    ]).bind(VEC3U);

    // Mesh
    Schema::with(vec![
        ("vertices", list(record(VEC3F))),
        ("faces", list(record(VEC3U))),
    ]).bind(MESH);

    let pyramid = Record::with(MESH, vec![
        ("vertices", *List::with(record(VEC3F), vec![
            *vec3f(0.0, 2.0, 0.0),
            *vec3f(1.0, 0.0, 1.0),
            *vec3f(1.0, 0.0, -1.0),
            *vec3f(-1.0, 0.0, -1.0),
            *vec3f(-1.0, 0.0, 1.0),
        ])),
        ("faces", *List::with(record(VEC3U), vec![
            *vec3u(0, 1, 2),
            *vec3u(0, 2, 3),
            *vec3u(0, 3, 4),
            *vec3u(0, 4, 1),
            *vec3u(1, 2, 3),
            *vec3u(1, 3, 4),
        ])),
    ]).unwrap();
}
```

## Global Functions

`acquire(type) -> Reference`

Allocate a new instance of the provided type.
```
let refer = acquire(list(integer()));
```
---

`release(refer:Reference)`

Destruct and deallocate a type.
```
release(refer);
```
---

`transfer(dst:Reference, src:Reference) -> Result<(),()>`

Move an object from one location to another, clearing the original.

```
let original = Sequence::with("Hello, world!");
let target = Sequence::new();

println!("{}", original.get());  // prints "Hello, world!"
println!("{}", target.get());    // prints ""

transfer(target, original).ok();

println!("{}", original.get());  // prints ""
println!("{}", target.get());    // prints "Hello, world!"
```
---

`copy(dst:Reference, src:Reference) -> Result<(),()>`

Copy the contents of an objcet to another location, keeping the original.

```
let original = Sequence::with("Hello, world!");
let target = Sequence::new();

println!("{}", original.get());  // prints "Hello, world!"
println!("{}", target.get());    // prints ""

copy(target, original).ok();

println!("{}", original.get());  // prints "Hello, world!"
println!("{}", target.get());    // prints "Hello, world!"
```
---

## Encoding
Encoding converts data between runtime memory and binary serialization.

`encode(refer:Reference) -> Vec<u8>`  
`encode_raw(refer:Reference) -> Vec<u8>`  
`encode_tag(refer:Reference) -> Vec<u8>`

Serializes an object into binary encoding.  
The raw variant does not produce a tag prefix for the root object.  
The tag variant only produces a tag prefix.

---

`decode(data:&Vec<u8>, index:&mut usize) -> Result<Type,()>`  
`decode_raw(data:&Vec<u8>, type_id:usize, index:&mut usize) -> Result<Type,()>`  
`decode_tag(data:&Vec<u8>, index:&mut usize) -> Result<usize,()>`

Parses a valid binary encoding and produces the represented object.  
The raw variant does not decode a tag prefix on the root object.  
The tag variant only decodes a tag prefix.

---

## Language Compiler
> Not implemented

## Common Methods

`new() -> Self`

```
let value = Integer::new();
```
---

`from(refer:Reference) -> Result<Self,()>`

```
match Integer::from(list.at(0)) {
    Ok(int) => { println!("Integer: {}", int.get()); }
    Err(_) => { println!("Not Integer"); }
}
```
---

`with(value) -> Self`

```
let b = Boolean::with(true);
```
---

`detatch()`

Prevents an allocated object from being dropped when the interface goes out of scope.

---

`*Dereference -> Reference`

```
let refer = *Integer::new();
```
---


## Varying
Stores a value of any other type.

`is_null() -> bool`

Indicates whether or not the variable contains a object.

---

`get() -> Reference`

Returns a reference to the contained object.

```
let var = Varying::with(*Boolean::with(true));
let value = Boolean::from(var.get()).unwrap();
```
---

`set(refer:Reference)`

Replaces the contained object.

```
let var = Varying::new();
var.set(*Sequence::with("Hello!"));
```
---

`clear()`

Removes the contained object.

---


## Boolean
Stores the value true or false.

`get() -> bool`

Returns the contained value.

```
let value = Boolean::with(true);
if value.get() {
    println!("True");
}
```
---

`set(value:bool)`

Replaces the contained value.

```
let mut value = Boolean::new();
value.set(true);
```
---


## Natural
Stores a non-negative integer value.

`get() -> u64`

Returns the contained value.

```
let value = Integer::with(-1);
println!("{}", value.get());
```
---

`set(value:u64)`

Replaces the contained value.

```
let mut value = Integer::new();
value.set(-273);
```
---


## Integer
Stores a signed integer value.

`get() -> i64`

Returns the contained value.

```
let value = Integer::with(-1);
println!("{}", value.get());
```
---

`set(value:i64)`

Replaces the contained value.

```
let mut value = Integer::new();
value.set(-273);
```
---

## Decimal
Stores a constant-magnitude number with whole and decimal components.

> Not implemented.

## Significant
Stores a fixed-precision, variable-magnitude number.

> Encode not implemented.

`get() -> f64`

Returns the contained value.

---

`set(value:f64)`

Replaces the contained value.

---


## Block
Constant-sized series of bytes.

`new(size:usize) -> Block`

Produces a new block of the specified size.

---

`size() -> usize`

Returns the size of the allocated block.

---

`get() -> Vec<u8>`

Returns the contents of the block.

---

`at(index:usize) -> u8`

Returns the byte at the specified index or zero if out of bounds.

---

`set(data:Vec<u8>)`

Replaces the contents of the block up to the length of the parameter.

---

`set_at(index:usize, data:u8)`

Replaces the byte at the given index.

---


## Sequence
Variable-sized series of bytes.

`capacity() -> usize`

Returns the memory capacity of the sequeunce.

---

`size() -> usize`

Returns the length of the series.

---

`get() -> String`

Returns a UTF-8 string representation of the series.

---

`get_raw() -> Vec<u8>`

Returns the contents of the series.

---

`at(index:usize) -> u8`

Returns the byte at the specified index or zero if out of bounds.

---

`set(data:&str)`

Replaces the contents with the byte representation of a string.

---

`set_raw(data:Vec<u8>)`

Replaces the contents with a series of bytes.

---

`set_at(index:usize, data:u8)`

Replaces a byte at the given index.

---

`reserve(capacity:usize)`

Reallocates the sequeunce to have capacity not less than the specified size.

---


## Array
Constant-sized, ordered collection of items.

`new(length:usize, type_id:usize) -> Array`

Produces a new array of given type and length.

---

`length() -> usize`

Returns the length of the array.

---

`at(index:usize) -> Reference`

Returns a reference to the element at the given index or a null reference if out of bounds.

---

`set(index:usize, source:Reference)`

Replaces the element at the given index with a copy of the source.

---

`kindof() -> usize`

Returns the type identifier of the contents.

---


## List
Variable-sized, ordered collection of items.

`new(type_id:usize) -> List`

Produces a new list of the given type.

---

`capacity() -> usize`

Returns the allocated capacity of the list.

---

`length() -> usize`

Returns the length of the list.

---

`at(index:usize) -> Reference`

Returns the object at the given index or a null reference if out of bounds.

---

`set(index:usize, source:Reference)`

Replaces the object at the given index with a copy of the source.

---

`insert(index:usize, source:Reference)`

Inserts a copy of the source at a given index.

---

`remove(index:usize)`

Removes the object at the given index from the list.

---

`reserve(capacity:usize)`

Reallocates the list to have capacity not less than the specified size.

---

`clear()`

Removes all elements from the list.

---


## Schema
Definition of an abstract structure composed of named items.

`with(members:Vec<(&str, usize)>) -> Schema`

Produces a schema with the provided member assignments.

```
let schema_rgba = Schema::with(vec![
    ("r", significant()),
    ("g", significant()),
    ("b", significant()),
    ("a", significant()),
]);
```
---

`get(index:usize) -> usize`

Returns the type identifier of the given index or zero if out of bounds.

---

`add(type_id:usize) -> usize`

Appends a member of the specified type to the schema.

---

`remove(index:usize)`

Removes the member at the given index from the schema.

---

`assign(key:&str, type_id:usize) -> usize`

Appends a member of the specified type to the schema and maps the provided string to that index.

---

`map(key:&str, index:usize)`

Maps a string to the specified index.

---

`unmap(key:&str)`

Removes a mapping of the specified string from the schema.

---

`clear()`

Removes all members and mappings from the schema.

---

`bind(id:usize)`

Submits the template to the schema database under the provided identifier.

> Note: zero is used as a wildcard identifier and is not a valid parameter.

```
let vec3i = Schema::with(vec![
    ("x", integer()),
    ("y", integer()),
    ("z", integer()),
]).bind(0x100);
```
---


## Record
Instance of a schema.

`new(schema_id:usize) -> Result<Record,()>`

Produces a new record of the provided schema.

---

`with(schema_id:usize, data:Vec<(&str, Reference)>) -> Result<Record,()>`

Produces a record of the provided schema and keyed assignments.

---

`with_values(schema_id:usize, data:Vec<Reference>) -> Result<Record,()>`

Produces a record of the provided schema and indexed assignments.

---

`length() -> usisze`

Returns the number of elements in the record.

---

`at(index:usize) -> Reference`

Returns a reference to the member at the given index.

---

`set(index:usize, source:Reference)`

Replaces the member at the given index with a copy of the source.

---

`keyof(index:usize) -> String`

Returns the string mapped to the specified index or an empty string if no mapping exists.

---

`indexof(key:&str) -> usize`

Returns the index of a mapped string or the number of elements in the record if no mapping exists.

---

`kindof(index:usize) -> usize`

Returns the type identifier of the given index.

---
