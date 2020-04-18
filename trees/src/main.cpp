#include <iostream>
#include <chrono>
#include <random>
#include <algorithm>
#include <functional>
#include <fstream>
#include <sstream>
#include "bst.hpp"
#include "avl.hpp"

namespace chrono = std::chrono;

int display_menu();

int main()
{
	//int option = 0;
	//auto bst_tree = bst::Tree();
	//auto avl_tree = avl::Tree();
	//
	//forever
	//{
	//	option = display_menu();
	//	//std::cout << "option: " << option << ", typeof(option): " << typeid(option).name() << std::endl;
	//	int num = 0, count = 0;
	//	switch (option)
	//	{
	//	case 1:
	//		num = 0;
	//		count = 0;
	//		std::cout << "Podaj ilosc liczb: ";
	//		std::cin >> count;
	//		std::cout << std::endl << "Podaj liczby:" << std::endl;
	//		for(int i = 0; i < count; i++)
	//		{
	//			std::cin >> num;
	//			bst_tree.insert(num);
	//		}
	//		break;
	//	case 2:
	//		num = 0;
	//		count = 0;
	//		std::cout << "Podaj ilosc kluczy: ";
	//		std::cin >> count;
	//		std::cout << std::endl << "Podaj klucze:" << std::endl;
	//		for(int i = 0; i < count; i++)
	//		{
	//			std::cin >> num;
	//			bst_tree.remove(num);
	//		}
	//		break;
	//	case 3:
	//		bst_tree.min();
	//		break;
	//	case 4:
	//		bst_tree.max();
	//		break;
	//	case 5:
	//		bst_tree.inorder();
	//		break;
	//	case 6:
	//		bst_tree.preorder();
	//		break;
	//	case 7:
	//		bst_tree.remove_all();
	//		break;
	//	case 8:
	//		num = 0;
	//		std::cout << "Podaj korzen poddrzewa: ";
	//		std::cin >> num;
	//		bst_tree.subtree_pre_walk(num);
	//		break;
	//	case 9:
	//		num = 0;
	//		count = 0;
	//		std::cout << "Podaj ilosc liczb: ";
	//		std::cin >> count;
	//		std::cout << std::endl << "Podaj liczby:" << std::endl;
	//		for(int i = 0; i < count; i++)
	//		{
	//			std::cin >> num;
	//			avl_tree.insert(num);
	//		}
	//		break;
	//	case 10:
	//		num = 0;
	//		count = 0;
	//		std::cout << "Podaj ilosc kluczy: ";
	//		std::cin >> count;
	//		std::cout << std::endl << "Podaj klucze:" << std::endl;
	//		for(int i = 0; i < count; i++)
	//		{
	//			std::cin >> num;
	//			avl_tree.remove(num);
	//		}
	//		break;
	//	case 11:
	//		avl_tree.min();
	//		break;
	//	case 12:
	//		avl_tree.max();
	//		break;
	//	case 13:
	//		avl_tree.inorder();
	//		break;
	//	case 14:
	//		avl_tree.preorder();
	//		break;
	//	case 15:
	//		avl_tree.remove_all();
	//		break;
	//	case 16:
	//		num = 0;
	//		std::cout << "Podaj korzen poddrzewa: ";
	//		std::cin >> num;
	//		avl_tree.subtree_pre_walk(num);
	//		break;
	//	default:
	//		break;
	//	}
	//	
	//	if(option == 0)
	//		break;
	//}

	std::ofstream results("./results.txt", std::ios::out | std::ios::app);
	auto times = std::stringstream(std::ios::in | std::ios::out);

	std::vector<int> values(42'000);
	auto random = std::bind(std::uniform_int_distribution<int>(-1'000'000, 1'000'000), std::mt19937(chrono::high_resolution_clock::now().time_since_epoch().count()));

	auto average_bst_build_time = 0LL;
	auto average_bst_min_search_time = 0LL;
	auto average_bst_inorder_walk_time = 0LL;
	auto average_avl_build_time = 0LL;
	auto average_avl_min_search_time = 0LL;
	auto average_avl_inorder_walk_time = 0LL;

	for (int i = 0; i < 10; i++) {
		std::generate(values.begin(), values.end(), random);
		std::sort(values.begin(), values.end(), [](const int first, const int second) -> bool { return (first - second) > 0; });

		const auto bst_build_start = chrono::high_resolution_clock::now();
		const auto bst_tree = bst::Tree(values);
		const auto bst_build_end = chrono::high_resolution_clock::now();
		const auto bst_build_time = chrono::duration_cast<chrono::nanoseconds>(bst_build_end - bst_build_start);
		average_bst_build_time += bst_build_time.count();

		const auto bst_min_search_start = chrono::high_resolution_clock::now();
		auto min = bst_tree.min();
		const auto bst_min_search_end = chrono::high_resolution_clock::now();
		auto bst_min_search_time = chrono::duration_cast<chrono::nanoseconds>(bst_min_search_end - bst_min_search_start);
		average_bst_min_search_time += bst_min_search_time.count();

		const auto bst_inorder_start = chrono::high_resolution_clock::now();
		bst_tree.inorder();
		const auto bst_inorder_end = chrono::high_resolution_clock::now();
		auto bst_inorder_time = chrono::duration_cast<chrono::nanoseconds>(bst_inorder_end - bst_inorder_start);
		average_bst_inorder_walk_time += bst_inorder_time.count();

		const auto avl_build_start = chrono::high_resolution_clock::now();
		const auto avl_tree = avl::Tree(values);
		const auto avl_build_end = chrono::high_resolution_clock::now();
		auto avl_build_time = chrono::duration_cast<chrono::nanoseconds>(avl_build_end - avl_build_start);
		average_avl_build_time += avl_build_time.count();

		const auto avl_min_search_start = chrono::high_resolution_clock::now();
		auto tmp = avl_tree.min();
		const auto avl_min_search_end = chrono::high_resolution_clock::now();
		auto avl_min_search_time = chrono::duration_cast<chrono::nanoseconds>(avl_min_search_end - avl_min_search_start);
		average_avl_min_search_time += avl_min_search_time.count();

		const auto avl_inorder_start = chrono::high_resolution_clock::now();
		avl_tree.inorder();
		const auto avl_inorder_end = chrono::high_resolution_clock::now();
		auto avl_inorder_time = chrono::duration_cast<chrono::nanoseconds>(avl_inorder_end - avl_inorder_start);
		average_avl_inorder_walk_time += avl_inorder_time.count();
	}

	times << "BST\n" << average_bst_build_time / 10 << '\n'
		<< average_bst_min_search_time / 10 << '\n'
		<< average_bst_inorder_walk_time / 10 << "\n\n"
		<< "AVL\n" << average_avl_build_time / 10 << '\n'
		<< average_avl_min_search_time / 10 << '\n'
		<< average_avl_inorder_walk_time / 10 << "\n\n";

	results << times.str();

	results.close();

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