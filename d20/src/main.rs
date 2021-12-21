use std::collections::HashSet;
use std::fs;

fn main() {
    let input = fs::read_to_string("d20_input").unwrap();
    let image = Image::parse(input);

    let p1_ans = image.enhance_times(2).canvas.len();
    assert_eq!(p1_ans, 5268);
    println!("p1: {}", p1_ans);

    let image_enhanced = image.enhance_times(50);

    let p2_ans = image_enhanced.canvas.len();
    assert_eq!(p2_ans, 16875);
    println!("p2: {}", p2_ans);

    image_enhanced.draw();
}

#[derive(Debug, Clone)]
pub struct Image {
    algo: [u8; 512],
    canvas: HashSet<(i64, i64)>,
    min_x: i64,
    max_x: i64,
    min_y: i64,
    max_y: i64,
    missing_as: u8,
}

const OFFSET: [i64; 3] = [-1, 0, 1];

impl Image {
    pub fn enhance_times(&self, times: u32) -> Image {
        let mut new_image = self.clone();

        for _ in 0..times {
            new_image = new_image.enhance();
        }

        new_image
    }

    pub fn enhance(&self) -> Image {
        let mut new_image = self.clone();
        new_image.canvas.clear();

        let mut non_missing_as = 1;
        if self.algo[0] == 1 {
            new_image.missing_as = self.missing_as ^ 1;
            non_missing_as = self.missing_as ^ 0;
        }

        let margin = 10;
        for y in (self.min_y - margin)..(self.max_y + margin + 1) {
            for x in (self.min_x - margin)..(self.max_x + margin + 1) {
                let new_pixel = self.algo[self.get_pixel_code(x, y)];

                if new_pixel == non_missing_as {
                    new_image.canvas.insert((x, y));

                    if x < new_image.min_x {
                        new_image.min_x = x;
                    }
                    if x > new_image.max_x {
                        new_image.max_x = x;
                    }
                    if y < new_image.min_y {
                        new_image.min_y = y;
                    }
                    if y > new_image.max_y {
                        new_image.max_y = y;
                    }
                }
            }
        }

        new_image
    }

    pub fn draw(&self) {
        println!();

        for y in self.min_y..self.max_y + 1 {
            for x in self.min_x..self.max_x + 1 {
                if self.canvas.contains(&(x, y)) {
                    if self.missing_as == 0 {
                        print!("#");
                    } else {
                        print!(".");
                    }
                } else {
                    if self.missing_as == 0 {
                        print!(".");
                    } else {
                        print!("#");
                    }
                }
            }
            println!();
        }

        println!();
    }

    pub fn get_pixel_code(&self, x: i64, y: i64) -> usize {
        let mut v = Vec::with_capacity(9);

        for dy in OFFSET {
            for dx in OFFSET {
                let bit = if self.canvas.contains(&(x + dx, y + dy)) {
                    1 ^ self.missing_as
                } else {
                    self.missing_as
                };

                v.push(bit);
            }
        }

        let (res, _) = v.iter().rev().fold((0, 0), |(res, power), bit| {
            (res + ((*bit as usize) << power), power + 1)
        });

        res
    }

    pub fn parse(input: String) -> Image {
        let mut lines = input.trim_end().split("\n");

        let mut algo = [0u8; 512];
        for (idx, ch) in lines.next().unwrap().chars().enumerate() {
            if ch == '#' {
                algo[idx] = 1;
            }
        }

        lines.next().unwrap();

        let mut canvas = HashSet::new();
        let mut max_x = 0;
        let mut max_y = 0;
        for (y, row) in lines.enumerate() {
            let y = y as i64;

            for (x, ch) in row.chars().enumerate() {
                let x = x as i64;

                if ch == '#' {
                    canvas.insert((x, y));
                }

                if x > max_x {
                    max_x = x;
                }
            }

            if y > max_y {
                max_y = y;
            }
        }

        Image {
            algo,
            canvas,
            max_x,
            max_y,
            min_x: 0,
            min_y: 0,
            missing_as: 0,
        }
    }
}

// run with `cargo test --bin d20 -- --nocapture`
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn p1_test() {
        let test_input = fs::read_to_string("../d20_test_input").unwrap();
        let test_image = Image::parse(test_input);
        assert_eq!(test_image.canvas.len(), 10);
        assert_eq!(test_image.algo[0..5], [0, 0, 1, 0, 1]);

        assert_eq!(test_image.get_pixel_code(2, 2), 34);

        let enhanced = test_image.enhance();
        let enhanced2 = enhanced.enhance();
        assert_eq!(enhanced2.canvas.len(), 35);
    }

    #[test]
    fn p2_test() {
        let test_input = fs::read_to_string("../d20_test_input").unwrap();
        let test_image = Image::parse(test_input);

        let test_image_enhanced = test_image.enhance_times(50);
        assert_eq!(test_image_enhanced.canvas.len(), 3351);

        let input = fs::read_to_string("../d20_input").unwrap();
        let image = Image::parse(input);
        let image_enhanced = image.enhance_times(50);
        assert_eq!(image_enhanced.canvas.len(), 16875);
    }
}
