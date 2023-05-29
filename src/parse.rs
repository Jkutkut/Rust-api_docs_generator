use super::model::*;

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
            r = r + &format!("</details>\n\n");
        }
    }

    if let Some(legends) = collection.legend {
        r = r + &format!("<details><summary><h2>🔧 Legend</h2></summary>\n\n");
        for legend in legends {
            match legend {
                Legend::Definition(legend) => {
                    r = r + &format!("## Legend\n\n");
                    r = r + &format!("{}\n\n", legend.description);
                    r = r + &format!("| Element | Meaning |\n");
                    r = r + &format!("| ------- | ------- |\n");
                    for (element, meaning) in legend.legend {
                        r = r + &format!("| `{}` | {} |\n", element, meaning);
                    }
                    r = r + &format!("\n\n");
                },
                Legend::Code(legend) => {
                    r = r + &format!("## API Codes\n\n");
                    r = r + &format!("| Code | Meaning | Description |\n");
                    r = r + &format!("| ---- | ------- | ----------- |\n");
                    for code in legend {
                        r = r + &format!("| `{}` | *{}* | {} |\n",
                            code.code, code.meaning, code.description
                        );
                    }
                    r = r + &format!("\n\n");
                }
            }
        }
        r = r + &format!("</details>\n\n");
    }
    r = r + &format!("Made with ❤️ using [api_docs_generator](https://github.com/Jkutkut/rust-api_docs_generator).\n");
    r
}