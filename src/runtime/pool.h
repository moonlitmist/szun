#ifndef H_POOL
#define H_POOL

template <typename T>
class Pool {
public:
    Pool()
    {
        m_next = 0;
    }
    
    ~Pool()
    { }

    size_t add(const T& data)
    {
        size_t location = m_next;

        if(m_next == m_nodes.size()) {
            m_nodes.push_back(Node());
            m_next++;

            m_nodes[location].active = true;
            m_nodes[location].data.value = data;
        }
        else {
            m_nodes[location].active = true;
            m_next = m_nodes[location].data.addr;
            m_nodes[location].data.value = data;
        }

        return location + 1;
    }

    void remove(size_t id)
    {
        if(has(id)) {
            size_t index = id - 1;
            m_nodes[index].active = false;
            m_nodes[index].data.addr = m_next;
            m_next = index;
        }
    }

    bool has(size_t id)
    {
        if(id > 0 && id <= m_nodes.size()) {
            if(m_nodes[id - 1].active) {
                return true;
            }
        }
        return false;
    }

    bool get(size_t id, T& data)
    {
        if(has(id)) {
            data = m_nodes[id - 1].data.value;
            return true;
        }
        return false;
    }

private:
    struct Node {
        bool active;
        union {
            T value;
            size_t addr;
        } data;

        Node() {
            active = false;
            data.addr = 0;
        }
    };
    std::vector<Node> m_nodes;
    size_t m_next;
};

#endif
