use image::{DynamicImage, GenericImage, GenericImageView, Rgba};
use rand::Rng;
// Import the slider captcha library

#[derive(Debug, Clone)]
pub struct SliderPuzzle {
    pub cropped_puzzle: image::DynamicImage,
    pub puzzle_piece: image::DynamicImage,
    pub x: f64,
    pub y: f64,
}

impl SliderPuzzle {
    pub fn new(image_path: &str) -> Result<SliderPuzzle, String> {
        let input_image = match image::open(image_path) {
            Ok(image) => image,
            Err(e) => return Err(e.to_string()),
        };
        let (width, height) = input_image.dimensions();
        // Define the size of the puzzle piece.
        let piece_width = width / 5;
        let piece_height = height / 5;
        // Choose a random starting position for the puzzle piece.
        let mut rng = rand::thread_rng();
        let start_x = rng.gen_range(0..(width - piece_width));
        let start_y = rng.gen_range(piece_height..(2 * piece_height));

        // Crop the puzzle piece out of the input image.
        let mut puzzle_piece = DynamicImage::new_rgb8(piece_width, piece_height);
        for y in 0..piece_height {
            for x in 0..piece_width {
                let pixel = input_image.get_pixel(start_x + x, start_y + y);
                let rgba_pixel = Rgba([pixel[0], pixel[1], pixel[2], pixel[3]]);
                puzzle_piece.put_pixel(x, y, rgba_pixel);
            }
        }
        // Create a blank canvas for the cropped image.
        let mut cropped_image = DynamicImage::new_rgba8(width, height);
        for y in 0..height {
            for x in 0..width {
                let pixel = input_image.get_pixel(x, y);
                let mut rgba_pixel = Rgba([pixel[0], pixel[1], pixel[2], pixel[3]]);
                if x >= start_x
                    && x < start_x + piece_width
                    && y >= start_y
                    && y < start_y + piece_height
                {
                    rgba_pixel[3] = 0;
                }
                cropped_image.put_pixel(x, y, rgba_pixel);
            }
        }

        Ok(SliderPuzzle {
            cropped_puzzle: cropped_image,
            puzzle_piece,
            y: (start_y as f64 / height as f64),
            x: (start_x as f64 / width as f64),
        })
    }
    pub fn update_x(&mut self, x: f64) -> &mut SliderPuzzle {
        self.x = x;
        self
    }
    pub fn get_x(&self) -> f64 {
        self.x
    }
}
pub fn verify_puzzle(solution: f64, submission: f64, error_margin: f64) -> bool {
    (solution - submission).abs() < error_margin
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_generate_puzzle() {
        let binding = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("test")
            .join("archworkout.png");
        let input_path = binding.to_str().unwrap();
        let slider_puzzle = SliderPuzzle::new(input_path).unwrap();
        let puzzle_piece_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("test")
            .join("piece.png")
            .to_str()
            .unwrap()
            .to_owned();
        let cropped_puzzle_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("test")
            .join("puzzle.png")
            .to_str()
            .unwrap()
            .to_owned();
        slider_puzzle
            .puzzle_piece
            .save(puzzle_piece_path.clone())
            .unwrap();
        slider_puzzle
            .cropped_puzzle
            .save(cropped_puzzle_path.clone())
            .unwrap();
        println!(
            "SOLUTION:\nx: {:?}\ny: {:?}\n",
            slider_puzzle.x, slider_puzzle.y
        );
        println!(
            "Check images at:\n{:?}\n{:?}",
            cropped_puzzle_path, puzzle_piece_path
        );
    }
    #[test]
    fn playground() {
        let x: u32 = 100;
        let y: f32 = 1.23;

        let z: f32 = x as f32 + y;
        println!("{}", z);

        assert!(z == 101.23)
    }
}
