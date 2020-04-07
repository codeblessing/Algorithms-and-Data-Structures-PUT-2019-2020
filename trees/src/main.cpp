#include <iostream>
#include <typeinfo>
#include "bst.hpp"
#include "avl.hpp"

int display_menu();

int main()
{
	int option = 0;
	auto bst_tree = bst::Tree();
	auto avl_tree = avl::Tree();
	
	forever
	{
		option = display_menu();
		//std::cout << "option: " << option << ", typeof(option): " << typeid(option).name() << std::endl;
		int num = 0, count = 0;
		switch (option)
		{
		case 1:
			num = 0;
			count = 0;
			std::cout << "Podaj ilosc liczb: ";
			std::cin >> count;
			std::cout << std::endl << "Podaj liczby:" << std::endl;
			for(int i = 0; i < count; i++)
			{
				std::cin >> num;
				bst_tree.insert(num);
			}
			break;
		case 2:
			num = 0;
			count = 0;
			std::cout << "Podaj ilosc kluczy: ";
			std::cin >> count;
			std::cout << std::endl << "Podaj klucze:" << std::endl;
			for(int i = 0; i < count; i++)
			{
				std::cin >> num;
				bst_tree.remove(num);
			}
			break;
		case 3:
			bst_tree.min();
			break;
		case 4:
			bst_tree.max();
			break;
		case 5:
			bst_tree.inorder();
			break;
		case 6:
			bst_tree.preorder();
			break;
		case 7:
			bst_tree.remove_all();
			break;
		case 8:
			num = 0;
			std::cout << "Podaj korzen poddrzewa: ";
			std::cin >> num;
			bst_tree.subtree_pre_walk(num);
			break;
		case 9:
			num = 0;
			count = 0;
			std::cout << "Podaj ilosc liczb: ";
			std::cin >> count;
			std::cout << std::endl << "Podaj liczby:" << std::endl;
			for(int i = 0; i < count; i++)
			{
				std::cin >> num;
				avl_tree.insert(num);
			}
			break;
		case 10:
			num = 0;
			count = 0;
			std::cout << "Podaj ilosc kluczy: ";
			std::cin >> count;
			std::cout << std::endl << "Podaj klucze:" << std::endl;
			for(int i = 0; i < count; i++)
			{
				std::cin >> num;
				avl_tree.remove(num);
			}
			break;
		case 11:
			avl_tree.min();
			break;
		case 12:
			avl_tree.max();
			break;
		case 13:
			avl_tree.inorder();
			break;
		case 14:
			avl_tree.preorder();
			break;
		case 15:
			avl_tree.remove_all();
			break;
		case 16:
			num = 0;
			std::cout << "Podaj korzen poddrzewa: ";
			std::cin >> num;
			avl_tree.subtree_pre_walk(num);
			break;
		default:
			break;
		}
		
		if(option == 0)
			break;
	}
	return 0;
}

int display_menu()
{
	int option = 0;
	std::cout << "Wybierz opcje:" << std::endl;
	std::cout << "[ 0] Wyjdz z programu." << std::endl;
	std::cout << "[ 1] Wstaw klucze do BST." << std::endl;
	std::cout << "[ 2] Usun klucze z BST." << std::endl;
	std::cout << "[ 3] Znajdz minimum w BST." << std::endl;
	std::cout << "[ 4] Znajdz maximum w BST." << std::endl;
	std::cout << "[ 5] Wypisz BST in-order." << std::endl;
	std::cout << "[ 6] Wypisz BST pre-order." << std::endl;
	std::cout << "[ 7] Usun cale BST." << std::endl;
	std::cout << "[ 8] Wypisz poddrzewo BST." << std::endl;
	std::cout << "[ 9] Wstaw klucze do drzewa AVL." << std::endl;
	std::cout << "[10] Usun klucze z drzewa AVL." << std::endl;
	std::cout << "[11] Znajdz minimum w drzewie AVL." << std::endl;
	std::cout << "[12] Znajdz maximum w drzewie AVL." << std::endl;
	std::cout << "[13] Wypisz drzewo AVL in-order." << std::endl;
	std::cout << "[14] Wypisz drzewo AVL pre-order." << std::endl;
	std::cout << "[15] Usun cale drzewo AVL." << std::endl;
	std::cout << "[16] Wypisz poddrzewo AVL." << std::endl;
	std::cin >> option;
	return option;
}