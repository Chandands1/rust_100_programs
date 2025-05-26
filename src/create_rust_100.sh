cd src

# Define names
file_names=(
"hello_world" "add_two_numbers" "swap_two_numbers" "odd_or_even_checker" "larger_of_two_numbers"
"simple_calculator" "fahrenheit_celsius" "kilometers_miles" "simple_interest" "area_of_circle"
"area_of_rectangle" "perimeter_of_square" "compound_interest" "grade_calculator" "leap_year_checker"
"factorial_loop" "factorial_recursion" "fibonacci_n_terms" "fibonacci_recursive" "sum_first_n"
"count_digits" "sum_of_digits" "reverse_integer" "palindrome_number" "prime_number_test"
"primes_in_range" "gcd_euclid" "lcm_using_gcd" "power" "multiplication_table"
"ascii_value" "print_alphabet" "vowel_or_consonant" "string_length" "reverse_string"
"palindrome_string" "count_vowels_consonants" "word_count" "largest_in_array" "smallest_in_array"
"array_sum_avg" "linear_search" "binary_search" "bubble_sort" "selection_sort"
"insertion_sort" "merge_two_arrays" "matrix_addition" "matrix_multiplication" "transpose_matrix"
"pattern_right_triangle" "pattern_pyramid" "pattern_inverted_pyramid" "star_diamond_pattern" "ascii_art_initials"
"simple_stopwatch" "random_guess" "coin_toss" "dice_roll" "cli_calculator"
"read_text_file" "write_text_file" "append_log_entry" "file_copy_utility" "read_csv"
"json_encode" "json_decode" "struct_point" "struct_display" "enum_traffic_light"
"enum_calculator" "vector_crud" "stack_vec" "queue_vecdeque" "hashmap_frequency"
"ownership_demo" "borrowing_demo" "lifetime_example" "simple_closure" "sort_with_closure"
"custom_iterator" "generic_swap" "result_handling" "option_handling" "thread_hello"
"threaded_counter" "channel_message" "simple_timer" "progress_bar" "logger_file"
"read_env" "unit_test_add" "benchmark_instant" "feature_flag" "const_demo"
"static_counter" "unsafe_pointer" "trait_object" "module_split" "cargo_build_run"
)

# Create folders and files
for i in {1..100}
do
    name=${file_names[$((i-1))]}
    folder_name=$(printf "%03d_%s" $i $name)
    mkdir "$folder_name"
    touch "$folder_name/$name.rs"
done
