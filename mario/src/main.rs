use std::io::{self, Write};

fn main() {
    let mut height = String::new();
    loop
    {
        // Getting height 
        print!("Height: ");
        io::stdout()
            .flush()
            .expect("Error flushing prompt");

        height.clear();

        io::stdin()
            .read_line(&mut height)
            .expect("Error reading input");
        
        // Checking integer height received
        let height: u32 = match height.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        // Validating height in range
        if height < 1 || height > 8 {
            continue;
        }
        
        // Printing pyramid
        for row in 1..=height{
            for _ in (1..=height-row).rev(){
                print!(" ");
            }
            for _ in 1..=row{
                print!("#");
            }
            print!("  ");
            for _ in 1..=row{
                print!("#");
            }
            print!("\n");
        }
        break;
    }
    
}
