extern crate reqwest;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use reqwest::header;
use std::env;

#[derive(Deserialize, Debug)]
struct Response {
    data: Data,
}

#[derive(Deserialize, Debug)]
struct Team {
    members: TeamMemberConnection,
}

#[derive(Deserialize, Debug)]
struct Organization {
    team: Team,
}

#[derive(Deserialize, Debug)]
struct Data {
    organization: Organization,
}

#[derive(Deserialize, Debug)]
struct TeamMemberConnection {
    nodes: Vec<User>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct User {
    login: String,
    public_keys: PublicKeyConnection,
}

#[derive(Deserialize, Debug)]
struct PublicKeyConnection {
    nodes: Vec<PublicKey>,
}

#[derive(Deserialize, Debug)]
struct PublicKey {
    key: String,
}

#[derive(Serialize, Debug)]
struct Query {
    query: String,
}

fn query(token: &String, organization: &String, team: &String) {
    let client = reqwest::Client::new();

    let query = Query {
        query: format!(
            r#"
query {{
  organization(login: "{}") {{
    team(slug: "{}") {{
      members(first: 100) {{
        nodes {{
          login
          publicKeys(first: 100) {{
            nodes {{
              key
            }}
          }}
        }}
      }}
    }}
  }}
}}
"#,
            &organization, &team
        ),
    };

    let mut resp = client
        .post("https://api.github.com/graphql")
        .json(&query)
        .header(header::Authorization(header::Bearer {
            token: token.clone(),
        }))
        .send()
        .unwrap();

    let data: Response = serde_json::from_str(&resp.text().unwrap()).unwrap();

    for user in &data.data.organization.team.members.nodes {
        for key in &user.public_keys.nodes {
            println!("{} {}", &key.key, user.login);
        }
    }
}

fn main() {
    match env::var("GITHUB_TOKEN").ok() {
        Some(token) => match env::var("GITHUB_ORGANIZATION").ok() {
            Some(organization) => match env::var("GITHUB_TEAM").ok() {
                Some(team) => {
                    query(&token, &organization, &team);
                }
                _ => println!("missing GITHUB_TEAM"),
            },
            _ => println!("missing GITHUB_ORGANIZATION"),
        },
        _ => println!("missing GITHUB_TOKEN"),
    }
}
