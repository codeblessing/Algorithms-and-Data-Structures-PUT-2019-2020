#include "bst.hpp"
#include <iostream>
#include <vector>

namespace bst
{
	Tree::Tree()
	{
		this->root_ = nullptr;
	}

	Tree::Tree(const int value)
	{
		this->root_ = new Node(value);
	}

	Tree::Tree(const std::vector<int>& values)
	{
		this->root_ = nullptr;
		for (auto key : values)
		{
			this->insert(key);		
		}
	}

	void Tree::insert(const int value)
	{
		Node* last = nullptr;
		auto current = this->root_;
		while (current != nullptr)
		{
			last = current;
			if (value < current->get_value())
			{
				current = current->get_left();
			}
			else
			{
				current = current->get_right();
			}
		}
		
		auto node = new Node(value);
		node->set_parent(last);

		if (last == nullptr)
		{
			this->root_ = node;
		}
		else if (value < last->get_value())
		{
			last->set_left(node);
		}
		else
		{
			last->set_right(node);
		}
	}

	void Tree::inorder(Node* node) const
	{
		if(!node)
			node = this->root_;
		if(!node)
			return;

		if (node->get_left())
			inorder(node->get_left());
		
		std::cout << node->get_value() << ' ';

		if (node->get_right())
			inorder(node->get_right());

		if(node == this->root_)
			std::cout << std::endl;
	}

	void Tree::preorder(Node* node) const
	{
		if(!node)
			node = this->root_;
		if(!node)
			return;
		
		std::cout << node->get_value() << ' ';

		if (node->get_left())
			preorder(node->get_left());

		if (node->get_right())
			preorder(node->get_right());

		if(node == this->root_)
			std::cout << std::endl;
	}

	Tree::Node* Tree::max(Node* node) const
	{
		if(!node)
			node = this->root_;
		if(!node)
			return nullptr;
		
		while (node->get_right() != nullptr)
		{
			std::cout << node->get_value() << ' ';
			node = node->get_right();
		}
		std::cout << node->get_value() << std::endl;
		return node;
	}

	Tree::Node* Tree::min(Node* node) const
	{
		if(!node)
			node = this->root_;
		if(!node)
			return nullptr;
		
		while (node->get_left() != nullptr)
		{
			std::cout << node->get_value() << ' ';
			node = node->get_left();
		}
		std::cout << node->get_value() << std::endl;
		return node;
	}

	void Tree::subtree_pre_walk(const int key) const
	{
		const auto node = find(key);
		if(!node)
			return;
		preorder(node);
		std::cout << std::endl;
	}

	void Tree::remove(const int key)
	{
		const auto removed = find(key);
		remove(removed);
	}

	void Tree::remove(Node* node)
	{
		if(!node)
			return;
		
		if (node->get_left() == nullptr)
		{
			transplant(node, node->get_right());
		}
		else if (node->get_right() == nullptr)
		{
			transplant(node, node->get_left());
		}
		else
		{
			auto new_node = min(node);
			if (new_node->get_parent() != node)
			{
				transplant(new_node, new_node->get_right());
				new_node->set_right(node->get_right());
				new_node->get_right()->set_parent(new_node);
			}
			transplant(node, new_node);
			new_node->set_left(node->get_left());
			new_node->get_left()->set_parent(new_node);
		}
		delete node;
		node = nullptr;
	}

	void Tree::remove_all(Node* node)
	{
		if (!node)
			node = this->root_;
		if(!node)
			return;
		
		if (node->get_left() != nullptr)
			remove_all(node->get_left());
		
		if (node->get_right() != nullptr)
			remove_all(node->get_right());
		
		remove(node);
	}

	Tree::Node* Tree::find(const int key) const
	{
		auto node = this->root_;
		while (node)
		{
			if (node->get_value() == key)
				return node;

			node = (node->get_value() > key) ? node->get_left() : node->get_right();
		}
		return node;
	}

	void Tree::transplant(Node* old_node, Node* new_node)
	{
		if(!old_node)
			return;
		
		if (old_node->get_parent() == nullptr)
		{
			this->root_ = new_node;
		}
		else if (old_node == old_node->get_parent()->get_left())
		{
			old_node->get_parent()->set_left(new_node);
		}
		else
		{
			old_node->get_parent()->set_right(new_node);
		}

		if (new_node)
		{
			new_node->set_parent(old_node->get_parent());
		}
	}

	Tree::~Tree()
	{
		remove_all();
	}

	Tree::Node::Node(const int value)
	{
		this->value_ = value;
		this->parent_ = nullptr;
		this->left_ = nullptr;
		this->right_ = nullptr;
	}

	int Tree::Node::get_value() const
	{
		return this->value_;
	}

	Tree::Node* Tree::Node::get_parent() const
	{
		return this->parent_;
	}

	Tree::Node* Tree::Node::get_right() const
	{
		return this->right_;
	}

	Tree::Node* Tree::Node::get_left() const
	{
		return this->left_;
	}

	void Tree::Node::set_value(const int value)
	{
		this->value_ = value;
	}

	void Tree::Node::set_parent(Node* node)
	{
		this->parent_ = node;
	}

	void Tree::Node::set_right(Node* node)
	{
		this->right_ = node;
	}

	void Tree::Node::set_left(Node* node)
	{
		this->left_ = node;
	}
} // namespace bst