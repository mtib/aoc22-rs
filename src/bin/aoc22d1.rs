use std::{error::Error, fs, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    let root_dir = env!("CARGO_MANIFEST_DIR");
    let mut file_path = PathBuf::from(root_dir);
    file_path.push("./res/d1elf_calories.txt");

    if let Some(file_path_string) = file_path.to_str() {
        let foo: String = fs::read_to_string(file_path_string)?;

        let mut ranked_iter = foo
            .split("\n\n")
            .map(|rows| {
                rows.split("\n")
                    .map(|num_str| {
                        if let Ok(parsed) = num_str.parse::<usize>() {
                            parsed
                        } else {
                            0
                        }
                    })
                    .sum::<usize>()
            })
            .enumerate()
            .collect::<Vec<(usize, usize)>>();
        ranked_iter.sort_by(|(_, cal1), (_, cal2)| cal1.cmp(cal2).reverse());

        let top_three = &ranked_iter[0..3];
        println!(
            "#1 elf carries {} calories",
            top_three.get(0).map(|(_, cal)| *cal).unwrap_or(0)
        );
        println!(
            "top three elves carry {} together",
            top_three.iter().map(|(_, cal)| *cal).sum::<usize>()
        );
    } else {
        panic!("Failed to read input file");
    };

    Ok(())
}