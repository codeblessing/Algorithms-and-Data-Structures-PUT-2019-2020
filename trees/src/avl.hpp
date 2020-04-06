#ifndef _ADELSON_VELSKY_LANDIS_TREE_HPP_
#define _ADELSON_VELSKY_LANDIS_TREE_HPP_
#include <vector>

namespace avl
{
	class Tree
	{
		class Node;
	public:
		Tree();
		explicit Tree(std::vector<int> values);
		~Tree();
		auto min(Node* node = nullptr) const->Node*;
		auto max(Node* node = nullptr) const->Node*;
		void insert(int value);
		void remove(int key);
		void remove_all(Node* node = nullptr);
		void inorder(Node* node = nullptr) const;
		void preorder(Node* node = nullptr) const;
		void subtree_pre_walk(int key) const;
	private:
		class Node
		{
		public:
			explicit Node(int value);
			auto get_balance_factor() const -> int;
			auto get_value() const -> int;
			auto get_parent() const->Node*;
			auto get_right() const->Node*;
			auto get_left() const->Node*;
			auto get_height() const -> int;
			//void set_balance_factor(int value);
			void set_value(int value);
			void set_parent(Node* node);
			void set_right(Node* node);
			void set_left(Node* node);
			void set_height(int value);

		private:
			int _value;
			int _height;
			Node* _parent;
			Node* _right;
			Node* _left;
		};
		auto find(int key) const->Node*;
		void transplant(Node* old_node, Node* new_node);
		void remove(Node* node);
		void insert(Node* node, int value);
		void rr_rotation(Node* node);
		void ll_rotation(Node* node);
		void rl_rotation(Node* node);
		void lr_rotation(Node* node);
		Node* _root;
	};
} // namespace avl
#endif //_ADELSON_VELSKY_LANDIS_TREE_HPP_