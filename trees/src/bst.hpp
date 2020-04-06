#ifndef _BINARY_SEARCH_TREE_HPP_
#define _BINARY_SEARCH_TREE_HPP_
#include <vector>

namespace bst
{
	class Tree
	{
		class Node;

	public:
		Tree();
		explicit Tree(int value);
		explicit Tree(const std::vector<int>& values);
		auto min(Node* node = nullptr) const->Node*;
		auto max(Node* node = nullptr) const->Node*;
		void insert(int value);
		void remove(int key);
		void remove_all(Node* node = nullptr);
		void inorder(Node* node = nullptr) const;
		void preorder(Node* node = nullptr) const;
		void subtree_pre_walk(int key) const;
		~Tree();

	private:
		void transplant(Node* old_node, Node* new_node);
		void remove(Node* node);
		auto find(int key) const->Node*;
		Node* root_;
		class Node
		{
		public:
			explicit Node(int value);
			auto get_value() const -> int;
			auto get_parent() const -> Node*;
			auto get_right() const -> Node*;
			auto get_left() const -> Node*;
			void set_value(int value);
			void set_parent(Node* node);
			void set_right(Node* node);
			void set_left(Node* node);

		private:
			int value_;
			Node* parent_;
			Node* right_;
			Node* left_;
		};
	};
} // namespace bst
#endif //_BINARY_SEARCH_TEE_HPP_