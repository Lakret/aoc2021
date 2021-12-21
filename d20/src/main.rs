use std::collections::HashSet;
use std::fs;

// 1:08 start

fn main() {
    println!("Hello, world!");
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
    pub fn enhance(&self) -> Image {
        let mut new_image = self.clone();
        new_image.canvas.clear();

        let margin = 4;
        for y in (self.min_y - margin)..(self.max_y + margin + 1) {
            for x in (self.min_x - margin)..(self.max_x + margin + 1) {
                let new_pixel = self.algo[self.get_pixel_code(x, y)];

                if new_pixel == 1 {
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
                    print!("#");
                } else {
                    print!(".");
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn p1_test() {
        let test_input = fs::read_to_string("../d20_test_input").unwrap();
        let image = Image::parse(test_input);
        assert_eq!(image.canvas.len(), 10);
        assert_eq!(image.algo[0..5], [0, 0, 1, 0, 1]);

        assert_eq!(image.get_pixel_code(2, 2), 34);

        image.draw();

        let enhanced = image.enhance();
        enhanced.draw();

        let enhanced2 = enhanced.enhance();
        enhanced2.draw();
    }
}
