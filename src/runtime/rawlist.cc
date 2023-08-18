#include "rawlist.h"

void* rawlist_insert(RawList& list, size_t offset, size_t index)
{
    if(list.length == list.capacity) { rawlist_reserve(list, offset, (list.capacity == 0) + list.capacity * 2); }
    if(index > list.length) { index = list.length; }

    // shift elements back until index is reached
    for(size_t i = list.length; i > index; --i) {
        memcpy(rawlist_cell(list, offset, i), rawlist_cell(list, offset, i - 1), offset);
    }

    list.length++;
    return rawlist_cell(list, offset, index);
}

void rawlist_remove(RawList& list, size_t offset, size_t index)
{
    if(index > list.length) { index = list.length - 1; }

    // shift elements back until index is reached
    for(size_t i = index; i < list.length - 1; ++i) {
        memcpy(rawlist_cell(list, offset, i), rawlist_cell(list, offset, i + 1), offset);
    }

    list.length--;
    memset(rawlist_cell(list, offset, list.length), 0, offset);
}

void rawlist_reorder(RawList& list, size_t offset, size_t from, size_t to)
{
    // ensure there is a trailing cell in which to swap moving data
    if(list.length == list.capacity) { rawlist_reserve(list, offset, list.capacity * 2); }

    if(from > list.length) { from = list.length - 1; }
    if(to > list.length) { to = list.length - 1; }
    if(from != to) {
        // move data to unused cell
        memcpy(rawlist_cell(list, offset, from), rawlist_cell(list, offset, list.length), offset);

        if(from < to) {
            for(size_t i = from; i < to; ++i) {
                memcpy(rawlist_cell(list, offset, i), rawlist_cell(list, offset, i + 1), offset);
            }
        }
        else if(from > to) {
            for(size_t i = from; i > to; --i) {
                memcpy(rawlist_cell(list, offset, i), rawlist_cell(list, offset, i - 1), offset);
            }
        }

        // move data to target cell and clear swap cell
        memcpy(rawlist_cell(list, offset, from), rawlist_cell(list, offset, list.length), offset);
        memset(rawlist_cell(list, offset, list.length), 0, offset);
    }
}

inline uint8_t* rawlist_cell(RawList& list, size_t offset, size_t index)
{
    if(index < list.capacity) {
        return list.data + (offset * index);
    }
    else {
        return nullptr;
    }
}

void rawlist_reserve(RawList& list, size_t offset, size_t capacity)
{
    if(capacity > list.capacity) {
        size_t block_size = offset * capacity;
        void* old_data = list.data;

        list.data = reinterpret_cast<uint8_t*>(malloc(block_size));
        memset(list.data, 0, block_size);
        if(old_data != nullptr) {
            memcpy(list.data, old_data, offset * list.length);
            free(old_data);
        }

        list.capacity = capacity;
    }
}

void rawlist_clear(RawList& list)
{
    if(list.data != nullptr) {
        free(list.data);
    }
    list.capacity = 0;
    list.length = 0;
    list.data = nullptr;
}
