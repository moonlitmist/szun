#ifndef H_TYPETREE
#define H_TYPETREE

#include <cstdint>
#include <vector>

class TypeTree {
public:
    struct Node {
        size_t key;
        size_t parent;
        std::vector<size_t> children;
        
        Node(size_t key, size_t parent);
    };

    TypeTree();
    ~TypeTree();

    bool has(size_t id);

    size_t outer(size_t id, size_t type);
    size_t key(size_t id);
    size_t inner(size_t id);

private:
    std::vector<Node> m_nodes;
};

TypeTree::Node::Node(size_t key, size_t parent) :key(key), parent(parent) { }

TypeTree::TypeTree()
{
    m_nodes.push_back(Node(0, 0));
}

TypeTree::~TypeTree()
{ }

bool TypeTree::has(size_t id)
{
    return id < m_nodes.size();
}

size_t TypeTree::outer(size_t id, size_t key)
{
    size_t find = 0;
    if(key != 0 && id < m_nodes.size()) {
        Node& node = m_nodes[id];
        for(size_t child : node.children) {
            if(m_nodes[child].key == key) {
                find = child;
                break;
            }
        }

        if(find == 0) {
            m_nodes.push_back(Node(key, id));
            find = m_nodes.size() - 1;
            m_nodes[id].children.push_back(find);
        }
    }
    return find;
}

size_t TypeTree::key(size_t id)
{
    if(id > 0 && id < m_nodes.size()) {
        return m_nodes[id].key;
    }
    return 0;
}

size_t TypeTree::inner(size_t id)
{
    if(id > 0 && id < m_nodes.size()) {
        return m_nodes[id].parent;
    }
    return 0;
}

#endif
