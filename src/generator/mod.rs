mod dice;

pub fn generate_word() -> String {
    let consonants = vec![
        'b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n', 'p', 'q', 'r', 's', 't', 'v', 'w',
        'x', 'z',
    ];
    let vocals = vec!['a', 'e', 'i', 'o', 'u', 'y'];

    let word_length = dice::roll_custom(11);

    for _ in 0..word_length {
        let length = consonants.len() as i32;
        let number = dice::roll_custom(length);
        let random_consonant = consonants[number as usize];
        println!("{}", random_consonant);
    }
    String::from("asdas")
}
