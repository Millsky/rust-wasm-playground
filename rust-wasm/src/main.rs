fn main() {
    fn is_anagram(s1: String, s2: String) -> bool {
        let sorted_string: String = sort(s1);
        let sorted_string_2: String = sort(s2);
        return is_string_equal(sorted_string, sorted_string_2);
    }

    fn sort(s: String) -> String {
        let mut chars: Vec<char> = s.chars().collect();
        chars.sort();
        let new_string: String = chars.iter().collect();
        return new_string;
    }

    fn is_string_equal(s1: String, s2: String) -> bool {
        return s1 == s2;
    }

    println!(" ===== IS ABS ANAGRAM OF SBA ==== {}", is_anagram("abs".to_string(), "sba".to_string()));
    println!(" ===== IS COOL ANAGRAM OF OOO ==== {}", is_anagram("COOL".to_string(), "OOO".to_string()));
}
