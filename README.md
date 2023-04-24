# slider_captcha_server

a slider captcha puzzle creation and verification library to pretect http apis

# current implementation

This implementation only contains a single function that generates the puzzle by :

- taking an image as input
- selects a random coordinate from the image
- cropping out a piece of the image from the random coordinate
- writes the piece.png and the cropped image to the test folder
- returns the path of the images and the coordinates that represent the solution

# todo

- make generate_puzzle return DynamicImage rather than saving image to dist and returning the path
- provide an example using actix web server

# Test

```bash
cargo test -- --nocapture

# check the test folder for image artifacts
```
