#ifndef H_SPARSELIST
#define H_SPARSELIST

#include <vector>

template <typename T>
class SparseList {
public:
    SparseList()
    { }

    ~SparseList()
    { }

    size_t length() const
    {

    }

    void set(size_t index, const T& value)
    {
        // find first header with bounds less than index
        size_t header_index = 0;
        for(; header_index < m_headers.size() && index > (m_headers[header_index].start + m_headers[header_index].length); ++header_index) { }

        Header header {0};

        if(header_index == m_headers.size()) {
            // append header to end
            header.start = index;
            header.length = 1;
            header.index = m_data.size();
            m_data.push_back(value);
            m_headers.push_back(header);
        } else {
            if(index < m_headers[header_index].start) {
                // insert header before current
                header.start = index;
                header.length = 1;
                header.index = m_headers[header_index].index;
                m_data.insert(m_data.begin() + m_headers[header_index].index, value);
                m_headers.insert(m_headers.begin() + header_index, header);

                for(size_t i = header_index + 1; i < m_headers.size(); ++i) {
                    m_headers[i].index++;
                }
            } else {
                size_t offset = index - m_headers[header_index].start;
                if(offset < m_headers[header_index].length) {
                    // update existing value
                    m_data[m_headers[header_index].index + offset] = value;
                } else {
                    // append value to current header
                    m_data.insert(m_data.begin() + (m_headers[header_index].index + offset), value);
                    m_headers[header_index].length += 1;

                    for(size_t i = header_index + 1; i < m_headers.size(); ++i) {
                        m_headers[i].index++;
                    }
                }
            }

            // join headers if ranges intersect
            if(header_index + 1 < m_headers.size()) {
                if(m_headers[header_index + 1].start == m_headers[header_index].start + m_headers[header_index].length) {
                    m_headers[header_index].length += m_headers[header_index + 1].length;
                    m_headers.erase(m_headers.begin() + (header_index + 1));
                }
            }
        }
    }

    void unset(size_t index)
    {
        // find first header with bounds less than index
        size_t header_index = 0;
        for(; header_index < m_headers.size() && index > (m_headers[header_index].start + m_headers[header_index].length - 1); ++header_index) { }

        Header header {0};

        if(header_index < m_headers.size() && index >= m_headers[header_index].start) {
            size_t offset = index - m_headers[header_index].start;
            size_t data_index = m_headers[header_index].index + offset;
            if(offset == 0) {
                // shift start of range
                m_headers[header_index].start++;
            } else if(offset < m_headers[header_index].length - 1) {
                // split header at index
                header.start = index + 1;
                header.length = m_headers[header_index].length - offset - 1;
                header.index = m_headers[header_index].index + offset + 1;
                m_headers[header_index].length = offset + 1;
                m_headers.insert(m_headers.begin() + (header_index + 1), header);
            }

            m_data.erase(m_data.begin() + data_index);
            m_headers[header_index].length--;

            for(size_t i = header_index + 1; i < m_headers.size(); ++i) {
                m_headers[i].index--;
            }

            if(m_headers[header_index].length == 0) {
                m_headers.erase(m_headers.begin() + header_index);
            }
        }
    }

    bool has(size_t index) const
    {
        return find_header(index) != m_headers.size();
    }

    T* get(size_t index)
    {
        size_t header_index = find_header(index);
        if(header_index < m_headers.size()) {
            size_t offset = index - m_headers[header_index].start;
            return &m_data[m_headers[header_index].index + offset];
        }
        return nullptr;
    }

    std::vector<size_t> indices() const
    {
        std::vector<size_t> result;
        for(size_t h = 0; h < m_headers.size(); ++h) {
            for(size_t i = 0; i < m_headers[h].length; ++i) {
                result.push_back(m_headers[h].start + i);
            }
        }
        return result;
    }

private:
    struct Header {
        size_t start;
        size_t length;
        size_t index;
    };

    size_t find_header(size_t index) const
    {
        size_t bound_lower = 0;
        size_t bound_upper = m_headers.size();
        size_t header_index = 0;

        while(bound_lower != bound_upper) {
            header_index = bound_lower + ((bound_upper - bound_lower) / 2);

            if(index >= m_headers[header_index].start && index < (m_headers[header_index].start + m_headers[header_index].length)) {
                return header_index;
            } else if(index < m_headers[header_index].start) {
                // update upper bound
                bound_upper = header_index;
            } else {
                // update lower bound
                bound_lower = header_index + 1;
            }
        }

        return m_headers.size();
    }

    size_t m_root;
    std::vector<Header> m_headers;
    std::vector<T> m_data;
};

#endif
