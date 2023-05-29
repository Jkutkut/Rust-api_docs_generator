mod endpoint {
    use serde_derive::Deserialize;

    #[derive(Deserialize, Debug)]
    pub struct ApiCollection {
        pub name: String,
        pub description: String,
        pub apis: Vec<Api>,
        pub legend: Option<Vec<Legend>>,
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
        pub examples: Vec<EndpointExample>,
    }

    #[derive(Deserialize, Debug)]
    pub struct Parameter {
        pub name: String,
        pub description: String,
        pub example: String,
    }

    #[derive(Deserialize, Debug)]
    pub struct Filter {
        pub name: String,
        pub description: String,
        pub example: String,
    }

    #[derive(Deserialize, Debug)]
    pub struct EndpointExample {
        pub description: String,
        pub endpoint: String,
        pub data: Option<Vec<String>>,
        pub response_description: String,
        pub response_code: u16,
        pub response_body: Option<Vec<String>>,
    }

    #[derive(Deserialize, Debug)]
    pub enum Legend {
        Definition(LegendDefinition),
        Code(Vec<LegendCode>)
    }

    #[derive(Deserialize, Debug)]
    pub struct LegendDefinition {
        pub description: String,
        pub legend: Vec<(String, String)>
    }

    #[derive(Deserialize, Debug)]
    pub struct LegendCode {
        pub code: String,
        pub meaning: String,
        pub description: String
    }
}
use endpoint::*;

mod parse {
    use super::*;
    pub fn markdown(
        collection: ApiCollection
    ) -> String {
        let mut r = String::new();
        r = r + &format!("# 🚀 {}\n\n", collection.name);
        r = r + &format!("{}\n\n", collection.description);
        for api in collection.apis {
            r = r + &format!("## 📡 {}: `{}`\n\n", api.title, api.route);
            r = r + &format!("{}\n\n", api.description);
            for endpoint in api.endpoints {
                r = r + &format!("<details><summary> \
                    <b>{} <code>{}{}</code></b></summary>\n\n",
                    endpoint.method, api.route, endpoint.route
                );
                r = r + &format!("{}\n\n", endpoint.description);

                if let Some(parameters) = endpoint.parameters {
                    r = r + &format!("### Parameters\n\n");
                    r = r + &format!("| Name | Example | Description |\n");
                    r = r + &format!("| ---- | --- | --- |\n");
                    for param in parameters {
                        if !endpoint.route.contains(&format!(":{}", param.name)) {
                            eprintln!("Warning: parameter {} not found in route {}",
                                param.name, endpoint.route
                            );
                        }
                        r = r + &format!("| `{}` | `.../{}={}/...` | {} |\n",
                            param.name, param.name, param.example, param.description
                        );
                    }
                    r = r + &format!("\n\n");
                }

                if let Some(filters) = endpoint.filters {
                    r = r + &format!("### Filters\n\n");
                    r = r + &format!("| Name | Example | Description |\n");
                    r = r + &format!("| ---- | --- | --- |\n");
                    for filter in filters {
                        r = r + &format!("| `{}` | `?...&{}...` | {} |\n",
                            filter.name, filter.example, filter.description
                        );
                    }
                    r = r + &format!("\n\n");
                }
                match endpoint.examples.len() {
                    0 => eprintln!(
                        "Warning: no examples found for endpoint {}",
                            endpoint.route
                    ),
                    _ => {
                        r = r + &format!("<details><summary>Examples</summary>\n\n");
                        // for example in endpoint.examples {
                        for (i, example) in endpoint.examples.iter().enumerate() {
                            r = r + &format!("### {}\n\n", i + 1);
                            r = r + &format!("{}\n\n", example.description);
                            r = r + &format!("```javascript\n");
                            r = r + &format!("fetch(\"{}{}\", {{\n",
                                api.route, example.endpoint
                            );
                            r = r + &format!("    method: \"{}\",\n",
                                endpoint.method
                            );
                            if let Some(data) = &example.data {
                                r = r + &format!("    headers: {{ \"Content-Type\": \"application/json\" }},\n");
                                r = r + &format!("    body: JSON.stringify({}),\n",
                                    data.join(", ")
                                );
                            }
                            r = r + &format!("}})\n");
                            r = r + &format!(".then(response => response.json())\n");
                            r = r + &format!(".then(json => console.log(json));\n");
                            r = r + &format!("```\n\n");

                            r = r + &format!("Response:\n\n");
                            r = r + &format!("{}\n\n", example.response_description);
                            r = r + &format!("```json\n");
                            r = r + &format!("{}\n\n", example.response_code);
                            if let Some(body) = &example.response_body {
                                r = r + &format!("{}\n", body.join("\n"));
                            }
                            else {
                                r = r + "No response body.\n"
                            }
                            r = r + &format!("```\n\n");
                            
                        }
                        r = r + &format!("</details>\n\n");
                    }
                }

                // TODO legend 🔧

                r = r + &format!("</details>\n\n");
            }
        }
        // TODO add signature
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
