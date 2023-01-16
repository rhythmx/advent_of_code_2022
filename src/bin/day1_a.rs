use std::io;

struct ElfInventory {
    calories_list : Vec<i64>,
    total_calories : i64
}

impl ElfInventory {
    fn new() -> ElfInventory {
        ElfInventory{
            calories_list: Vec::new(),
            total_calories: 0
        }
    }
}

fn main() {
    println!("Please enter elf calorie inventory in the specified format.");
    println!("Use two successive newlines or 'EOF' to end the input.");
    println!("Input>");

    let mut elves: Vec<ElfInventory> = Vec::new();
    let mut running = true;
    let mut max_calories = 0;
    while running {
        let mut elf = ElfInventory::new();
        loop {
            let mut line = String::new();
            let value : &str = match io::stdin().read_line(&mut line) {
                Ok(_) => line.strip_suffix("\r\n").unwrap(),
                Err(_) => "EOF"
            };
            if value == "" {
                break;
            }
            if value == "EOF" {
                running = false;
                break;
            }
            let calories = value.parse().unwrap();
            elf.total_calories += calories;
            if elf.total_calories > max_calories {
                max_calories = elf.total_calories;
            }
            elf.calories_list.push(calories);
            println!("read: {value}");
        }
        if elf.calories_list.len() > 0 {
            elves.push(elf);
        } else {
            running = false;
        }
    }
    //let mut i = 0;
    //for elf in &elves {
    //    println!("elf #{i}:");
    //    for calories in &elf.calories_list {
    //        println!("    {calories}");
    //    }
    //   i+=1;
    //}
    println!("Max total calories is {max_calories}");
}