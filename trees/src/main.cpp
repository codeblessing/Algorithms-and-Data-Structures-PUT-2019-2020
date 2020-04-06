#include <iostream>
#include "bst.hpp"
#include "avl.hpp"

int main()
{
	const std::vector<int> data = {5, 6, 4, 7, 3, 8, 2, 9, 1, 10};
	auto bst_tree = bst::Tree(data);
	auto avl_tree = avl::Tree(data);

	std::cout << "bst::Tree in-order:" << std::endl;
	bst_tree.inorder();
	std::cout << "bst::Tree pre-order:" << std::endl;
	bst_tree.preorder();
	std::cout << "avl::Tree in-order:" << std::endl;
	avl_tree.inorder();
	std::cout << "avl::Tree pre-order:" << std::endl;
	avl_tree.preorder();
	std::cin.get();
    return 0;
}