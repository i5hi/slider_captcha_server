# slider_captcha_server

a slider captcha puzzle creation and verification library to pretect http apis

## current implementation

This implementation only contains a single function that generates the puzzle by :

- taking an image as input
- selects a random coordinate from the image
- cropping out a piece of the image from the random coordinate
- writes the piece.png and the cropped image to the test folder
- returns the path of the images and the coordinates that represent the solution

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
# returns 2 images in base64 and a request_id
curl -X POST "localhost:18080/puzzle/solution?request_id=$request_id&solution=$solution"
# solution must be a % as a decimal representing the x coordinate of the solution.
```
