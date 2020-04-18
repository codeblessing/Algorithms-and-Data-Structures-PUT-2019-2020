#include "avl.hpp"
#include <iostream>
#include <algorithm>
#include <vector>

namespace avl
{
	Tree::Tree()
	{
		this->_root = nullptr;
	}

	Tree::Tree(const std::vector<int>& values)
	{
		this->_root = nullptr;
		
		for (auto key : values)
		{
			insert(key);
		}
	}

	Tree::Node* Tree::max(Node* node) const
	{
		if (!node)
			node = this->_root;
		
		if (!node)
			return nullptr;

		while (node->get_right())
		{
			node = node->get_right();
		}
		
		return node;
	}

	Tree::Node* Tree::min(Node* node) const
	{
		if (!node)
			node = this->_root;
		
		if (!node)
			return nullptr;

		while (node->get_left())
		{
			node = node->get_left();
		}
		
		return node;
	}

	Tree::Node* Tree::find(const int key) const
	{
		auto node = this->_root;
		
		while (node)
		{
			if (node->get_value() == key)
				return node;

			node = (node->get_value() > key) ? node->get_left() : node->get_right();
		}
		
		return node;
	}

	void Tree::insert(const int value)
	{
		if (!this->_root)
			this->_root = new Node(value);
		else
		{
			insert(this->_root, value);
		}
	}

	void Tree::inorder(Node* node) const
	{
		if (!node)
			node = this->_root;
		
		if (!node)
			return;

		if (node->get_left())
			inorder(node->get_left());

		std::cout << node->get_value() << ' ';

		if (node->get_right())
			inorder(node->get_right());
	}

	void Tree::preorder(Node* node) const
	{
		if (!node)
			node = this->_root;
		
		if (!node)
			return;

		std::cout << node->get_value() << ' ';

		if (node->get_left())
			preorder(node->get_left());

		if (node->get_right())
			preorder(node->get_right());
	}

	void Tree::subtree_pre_walk(const int key) const
	{
		const auto node = find(key);
		
		if (!node)
			return;
		
		preorder(node);
		
		std::cout << std::endl;
	}

	void Tree::remove(const int key)
	{
		const auto removed = find(key);
		//std::cout << "removed key: " << removed->get_value() << ", it's parent: " << removed->get_parent()->get_value() << std::endl;
		remove(removed);
	}
	
	void Tree::remove_unbalanced(Node* node)
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

	// ReSharper disable once CppMemberFunctionMayBeConst
	void Tree::update_height(Node* parent)
	{
		auto left_height = 0, right_height = 0;
		while (parent)
		{
			left_height = parent->get_left() ? parent->get_left()->get_height() : 0;
			right_height = parent->get_right() ? parent->get_right()->get_height() : 0;
			parent->set_height(1 + std::max(left_height, right_height));
			//std::cerr << "Updating tree height." << std::endl;
			//std::cerr << "Parent value: " << parent->get_value() << std::endl;
			parent = parent->get_parent();
		}
		//std::cerr << "Tree height updated." << std::endl;
	}

	void Tree::balance_tree(Node* node)
	{
		while (node)
		{
			if (node->get_balance_factor() > 1)
			{
				node->get_left()->get_balance_factor() > 0 ? ll_rotation(node) : lr_rotation(node);
			}
			else if (node->get_balance_factor() < -1)
			{
				node->get_right()->get_balance_factor() < 0 ? rr_rotation(node) : rl_rotation(node);
			}
			//std::cerr << "Balancing tree." << std::endl;
			//std::cerr << "Node value: " << node->get_value() << std::endl;
			node = node->get_parent();
		}
		//std::cerr << "Tree balanced." << std::endl;
	}

	void Tree::remove(Node* node)
	{
		if (!node)
			return;

		const auto parent = node->get_parent();
		if (!node->get_left())
		{
			//std::cout << "Removing node without left son" << std::endl;
			transplant(node, node->get_right());
			update_height(node->get_right());
			balance_tree(parent);
		}
		else if (!node->get_right())
		{
			//std::cout << "Removing node without right son" << std::endl;
			transplant(node, node->get_left());
			update_height(parent);
			balance_tree(parent);
		}
		else
		{
			auto new_parent = min(node->get_right());
			const auto np_right_son = new_parent->get_right();
			if (new_parent->get_parent() != node)
				new_parent->get_parent()->set_left(np_right_son);
			else
				new_parent->set_left(np_right_son);
			new_parent->set_parent(node->get_parent());
			new_parent->set_left(node->get_left());
			if (new_parent != node->get_right())
			{
				new_parent->set_right(node->get_right());
				if (new_parent->get_right())
					new_parent->get_right()->set_parent(new_parent);
			}
			if (new_parent->get_parent())
			{
				if (new_parent->get_parent()->get_left() == node)
					new_parent->get_parent()->set_left(new_parent);
				else
					new_parent->get_parent()->set_right(new_parent);
			}
			if (new_parent->get_left())
				new_parent->get_left()->set_parent(new_parent);

			const auto changed = min(node->get_right());
			//std::cerr << "Before updating the height." << std::endl;
			update_height(changed);
			//std::cerr << "Before balancing the tree." << std::endl;
			balance_tree(changed);
		}
		delete node;
		node = nullptr;
	}

