#![allow(non_snake_case,non_camel_case_types,dead_code)]

use std::collections::HashMap;
use std::collections::HashSet;

/*
    Fill in the boggle function below. Use as many helpers as you want.
    Test your code by running 'cargo test' from the tester_rs_simple directory.
    
    To demonstrate how the HashMap can be used to store word/coord associations,
    the function stub below contains two sample words added from the 2x2 board.
*/

fn searchForWord(i: u8, j: u8, board: & [&str], word: &str, word_index: usize, mut points_vec: &mut Vec<(u8, u8)>, mut points_set: &mut HashSet<(u8, u8)>, board_size: u8) -> Vec<(u8, u8)>{
    if word_index == word.len(){
        return points_vec.clone();
    }

    let next_char = word.chars().nth(word_index);
    let next_index = word_index + 1;
    let i_index = i as usize;
    let j_index = j as usize;

    if i > 0 { // Top Left, Top, Top Right
        if (j > 0) && (!points_set.contains(&(i-1, j-1))) && (board[i_index-1].chars().nth(j_index-1) == next_char) { // Top Left
            points_vec.push((i-1, j-1));
            points_set.insert((i-1, j-1));
            let search_result = searchForWord(i-1, j-1, board, word, next_index, &mut points_vec, &mut points_set, board_size);
            if search_result.len() != 0 {
                return search_result
            } else{
                points_vec.pop();
                points_set.remove(&(i-1, j-1));
            }
        }
        if (!points_set.contains(&(i-1, j))) && (board[i_index-1].chars().nth(j_index) == next_char) { // Top
            points_vec.push((i-1, j));
            points_set.insert((i-1, j));
            let search_result = searchForWord(i-1, j, board, word, next_index, &mut points_vec, &mut points_set, board_size);
            if search_result.len() != 0 {
                return search_result
            } else{
                points_vec.pop();
                points_set.remove(&(i-1, j));
            }
        }
        if (j < board_size - 1) && (!points_set.contains(&(i-1, j+1))) && (board[i_index-1].chars().nth(j_index+1) == next_char) { // Top Right
            points_vec.push((i-1, j+1));
            points_set.insert((i-1, j+1));
            let search_result = searchForWord(i-1, j+1, board, word, next_index, &mut points_vec, &mut points_set, board_size);
            if search_result.len() != 0 {
                return search_result
            } else{
                points_vec.pop();
                points_set.remove(&(i-1, j+1));
            }
        }
    }

    if i < board_size - 1 { // Bottom Left, Bottom, Bottom Right
        if (j > 0) && (!points_set.contains(&(i+1, j-1))) && (board[i_index+1].chars().nth(j_index-1) == next_char) { // Bottom Left
            points_vec.push((i+1, j-1));
            points_set.insert((i+1, j-1));
            let search_result = searchForWord(i+1, j-1, board, word, next_index, &mut points_vec, &mut points_set, board_size);
            if search_result.len() != 0 {
                return search_result
            } else{
                points_vec.pop();
                points_set.remove(&(i+1, j-1));
            }
        }
        if (!points_set.contains(&(i+1, j))) && (board[i_index+1].chars().nth(j_index) == next_char) { // Bottom
            points_vec.push((i+1, j));
            points_set.insert((i+1, j));
            let search_result = searchForWord(i+1, j, board, word, next_index, &mut points_vec, &mut points_set, board_size);
            if search_result.len() != 0 {
                return search_result
            } else{
                points_vec.pop();
                points_set.remove(&(i+1, j));
            }
        }
        if (j < board_size - 1) && (!points_set.contains(&(i+1, j+1))) && (board[i_index+1].chars().nth(j_index+1) == next_char) { // Bottom Right
            points_vec.push((i+1, j+1));
            points_set.insert((i+1, j+1));
            let search_result = searchForWord(i+1, j+1, board, word, next_index, &mut points_vec, &mut points_set, board_size);
            if search_result.len() != 0 {
                return search_result
            } else{
                points_vec.pop();
                points_set.remove(&(i+1, j+1));
            }
        }
    }

    if (j > 0) && (!points_set.contains(&(i, j-1))) && (board[i_index].chars().nth(j_index-1) == next_char) { // Left
        points_vec.push((i, j-1));
        points_set.insert((i, j-1));
        let search_result = searchForWord(i, j-1, board, word, next_index, &mut points_vec, &mut points_set, board_size);
        if search_result.len() != 0 {
            return search_result
        } else{
            points_vec.pop();
            points_set.remove(&(i, j-1));
        }
    }

    if (j < board_size - 1) && (!points_set.contains(&(i, j+1))) && (board[i_index].chars().nth(j_index+1) == next_char) { // Right
        points_vec.push((i, j+1));
        points_set.insert((i, j+1));
        let search_result = searchForWord(i, j+1, board, word, next_index, &mut points_vec, &mut points_set, board_size);
        if search_result.len() != 0 {
            return search_result
        } else{
            points_vec.pop();
            points_set.remove(&(i, j+1));
        }
    }

    return Vec::new(); // No Match -> Return Empty Vector
}

fn boggle(board: & [&str], words: & Vec<String>) -> HashMap<String, Vec<(u8, u8)>>
{
    let board_size:u8 = board.len() as u8;
    let board_full_size:usize = (board.len() * board.len()) as usize;
    let mut word_dict: HashMap<char, Vec<String>> = HashMap::new();
    for i in 0..(words.len()) {
        if words[i].len() <= board_full_size {
            let curr_str = words[i].clone();
            if curr_str.is_empty() {
                continue; // Skip empty strings
            }
            let first_char = curr_str.chars().next().unwrap();
            word_dict.entry(first_char).or_insert(Vec::new()).push(curr_str);
        }
    }

    let mut found_dict: HashMap<String, Vec<(u8, u8)>> = HashMap::new();
    for i in 0..(board_size) {
        for j in 0..(board_size) {
            let curr_char = board[i as usize].chars().nth(j as usize).unwrap();
            if !word_dict.contains_key(&curr_char) {
                continue;
            }

            let dict_arr = word_dict.get(&curr_char).unwrap();
            let mut new_dict_arr = Vec::new();
            for word in dict_arr.iter()  {
                let word_len = word.len();
                let word_index: usize = 1;
                let mut points_vec = Vec::new();
                points_vec.push((i, j));
                let mut points_set: HashSet<(u8, u8)> = HashSet::new();
                points_set.insert((i, j));
                let search_result = searchForWord(i, j, board, word as &str, word_index, &mut points_vec, &mut points_set, board_size);
                if search_result.len() == word_len {
                    found_dict.insert(word.to_string(), search_result);
                } else{
                    new_dict_arr.push(word.to_string());
                }
            }
            word_dict.insert(curr_char, new_dict_arr);
        }
    }

    return found_dict;
}
    
#[cfg(test)]
#[path = "tests.rs"]
mod tests;