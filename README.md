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

- the random coordinate currently is not the centre of the puzzle piece, rather the top left corner.
- library verify function (currently verify logic is in the actix example)
- actix server puzzle generate must return the Y axis position
- test actix server puzzle generate - convert base64 images to png
- test actix server puzzle verification
- better naming convention

# Test

```bash
cargo test -- --nocapture

# check the test folder for image artifacts

cargo run --example actix

# will start the actix server example

curl -X GET "localhost:18080/puzzle"

# returns 2 images in base64 and a request_id

curl -X GET "localhost:18080/verify?request_id=$request_id&solution=$solution"

# solution must be a decimal % representing the x coordinate of the solution.
```
