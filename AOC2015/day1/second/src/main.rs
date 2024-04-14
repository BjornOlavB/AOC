use std::path::Path;
use std::io;
use std::io::BufReader;
use std::fs::File;
use std::io::BufRead;

fn main()->io::Result<()> {
   let path = Path::new("../input"); 
    let file = File::open(&path)?;
    let reader = BufReader::new(file);
    let mut santa_pos = 0;
    let mut counter = 0;
    
    if let Some(line) = reader.lines().next() {
        for paranthesis in line?.chars(){
            match paranthesis {
            '(' => santa_pos += 1,
            ')' => santa_pos -= 1,
            _ => {}
            }  
            counter += 1;
            if santa_pos < 0  {
                print!("The character position that puts Santa in the basement for the first time is {}", counter);
                break;
            }
        }
    }
    Ok(())
}


/*Now, given the same instructions, find the position of the first character that causes him to enter the basement
 * (floor -1). The first character in the instructions has position 1, the second character has position 2, and so on.
For example:

    ) causes him to enter the basement at character position 1.
    ()()) causes him to enter the basement at character position 5.

What is the position of the character that causes Santa to first enter the basement?
*/
