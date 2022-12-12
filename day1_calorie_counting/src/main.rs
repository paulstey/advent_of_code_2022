use anyhow::Result;
use time::Instant;
use calorie_counting;

fn main() -> Result<()> {
    let t1 = Instant::now();

    // File hosts must exist in current path before this produces output
    let lines_string_vec = calorie_counting::lines_from_file("data/data.txt");
    let lines_i32_vec = calorie_counting::string_vec_to_i32_vec(&lines_string_vec)?;

    let mut current_calories = 0;
    let mut calorie_totals_vec = Vec::new();

    for calories in lines_i32_vec {
        match calories {
            0 => {
                calorie_totals_vec.push(current_calories);
                current_calories = 0;
            }
            _ => {
                current_calories += calories;
            }
        }
    }
    calorie_totals_vec.sort();
    calorie_totals_vec.reverse();

    let solution: i32 = calorie_totals_vec
        .get(0..3)
        .expect("calorie_totals_vec did not have 3 elements")
        .iter()
        .sum();

    println!("{:?}", Instant::now() - t1);
    println!("{:?}", solution);
    
    Ok(())
}


// ---
