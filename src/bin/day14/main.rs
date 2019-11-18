extern crate adventofcode2018;

const INPUT: usize = 190221;
const INPUT_STRING: &'static str = "190221";

fn main() {
    println!("Part One: {}", part_one());
    println!("Part Two: {}", part_two());
}

fn part_one() -> String {
    let mut recipes: Vec<u8> = vec![3, 7];
    let mut i = 0;
    let mut j = 1;

    while recipes.len() < (INPUT+10) {
        let a = recipes[i];
        let b = recipes[j];
        let mut sum = a + b;
        let mut digits = vec![];
        while sum >= 10 {
            digits.push(sum % 10);
            sum /= 10;
        }
        digits.push(sum);
        digits.reverse();
        for d in digits {
            recipes.push(d);
        }
        i += 1 + a as usize;
        i %= recipes.len();
        j += 1 + b as usize;
        j %= recipes.len();
    }

    let last10 = &recipes[INPUT..INPUT+10];
    
    let mut answer = String::new();
    for d in last10 {
        answer.push_str(&d.to_string());
    }

    answer
}

fn part_two() -> String {
    let mut recipes: Vec<u8> = vec![3, 7];
    let mut recipes_string = String::from("37");
    let mut i = 0;
    let mut j = 1;

    loop {
        let prev_num_recipes = recipes.len();
        
        let a = recipes[i];
        let b = recipes[j];
        let mut sum = a + b;
        let mut digits = vec![];
        while sum >= 10 {
            digits.push(sum % 10);
            sum /= 10;
        }
        digits.push(sum);
        digits.reverse();
        for d in digits {
            recipes.push(d);
            recipes_string.push_str(&d.to_string());
        }
        i += 1 + a as usize;
        i %= recipes.len();
        j += 1 + b as usize;
        j %= recipes.len();
        
        // check if finished
        let start = if prev_num_recipes > 6 {
            prev_num_recipes - 6
        } else {
            0
        };
        let tail = &recipes_string[start..];
        if let Some(offset) = tail.find(INPUT_STRING) {
            return (start+offset).to_string();
        }
    }

}
