#![allow(dead_code)]
mod a_test;
mod acm_icpc_team;
mod angry_prof;
mod arithmetic_slices;
mod array_to_bst;
mod asteroid_collision;
mod available_captures;
mod base_neg2;
mod baseball_game_solution;
mod basic_calc_ii;
mod binary_gap;
mod binary_search;
mod binary_strings;
mod bst_iterator;
mod can_make_palindrome_from_substring;
mod can_place_flowers;
mod car_pooling;
mod climbing_stairs;
mod coin_change;
mod comb_sum_iv;
mod combination_sum;
mod combination_sum_iii;
mod combinations;
mod combinations_ii;
mod compare_strings_by_freq_of_smallest_char;
mod counting_bits;
mod daily_temperatures;
mod delete_columns;
mod dep_tree;
mod different_ways_parentheses;
mod directionreduction;
mod distance_between_bus_stops;
mod duplicate_zeroes;
mod find_all_anagrams;
mod find_common_chars;
mod find_difference;
mod find_duplicates;
mod find_the_duplicate;
mod first_unique_char_in_string;
mod goat_latin;
mod hamming_distance;
mod how_many_numbers_iii;
mod insert_bst;
mod intersection_of_two_arrays_ii;
mod is_subsequence;
mod jewels_and_stones;
mod judge_circle;
mod jumping_on_clouds;
mod k_weakest_rows;
mod l4sum;
mod largest_triangle_area;
mod leaf_similar_trees;
mod letter_case_permutation;
mod longest_common_subsequence;
mod majority_elements;
mod max_consec_ones;
mod max_len_repeated_arr;
mod maximum_binary_tree;
mod maximum_sliding_window;
mod min_ascii_delete_sum;
mod min_cost_tickets;
mod min_index_sum_two_lists;
mod minimum_add_to_make_parens_valid;
mod minimum_size_subarray_sum;
mod minimum_window_substring;
mod missing_number;
mod my_atoi;
mod n_queens;
mod next_greater_element;
mod num_unique_emails;
mod number_1_bits;
mod number_of_atoms;
mod number_of_recent_calls;
mod partition_array_into_three_pairs_equal_sum;
mod permutation_in_string;
mod permutations;
mod permutations_ii;
mod phantom_test;
mod plus_one;
mod prize_draw;
mod product_of_array;
mod rabin_karp;
mod ransom_note;
mod relative_sorted_array;
mod remove_all_adjacent_dupes_in_string_ii;
mod remove_all_duplicates_in_string;
mod reshape_the_matrix;
mod reverse_integer;
mod reverse_only_letters;
mod rotate_string;
mod same_tree;
mod scanner;
mod search_a_2d_matrix;
mod shortest_unsorted_cont_subarr;
mod simple_text_editor;
mod single_number;
mod sock_merchant;
mod sort_char_by_frequency;
mod square_digits;
mod strsplit;
mod sub_k_diff;
mod sum_of_distances;
mod top_k_freq;
mod top_k_freq_words;
mod tribonacci;
mod two_city_sched;
mod uncommon_from_sentences;
mod unique_morse_code;
mod univalued_binary_tree;
mod valid_anagram;
mod valid_tic_tac_toe;

use std::{cell::RefCell, rc::Rc};
// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
