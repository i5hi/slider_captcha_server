# slider_captcha_server

a slider captcha puzzle creation and verification library to pretect http apis

## current implementation

This implementation only contains a single function that generates the puzzle by :

- taking a path to an image as input
- selects a random coordinate from the image
- cropping out a piece of the image from the random coordinate
- returns the following struct
```rust
pub struct SliderPuzzle{
    pub cropped_puzzle: DynamicImage,
    pub puzzle_piece: DynamicImage,
    pub x: f64,
    pub y: f64,
}
```
- the client will get 2 DynamicImages and `y` as the starting point
- `x` represents the solution

## todo

- test actix server puzzle generate - convert base64 images to png
- test actix server puzzle verification

## note

- the random coordinate currently is not the centre of the puzzle piece, rather the top left corner.
- currently verify logic is in the actix example

## Setup Environment

The standard way of installing Rust and Cargo is by using the script provided by rustup.rs:

```bash
curl https://sh.rustup.rs -sSf | sh
git clone https://github.com/i5hi/slider_captcha_server
cd slider_captcha_server
```

## Test

```bash
cargo test -- --nocapture
# check the test folder for image artifacts
```

## RUN LOCALHOST SERVER

```bash
cargo run --example actix
# will start the actix server example on port 18080

curl -X GET "localhost:18080/puzzle"

# returns :
# {
#     "puzzle_image": String, // base64 encoded
#     "piece_image": String, // base64 encoded
#     "id": String,
#     "y": f64,
# }

curl -X POST \
    -H 'content-type: application/json' \
    -d '{"id":"cfd47fd5-747c-4a64-8a74-43206aba8a21", "x":0.0239}' \
    'localhost:18080/puzzle/solution'

# solution must be a % as a decimal representing the x coordinate of the solution.
```
