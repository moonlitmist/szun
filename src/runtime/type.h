#include <cstdint>
#include <vector>
#include "sparselist.h"
#include "rawlist.h"

extern "C" struct Reference {
    size_t   type;
    uint8_t* address;
};

namespace Type {

namespace Tag {
    enum {
        Null     = 0x00,
        Varying  = 0x01,
        Boolean  = 0x02,
        Natural  = 0x10,
        Integer  = 0x11,
        //Decimal  = 0x12,
        Significant = 0x13,
        Block    = 0x1e,
        Sequence = 0x1f,
        Array    = 0x22,
        List     = 0x23,
        //Sparse   = 0x24,
        Record   = 0x7e,
        Schema   = 0x7f,
    };
}

typedef bool     Boolean;
typedef uint64_t Natural;
typedef int64_t  Integer;
typedef double   Significant;

struct Sequence {
    RawList data;
};

struct List {
    RawList data;
};

struct Sparse {
    struct Header {
        size_t start;
        size_t length;
        size_t index;

        Header() {
            start = 0;
            length = 0;
            index = 0;
        }
    };

    RawList data;
    RawList header;
};

struct Schema {
    struct Mapping {
        size_t key;
        size_t index;
    };

    RawList data;
    RawList map;
};

struct SchemaBinding {
    struct Row {
        size_t key;
        size_t type;
        size_t offset;
    };

    size_t binding;
    size_t alignment;
    size_t size;
    size_t references;
    
    SparseList<size_t> map;
    std::vector<Row> data;
};

}
