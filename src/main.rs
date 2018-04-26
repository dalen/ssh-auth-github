extern crate reqwest;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate ini;

use reqwest::header;
use ini::Ini;

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
    let conf = Ini::load_from_file("/etc/ssh-auth-github.ini").unwrap();
    let github = conf.section(Some("github".to_owned())).unwrap();
    let token = github.get("token").unwrap();
    let organization = github.get("organization").unwrap();
    let team = github.get("team").unwrap();
    query(&token, &organization, &team);
}