	// ReSharper disable once CppMemberFunctionMayBeConst
	void Tree::rr_rotation(Node* node)
	{
		if (!node || !node->get_right())
			return;

		auto new_parent = node->get_right();
		auto np_left_son = new_parent->get_left();

		new_parent->set_left(node);
		new_parent->set_parent(node->get_parent());
		
		if (node->get_parent())
			node->get_parent()->get_right() == node ? node->get_parent()->set_right(new_parent) : node->get_parent()->set_left(new_parent);
		else
			this->_root = new_parent;
		
		node->set_parent(new_parent);
		node->set_right(np_left_son);
		
		if (np_left_son)
			np_left_son->set_parent(node);

		update_height(node);
	}

	// ReSharper disable once CppMemberFunctionMayBeConst
	void Tree::ll_rotation(Node* node)
	{
		if (!node || !node->get_left())
			return;

		auto new_parent = node->get_left();
		auto np_right_son = new_parent->get_right();

		new_parent->set_right(node);
		new_parent->set_parent(node->get_parent());
		
		if (node->get_parent())
			node->get_parent()->get_left() == node ? node->get_parent()->set_left(new_parent) : node->get_parent()->set_right(new_parent);
		else
			this->_root = new_parent;
		
		node->set_parent(new_parent);
		node->set_left(np_right_son);
		
		if (np_right_son)
			np_right_son->set_parent(node);

		update_height(node);
	}

	void Tree::rl_rotation(Node* node)
	{
		if (!node || node->get_right())
			return;

		// ll_rotation
		
		auto rnode = node->get_right();

		if (!rnode || !rnode->get_left())
			return;

		auto new_parent = rnode->get_left();
		auto np_right_son = new_parent->get_right();

		new_parent->set_right(rnode);
		new_parent->set_parent(rnode->get_parent());
		
		if (rnode->get_parent())
			rnode->get_parent()->get_left() == rnode ? rnode->get_parent()->set_left(new_parent) : rnode->get_parent()->set_right(new_parent);
		else
			this->_root = new_parent;
		
		rnode->set_parent(new_parent);
		rnode->set_left(np_right_son);
		
		if (np_right_son)
			np_right_son->set_parent(rnode);

		// rr_rotation

		if (!node || !node->get_right())
			return;

		new_parent = node->get_right();
		auto np_left_son = new_parent->get_left();

		new_parent->set_left(node);
		new_parent->set_parent(node->get_parent());
		
		if (node->get_parent())
			node->get_parent()->get_right() == node ? node->get_parent()->set_right(new_parent) : node->get_parent()->set_left(new_parent);
		else
			this->_root = new_parent;
		
		node->set_parent(new_parent);
		node->set_right(np_left_son);
		
		if (np_left_son)
			np_left_son->set_parent(node);
		
		update_height(node);
	}

	void Tree::lr_rotation(Node* node)
	{
		if (!node || !node->get_left())
			return;

		// rr_rotation
		
		auto lnode = node->get_left();
		if (!lnode || !lnode->get_right())
			return;

		auto new_parent = lnode->get_right();
		auto np_left_son = new_parent->get_left();

		new_parent->set_left(lnode);
		new_parent->set_parent(lnode->get_parent());
		
		if (lnode->get_parent())
			lnode->get_parent()->get_right() == lnode ? lnode->get_parent()->set_right(new_parent) : lnode->get_parent()->set_left(new_parent);
		else
			this->_root = new_parent;
		
		lnode->set_parent(new_parent);
		lnode->set_right(np_left_son);
		
		if (np_left_son)
			np_left_son->set_parent(lnode);

		// ll_rotation

		if (!node || !node->get_left())
			return;

		new_parent = node->get_left();
		auto np_right_son = new_parent->get_right();

		new_parent->set_right(node);
		new_parent->set_parent(node->get_parent());
		
		if (node->get_parent())
			node->get_parent()->get_left() == node ? node->get_parent()->set_left(new_parent) : node->get_parent()->set_right(new_parent);
		else
			this->_root = new_parent;
		
		node->set_parent(new_parent);
		node->set_left(np_right_son);
		
		if (np_right_son)
			np_right_son->set_parent(node);

		update_height(node);	}

	void Tree::remove_all(Node* node)
	{
		if (!node)
			node = this->_root;
		if (!node)
			return;

		if (node->get_left())
			remove_all(node->get_left());

		if (node->get_right())
			remove_all(node->get_right());

		remove_unbalanced(node);
	}

	void Tree::transplant(Node* old_node, Node* new_node)
	{
		if (!old_node)
			return;

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
		if (node->get_value() > value)
		{
			if (node->get_left())
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
			if (node->get_right())
				insert(node->get_right(), value);
			else
			{
				auto new_node = new Node(value);
				new_node->set_parent(node);
				node->set_right(new_node);
			}
		}

		update_height(node);

		if (node->get_balance_factor() > 1)
		{
			node->get_left()->get_value() > value ? ll_rotation(node) : lr_rotation(node);
		}
		else if (node->get_balance_factor() < -1)
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
		return (this->get_left() ? this->get_left()->get_height() : 0) - (this->get_right() ? this->get_right()->get_height() : 0);
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