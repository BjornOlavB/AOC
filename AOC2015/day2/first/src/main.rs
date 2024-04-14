use std::io::BufRead;
use std::path::Path;
use std::io::BufReader;
use std::fs::File;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new("../input");
    let file = File::open(&path)?;
    let reader = BufReader::new(file);
    let mut total_square_feet = 0;
    let mut length_of_ribbon:i32= 0;

    for line_result in reader.lines() {
        let line = line_result?;
        let split_of_var: Vec<i32> = line.split('x').map(|num| num.parse::<i32>()).collect::<Result<Vec<i32>, _>>()?;

        let length = split_of_var[0];
        let width = split_of_var[1];
        let height = split_of_var[2];

        let base = length*width;
        let side = width*height;
        let side_two = height*length;

        let min_value = base.min(side.min(side_two));

        let square_feet_of_present = 2*base+2*side+2*side_two+min_value;
   
        total_square_feet += square_feet_of_present;
        // Part Two:
        let square_foot_cubic = length*width*height;
        let min_value_ribbon = length.min(width.min(height));
        let mid_value = if (length >= width && length <= height) || (length >= height && length <= width) {
            length
        } else if (width >= length && width <= height) || (width >= height && width <= length) {
          width
        } else {
              height
        };
        length_of_ribbon += min_value_ribbon*2+mid_value*2+square_foot_cubic;
    }

    println!("Number of square feet of paper: {}", total_square_feet);
    println!("The length of the ribbon in feet: {}", length_of_ribbon);

    Ok(())
}

/*
 * The elves are running low on wrapping paper, and so they need to submit an order for more. They have a list of the dimensions
 * (length l, width w, and height h) of each present, and only want to order exactly as much as they need.

Fortunately, every present is a box (a perfect right rectangular prism), which makes calculating the required wrapping paper for 
each gift a little easier: find the surface area of the box, which is 2*l*w + 2*w*h + 2*h*l. The elves also need a little extra paper for each present: 
the area of the smallest side.

For example:

    A present with dimensions 2x3x4 requires 2*6 + 2*12 + 2*8 = 52 square feet of wrapping paper plus 6 square feet of slack, for a total of 58 square feet.
    A present with dimensions 1x1x10 requires 2*1 + 2*10 + 2*10 = 42 square feet of wrapping paper plus 1 square foot of slack, for a total of 43 square feet.

All numbers in the elves' list are in feet. How many total square feet of wrapping paper should they order?

--- Part Two ---

The elves are also running low on ribbon. Ribbon is all the same width, so they only have to worry about the length they need to order, which they would again like to be exact.

The ribbon required to wrap a present is the shortest distance around its sides, or the smallest perimeter of any one face. 
Each present also requires a bow made out of ribbon as well; the feet of ribbon required for the perfect bow is equal to the cubic feet of volume of the present. 
Don't ask how they tie the bow, though; they'll never tell.

For example:

    A present with dimensions 2x3x4 requires 2+2+3+3 = 10 feet of ribbon to wrap the present plus 2*3*4 = 24 feet of ribbon for the bow, for a total of 34 feet.
    A present with dimensions 1x1x10 requires 1+1+1+1 = 4 feet of ribbon to wrap the present plus 1*1*10 = 10 feet of ribbon for the bow, for a total of 14 feet.

How many total feet of ribbon should they order?

*/
