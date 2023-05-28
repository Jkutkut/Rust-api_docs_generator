mod endpoint {
    use serde_derive::Deserialize;

    #[derive(Deserialize, Debug)]
    pub struct ApiCollection {
        pub name: String,
        pub description: String,
        pub apis: Vec<Api>,
    }

    #[derive(Deserialize, Debug)]
    pub struct Api {
        pub title: String,
        pub description: String,
        pub route: String,
        pub endpoints: Vec<Endpoint>,
    }

    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "UPPERCASE")]
    pub enum EndpointMethod {
        Get,
        Post,
        Put,
        Patch,
        Update,
        Delete,
        Options,
    }

    impl std::fmt::Display for EndpointMethod {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            match self {
                EndpointMethod::Get => write!(f, "GET"),
                EndpointMethod::Post => write!(f, "POST"),
                EndpointMethod::Put => write!(f, "PUT"),
                EndpointMethod::Patch => write!(f, "PATCH"),
                EndpointMethod::Update => write!(f, "UPDATE"),
                EndpointMethod::Delete => write!(f, "DELETE"),
                EndpointMethod::Options => write!(f, "OPTIONS"),
            }
        }
    }

    #[derive(Deserialize, Debug)]
    pub struct Endpoint {
        pub method: EndpointMethod,
        pub route: String,
        pub description: String,
        pub parameters: Option<Vec<Parameter>>,
        pub filters: Option<Vec<Filter>>,
        pub examples: Vec<Example>,
    }

    #[derive(Deserialize, Debug)]
    pub struct Parameter {
        pub name: String,
        pub description: String,
        pub example: Example,
    }

    #[derive(Deserialize, Debug)]
    pub struct Filter {
        pub name: String,
        pub description: String,
        pub example: Example,
    }

    #[derive(Deserialize, Debug)]
    #[serde(untagged)]
    pub enum Example {
        Simple(String),
        Complex(ComplexExample),
    }

    #[derive(Deserialize, Debug)]
    pub struct ComplexExample {
        pub description: String,
        pub code: Code,
        pub result_description: String,
        pub result: Code,
    }

    #[derive(Deserialize, Debug)]
    pub enum Code {
        CodeString(String),
        CodeBlock(Vec<String>),
    }
}
use endpoint::*;

mod parse {
    use super::*;
    pub fn markdown(
        collection: ApiCollection
    ) -> String {
        // 🚀 🥅 📬 🔧 📡
        let mut r = String::new();
        r = r + &format!("## {}\n\n", collection.name);
        r = r + &format!("{}\n\n", collection.description);
        for api in collection.apis {
            r = r + &format!("### {}: `{}`\n\n", api.title, api.route);
            r = r + &format!("{}\n\n", api.description);
            for endpoint in api.endpoints {
                r = r + &format!("<details><summary> \
                    <b>{} <code>{}{}</code></b></summary>\n\n",
                    endpoint.method, api.route, endpoint.route
                );
                r = r + &format!("{}\n\n", endpoint.description);

                if let Some(parameters) = endpoint.parameters {
                    r = r + &format!("#### Parameters\n\n");
                    r = r + &format!("| Name | Example | Description |\n");
                    r = r + &format!("| ---- | --- | --- |\n");
                    for param in parameters {
                        if !endpoint.route.contains(&format!(":{}", param.name)) {
                            eprintln!("Warning: parameter {} not found in route {}",
                                param.name, endpoint.route
                            );
                        }
                        let example = match param.example {
                            Example::Simple(example) => example,
                            Example::Complex(_) => {
                                eprintln!("Warning: complex example for parameter {}. Removing example",
                                    param.name
                                );
                                "".to_string()
                            }
                        };
                        r = r + &format!("| `{}` | {} | `{}` |\n",
                            param.name, example, param.description
                        );
                    }
                    r = r + &format!("\n\n");
                }

                // TODO filters

                // TODO examples

                r = r + &format!("</details>\n\n");
            }
        }
        r
    }
}

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

    // TODO generate markdown

    let content = parse::markdown(data);

    match std::fs::write(&file_output, content) {
        Ok(_) => println!("Done"),
        Err(err) => eprintln!("Error writing file {}: {}", file_output, err),
    }
}
