#ifndef RAWLIST_H
#define RAWLIST_H

#include <cstdlib>
#include <cstring>

struct RawList {
    size_t offset;
    size_t capacity;
    size_t length;
    uint8_t* data;
};

void* rawlist_insert(RawList& list, size_t offset, size_t index);
void rawlist_remove(RawList& list, size_t offset, size_t index);
inline uint8_t* rawlist_cell(RawList& list, size_t offset, size_t index);
void rawlist_reserve(RawList& list, size_t offset, size_t capacity);
void rawlist_clear(RawList& list);

#endif
