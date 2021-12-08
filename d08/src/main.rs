#[macro_use]
extern crate lazy_static;

use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;

lazy_static! {
    static ref UNIQUE_LENGTH_TO_DIGIT: HashMap<usize, u32> = {
        let mut m = HashMap::new();
        m.insert(2, 1);
        m.insert(3, 7);
        m.insert(4, 4);
        m.insert(7, 8);
        m
    };
}

type InputPart = Vec<HashSet<char>>;
type Input = Vec<(InputPart, InputPart)>;

fn main() -> Result<(), Box<dyn Error>> {
    let input = parse("d08_input")?;
    let p1_ans = p1(&input);
    println!("p1 = {}", p1_ans);

    let p2_ans = p2(&input)?;
    println!("p1 = {}", p2_ans);

    Ok(())
}

fn p1(input: &Input) -> u32 {
    let mut res = 0;

    for (_, out_part) in input {
        for w in out_part {
            if UNIQUE_LENGTH_TO_DIGIT.contains_key(&w.len()) {
                res += 1;
            }
        }
    }

    res
}

fn p2(input: &Input) -> Result<u32, Box<dyn Error>> {
    let mut res = 0;

    for (in_part, out_part) in input {
        let cipher = decipher(&in_part).ok_or("cannot decipher!")?;
        let out_value = decode_out(&out_part, &cipher)?;
        res += out_value;
    }

    Ok(res)
}

fn parse(path: &str) -> Result<Input, Box<dyn Error>> {
    let input = fs::read_to_string(path)?;
    input
        .trim_end()
        .split("\n")
        .map(|line| {
            let mut parts = line.split(" | ");
            let in_part = parts.next().ok_or("missing in part")?;
            let out_part = parts.next().ok_or("missing out part")?;

            Ok((parse_part(in_part), parse_part(out_part)))
        })
        .collect()
}

fn parse_part(part: &str) -> Vec<HashSet<char>> {
    part.split_whitespace()
        .map(|x| x.chars().collect())
        .collect::<Vec<_>>()
}

fn decipher(in_part: &InputPart) -> Option<[HashSet<char>; 10]> {
    // we know that 2-len word is always 1, 3-len word is always 7,
    // and the signal that 7 has that 1 doesn't have is `a`.
    let one_chars = in_part.iter().find(|w| w.len() == 2)?.clone();
    let seven_chars = in_part.iter().find(|w| w.len() == 3)?.clone();

    // 2 is the only digit that doesn't have f;
    // `c` is encountered in 8 digits, `f` is encountered in 9 digits.
    // since we know `a` from `get_a`, we can use this to distinguish `c` and `f`
    let c_char = one_chars
        .iter()
        .find(|ch| in_part.iter().filter(|w| w.contains(ch)).count() == 8)?
        .clone();

    // four and eight have unique length
    let four_chars = in_part.iter().find(|w| w.len() == 4)?.clone();
    let eight_chars = in_part.iter().find(|w| w.len() == 7)?.clone();

    // there are three 6-len digits: 0, 6, and 9.
    // 6 is the only one without `c`
    let six_chars = in_part
        .iter()
        .find(|w| w.len() == 6 && !w.contains(&c_char))?
        .clone();

    // `e` is the only char that is present in exactly 4 digits
    let e_char = "abcdefg"
        .chars()
        .find(|ch| in_part.iter().filter(|w| w.contains(ch)).count() == 4)?
        .clone();

    // 0 is the only 6-len digit with `e`, that is not 6
    let zero_chars = in_part
        .iter()
        .find(|w| w.len() == 6 && **w != six_chars && w.contains(&e_char))?
        .clone();

    // now the only remaining 6-len that is not 0 and not 6 is 9.
    let nine_chars = in_part
        .iter()
        .find(|w| w.len() == 6 && **w != zero_chars && **w != six_chars)?
        .clone();

    // 2 is the only 5-len digit with `e`
    let two_chars = in_part
        .iter()
        .find(|w| w.len() == 5 && w.contains(&e_char))?
        .clone();

    // 3 is the only 5-len digit with `c`, that is not 2
    let three_chars = in_part
        .iter()
        .find(|w| w.len() == 5 && **w != two_chars && w.contains(&c_char))?
        .clone();

    // 5 is the only remaining 5-len digit
    let five_chars = in_part
        .iter()
        .find(|w| w.len() == 5 && **w != two_chars && **w != three_chars)?
        .clone();

    Some([
        zero_chars,
        one_chars,
        two_chars,
        three_chars,
        four_chars,
        five_chars,
        six_chars,
        seven_chars,
        eight_chars,
        nine_chars,
    ])
}

fn decode_out(out_part: &InputPart, cipher: &[HashSet<char>; 10]) -> Result<u32, Box<dyn Error>> {
    let mut digits = vec![];

    for out_w in out_part {
        let digit = cipher
            .iter()
            .position(|w| w == out_w)
            .ok_or("no matching digit in cipher")?;
        digits.push(digit.to_string());
    }

    digits.join("").parse::<u32>().map_err(|e| e.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let test_input = parse("../d08_test_input").unwrap();
        assert_eq!(p1(&test_input), 26);

        let input = parse("../d08_input").unwrap();
        assert_eq!(p1(&input), 390);
    }

    #[test]
    fn p2_test() {
        let in_part = [
            "acedgfb", "cdfbe", "gcdfa", "fbcad", "dab", "cefabd", "cdfgeb", "eafb", "cagedb", "ab",
        ]
        .iter()
        .map(|x| x.chars().collect())
        .collect::<Vec<_>>();

        let out_part = ["cdfeb", "fcadb", "cdfeb", "cdbaf"]
            .iter()
            .map(|x| x.chars().collect::<HashSet<_>>())
            .collect::<Vec<_>>();

        let cipher = decipher(&in_part).unwrap();

        let [zero, one, two, three, four, five, six, seven, eight, nine] = &cipher;
        assert_eq!(zero, &"cagedb".chars().collect());
        assert_eq!(one, &"ab".chars().collect());
        assert_eq!(two, &"gcdfa".chars().collect());
        assert_eq!(three, &"fbcad".chars().collect());
        assert_eq!(four, &"abef".chars().collect());
        assert_eq!(five, &"cdfbe".chars().collect());
        assert_eq!(six, &"cdfgeb".chars().collect());
        assert_eq!(seven, &"adb".chars().collect());
        assert_eq!(eight, &"abcdefg".chars().collect());
        assert_eq!(nine, &"cefabd".chars().collect());

        let t = "cdfeb".chars().collect::<HashSet<_>>();
        assert_eq!(cipher.iter().position(|w| *w == t).unwrap(), 5);

        assert_eq!(decode_out(&out_part, &cipher).unwrap(), 5353);

        let test_input = parse("../d08_test_input").unwrap();
        assert_eq!(p2(&test_input).unwrap(), 61229);

        let input = parse("../d08_input").unwrap();
        assert_eq!(p2(&input).unwrap(), 1011785);
    }
}
