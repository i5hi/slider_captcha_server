extern crate image;
use image::{DynamicImage, GenericImage, GenericImageView, Rgba};
use rand::Rng;
use std::path::{PathBuf};

fn generate_puzzle(image_path: &str) -> Result<(String, String, u32), Box<dyn std::error::Error>> {
    // Load the input image and get its dimensions.
    let input_image = image::open(image_path)?;
    let (width, height) = input_image.dimensions();
    println!("Original Image Dimensions: {:?}x{:?} px", width, height);
    // Define the size of the puzzle piece.
    let piece_width = (width / 3) as u32;
    let piece_height = (height / 3) as u32;
    println!("Puzzle Piece Dimensions: {:?}x{:?} px", piece_width, piece_height);

    // Choose a random starting position for the puzzle piece.
    let mut rng = rand::thread_rng();
    let start_x = rng.gen_range(0..(width - piece_width));
    let start_y = rng.gen_range(piece_height..(2 * piece_height));

    println!("SOLUTION CO-ORDINATES: {:?},{:?}", start_x, start_y);
    // Crop the puzzle piece out of the input image.
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
            if x >= start_x && x < start_x + piece_width && y >= start_y && y < start_y + piece_height {
                rgba_pixel[3] = 0;
            }
            cropped_image.put_pixel(x, y, rgba_pixel);
        }
    }
    // Save the images to disk.
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

    puzzle_piece.save(puzzle_piece_path.clone())?;
    cropped_image.save(cropped_puzzle_path.clone())?;
    // Return the paths to the images and the coordinates of the solution.
    Ok((
        puzzle_piece_path.to_string(),
        cropped_puzzle_path.to_string(),
        start_x,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_generate_puzzle() {
        // set the path to the input image
        let binding = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("test")
            .join("archworkout.png");
        let input_path = binding.to_str().unwrap();

        // run the crop_image function
        let (cropped_file_path, output_file_path, _) = generate_puzzle(input_path).unwrap();

        // set the expected output paths
        let expected_cropped_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("test")
            .join("piece.png")
            .to_str()
            .unwrap()
            .to_owned();

        let expected_output_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("test")
            .join("puzzle.png")
            .to_str()
            .unwrap()
            .to_owned();

        assert_eq!(cropped_file_path, expected_cropped_path);
        assert_eq!(output_file_path, expected_output_path);
    }
}
