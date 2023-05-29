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
#[serde(untagged)]
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