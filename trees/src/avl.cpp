#include "avl.hpp"
#include <iostream>
#include <algorithm>
#include <vector>

#define forever while(true)

namespace avl
{
	Tree::Tree()
	{
		this->_root = nullptr;
	}

	Tree::Tree(std::vector<int> values)
	{
		this->_root = nullptr;
		std::sort(values.begin(), values.end());
		for (auto i = values.begin() + (values.size() / 2); i != values.end(); ++i)
		{
			this->insert(*i);
		}
		for (auto i = values.begin(); i != (values.begin() + (values.size() / 2)); ++i)
		{
			this->insert(*i);
		}
	}

	Tree::Node* Tree::max(Node* node) const
	{
		if (node == nullptr)
			node = this->_root;

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
		if (node == nullptr)
			node = this->_root;

		while (node->get_left() != nullptr)
		{
			std::cout << node->get_value() << ' ';
			node = node->get_left();
		}
		std::cout << node->get_value() << std::endl;
		return node;
	}

	Tree::Node* Tree::find(const int key) const
	{
		auto node = this->_root;
		while (node != nullptr)
		{
			if (node->get_value() == key)
				return node;

			node = (node->get_value() > key) ? node->get_left() : node->get_right();
		}
		return node;
	}

	void Tree::insert(const int value)
	{
		if(!this->_root)
			this->_root = new Node(value);
		else
		{
			insert(this->_root, value);
		}
	}

	void Tree::inorder(Node* node) const
	{
		if (node == nullptr)
			node = this->_root;

		if (node->get_left() != nullptr)
			inorder(node->get_left());

		std::cout << node->get_value() << ' ';

		if (node->get_right() != nullptr)
			inorder(node->get_right());

		if (node == this->_root)
			std::cout << std::endl;
	}

	void Tree::preorder(Node* node) const
	{
		if (node == nullptr)
			node = this->_root;

		std::cout << node->get_value() << ' ';

		if (node->get_left() != nullptr)
			preorder(node->get_left());

		if (node->get_right() != nullptr)
			preorder(node->get_right());

		if (node == this->_root)
			std::cout << std::endl;
	}

	void Tree::subtree_pre_walk(const int key) const
	{
		const auto node = find(key);
		preorder(node);
	}

	void Tree::remove(const int key)
	{
		const auto removed = find(key);
		remove(removed);
	}

	void Tree::remove(Node* node)
	{
		auto parent = node->get_parent();
		if (node->get_left() == nullptr)
		{
			transplant(node, node->get_right());
			while (parent)
			{
				parent->set_height(1 + std::max(parent->get_left()->get_height(), parent->get_right()->get_height()));
				parent = parent->get_parent();
			}
		}
		else if (node->get_right() == nullptr)
		{
			transplant(node, node->get_left());
			while (parent)
			{
				parent->set_height(1 + std::max(parent->get_left()->get_height(), parent->get_right()->get_height()));
				parent = parent->get_parent();
			}
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

	void Tree::rr_rotation(Node* node)
	{
		auto right_son = node->get_right();
		auto successor = right_son->get_left();

		right_son->set_left(node);
		right_son->set_parent(node->get_parent());
		node->set_parent(right_son);
		node->set_right(successor);
		if(successor)
			successor->set_parent(node);

		node->set_height(1 + std::max(node->get_left()->get_height(), node->get_right()->get_height()));
		right_son->set_height(1 + std::max(right_son->get_left()->get_height(), right_son->get_right()->get_height()));
	}

	void Tree::ll_rotation(Node* node)
	{
		auto left_son = node->get_left();
		auto successor = left_son->get_right();

		left_son->set_right(node);
		left_son->set_parent(node->get_parent());
		node->set_parent(left_son);
		node->set_left(successor);
		if(successor)
			successor->set_parent(node);
		
		node->set_height(1 + std::max(node->get_left()->get_height(), node->get_right()->get_height()));
		left_son->set_height(1 + std::max(left_son->get_right()->get_height(), left_son->get_left()->get_height()));
	}

	void Tree::rl_rotation(Node* node)
	{
		ll_rotation(node);
		rr_rotation(node);
	}

	void Tree::lr_rotation(Node* node)
	{
		rr_rotation(node);
		ll_rotation(node);
	}

	void Tree::remove_all(Node* node)
	{
		if (node == nullptr)
			node = this->_root;

		if (node->get_left() != nullptr)
			remove_all(node->get_left());

		if (node->get_right() != nullptr)
			remove_all(node->get_right());

		remove(node);
	}

	void Tree::transplant(Node* old_node, Node* new_node)
	{
		if (old_node->get_parent() == nullptr)
		{
			this->_root = new_node;
		}
		else if (old_node == old_node->get_parent()->get_left())
		{
			old_node->get_parent()->set_left(new_node);
		}
		else
		{
			old_node->get_parent()->set_right(new_node);
		}

		if (new_node != nullptr)
		{
			new_node->set_parent(old_node->get_parent());
		}
	}

	void Tree::insert(Node* node, const int value)
	{
		if(node->get_value() > value)
		{
			if(node->get_left())
				insert(node->get_left(), value);
			else
			{
				auto new_node = new Node(value);
				new_node->set_parent(node);
				node->set_left(new_node);
			}
		}
		else
		{
			if(node->get_right())
				insert(node->get_right(), value);
			else
			{
				auto new_node = new Node(value);
				new_node->set_parent(node);
				node->set_right(new_node);
			}
		}

		node->set_height(1 + std::max(node->get_right()->get_height(), node->get_left()->get_height()));
		
		if(node->get_balance_factor() > 1)
		{
			node->get_left()->get_value() > value ? ll_rotation(node) : lr_rotation(node);
		}
		else if(node->get_balance_factor() < -1)
		{
			node->get_right()->get_value() < value ? rr_rotation(node) : rl_rotation(node);
		}
	}

	Tree::~Tree()
	{
		delete this->_root;
		this->_root = nullptr;
	}

	Tree::Node::Node(const int value)
	{
		this->_value = value;
		this->_height = 1;
		this->_parent = nullptr;
		this->_left = nullptr;
		this->_right = nullptr;
	}

	auto Tree::Node::get_balance_factor() const -> int
	{
		return static_cast<int>(this->get_left()->get_height() - this->get_right()->get_height());
	}

	int Tree::Node::get_value() const
	{
		return this->_value;
	}

	Tree::Node* Tree::Node::get_parent() const
	{
		return this->_parent;
	}

	Tree::Node* Tree::Node::get_right() const
	{
		return this->_right;
	}

	Tree::Node* Tree::Node::get_left() const
	{
		return this->_left;
	}

	auto Tree::Node::get_height() const -> int
	{
		return this->_height;
	}

	/*void Tree::Node::set_balance_factor(const int value)
	{
		this->balance_factor_ = value;
	}*/

	void Tree::Node::set_value(const int value)
	{
		this->_value = value;
	}

	void Tree::Node::set_parent(Node* node)
	{
		this->_parent = node;
	}

	void Tree::Node::set_right(Node* node)
	{
		this->_right = node;
	}

	void Tree::Node::set_left(Node* node)
	{
		this->_left = node;
	}

	void Tree::Node::set_height(const int value)
	{
		this->_height = value;
	}
} // namespace avl