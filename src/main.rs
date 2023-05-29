mod model;

mod parse;

use model::ApiCollection;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let (file_input, file_output) = match args.len() {
        2 => {
            (args[1].clone(), args[1].replace(".json", ".md"))
        }
        3 => (args[1].clone(), args[2].clone()),
        _ => {
            println!("Usage: {} <file_input> <file_output>", args[0]);
            return;
        }
    };

    let input = match std::fs::read_to_string(&file_input) {
        Ok(input) => input,
        Err(err) => {
            eprintln!("Error reading file {}: {}", file_input, err);
            return;
        }
    };

    println!("Generating docs for {} into {}", file_input, file_output);

    let data = match serde_json::from_str::<ApiCollection>(&input) {
        Ok(data) => data,
        Err(err) => {
            eprintln!("Error parsing file {}:\n{}", file_input, err);
            return;
        }
    };

    let content = parse::markdown(data);

    match std::fs::write(&file_output, content) {
        Ok(_) => println!("Done"),
        Err(err) => eprintln!("Error writing file {}: {}", file_output, err),
    }
}
