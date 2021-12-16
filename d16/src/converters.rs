pub fn to_decimal(bin_slize: &[u8]) -> u64 {
    let (res, _) = bin_slize.iter().rev().fold((0, 0), |(res, power), bit| {
        (res + ((*bit as u64) << power), power + 1)
    });

    res
}

pub fn to_binary(hexstr: &str) -> Vec<u8> {
    let mut res = Vec::with_capacity(hexstr.len() * 4);

    for ch in hexstr.chars() {
        match ch {
            '0' => res.extend([0, 0, 0, 0]),
            '1' => res.extend([0, 0, 0, 1]),
            '2' => res.extend([0, 0, 1, 0]),
            '3' => res.extend([0, 0, 1, 1]),
            '4' => res.extend([0, 1, 0, 0]),
            '5' => res.extend([0, 1, 0, 1]),
            '6' => res.extend([0, 1, 1, 0]),
            '7' => res.extend([0, 1, 1, 1]),
            '8' => res.extend([1, 0, 0, 0]),
            '9' => res.extend([1, 0, 0, 1]),
            'A' => res.extend([1, 0, 1, 0]),
            'B' => res.extend([1, 0, 1, 1]),
            'C' => res.extend([1, 1, 0, 0]),
            'D' => res.extend([1, 1, 0, 1]),
            'E' => res.extend([1, 1, 1, 0]),
            'F' => res.extend([1, 1, 1, 1]),
            _ => todo!(),
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_binary_test() {
        assert_eq!(
            to_binary("D2FE28"),
            [1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0]
        );
    }

    #[test]
    fn to_decimal_test() {
        assert_eq!(to_decimal(&[1, 0, 0]), 4);
        assert_eq!(to_decimal(&[1, 1, 0, 1, 1]), 27);
        assert_eq!(to_decimal(&[0, 0, 0, 1, 1, 0, 1, 1]), 27);
        assert_eq!(to_decimal(&[0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 1, 0, 1]), 2021);
    }
}
