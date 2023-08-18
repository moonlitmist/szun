#ifndef H_NameTree
#define H_NameTree

#include <cstdint>
#include <vector>
#include <string>

class NameTree {
public:
    struct Node {
        std::string prefix;
        size_t parent;
        std::vector<size_t> children;
        
        Node(const std::string& prefix, size_t parent);
    };

    NameTree();
    ~NameTree();

    size_t indexof(const std::string& value);
    std::string keyof(size_t) const;

private:
    std::vector<Node> m_nodes;
};

NameTree::Node::Node(const std::string& prefix, size_t parent)
    :prefix(prefix), parent(parent) { }

NameTree::NameTree()
{
    m_nodes.push_back(Node("", 0));
}

NameTree::~NameTree()
{ }

size_t NameTree::indexof(const std::string& key)
{
    size_t current = 0;
    size_t index = 0;

    // descend node tree until key is found
    while(index < key.length()) {
        Node& node = m_nodes[current];

        // search child nodes for first character match
        bool find = false;
        for(size_t list_index = 0; list_index < node.children.size(); ++list_index) {
            size_t child_index = node.children[list_index];
            Node& child = m_nodes[child_index];

            // match extent of key to child prefix
            if(child.prefix[0] == key[index]) {
                index++;
                find = true;

                size_t prefix_index = 1;
                for(; prefix_index < child.prefix.length() && index < key.length(); prefix_index++ && index++) {
                    // check whether prefixes are equal
                    if(child.prefix[prefix_index] != key[index]) { break; }
                }

                // if prefix matches, continue to child node
                if(prefix_index == child.prefix.length()) {
                    current = child_index;
                    break;
                }

                // otherwise, branch prefix and create node for key
                else {
                    // create intermediate node, retaining node index of original
                    m_nodes.push_back(Node(m_nodes[child_index].prefix.substr(0, prefix_index), current));
                    size_t intermediate_index = m_nodes.size() - 1;
                    m_nodes[current].children[list_index] = intermediate_index;
                    m_nodes[intermediate_index].children.push_back(child_index);
                    
                    // update child node
                    m_nodes[child_index].prefix = m_nodes[child_index].prefix.substr(prefix_index);
                    m_nodes[child_index].parent = intermediate_index;

                    // create branching node if value is not prefix
                    if(index != key.length()) {
                        m_nodes.push_back(Node(key.substr(index), intermediate_index));
                    }

                    return m_nodes.size() - 1;
                }
            }
        }
        
        // if first character is not found, create new child
        if(!find) {
            m_nodes.push_back(Node(key.substr(index), current));
            m_nodes[current].children.push_back(m_nodes.size() - 1);
            return m_nodes.size() - 1;
        }
    }

    return current;
}

std::string NameTree::keyof(size_t id) const
{
    size_t length = 0;
    std::string out;

    if(id < m_nodes.size()) {
        for(size_t current = id; current != 0;) {
            length += m_nodes[current].prefix.length();
            current = m_nodes[current].parent;
        }

        out = std::string(length, 0);
        size_t index = length - 1;

        for(size_t current = id; current != 0;) {
            size_t prefix_length = m_nodes[current].prefix.length() - 1;
            for(size_t i = 0; i < m_nodes[current].prefix.length(); ++i) {
                out[index--] = m_nodes[current].prefix[prefix_length - i];
            }
            current = m_nodes[current].parent;
        }
    }

    return out;
}

#endif
