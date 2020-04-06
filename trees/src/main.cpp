#include <iostream>
#include "bst.hpp"

int main()
{
	const std::vector<int> data = {5, 6, 4, 7, 3, 8, 2, 9, 1, 10};
	auto bst_tree = bst::Tree(data);
    for (auto key : data)
    {
	    bst_tree.insert(key);
    }

	std::cout << "Tree in-order:" << std::endl;
	bst_tree.inorder();
	std::cout << "Tree pre-order:" << std::endl;
	bst_tree.preorder();
	std::cin.get();
    return 0;
}