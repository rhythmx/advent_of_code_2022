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
            elf.calories_list.push(calories);
            println!("read: {value}");
        }
        if elf.calories_list.len() > 0 {
            elves.push(elf);
        } else {
            running = false;
        }
    }

    elves.sort_by(|a,b| b.total_calories.cmp(&a.total_calories));
    if elves.len() > 3 {
        let total = elves.get(0).unwrap().total_calories + 
                    elves.get(1).unwrap().total_calories + 
                    elves.get(2).unwrap().total_calories;
        println!("Total of largest 3 elf inventories combined is {total}");
    }
}