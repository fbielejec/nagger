#[macro_use]
extern crate maplit;

use async_std::prelude::*;
use async_std::stream;
use graphql_client::{GraphQLQuery, Response};
use log::{debug, info, warn, error};
use reqwest::blocking::Client;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::time::Duration;

type URI = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "resources/schema.docs.graphql",
    query_path = "resources/query.graphql",
    response_derives = "Debug"
)]
struct RepoView;

#[derive(Debug)]
#[derive(Clone)]
struct Config<'a> {
    owner: String,
    name: String,
    github_api_token: String,
    slack_hook_url: String,
    log_level: String,
    user_to_id: HashMap<&'a str, &'a str>,
    interval: u64,
}

#[async_std::main]
async fn main()
              -> Result<(), anyhow::Error>
{

    let config = Config {
        owner: get_env_var ("REPO_OWNER",  None)?,
        name: get_env_var ("REPO_NAME", None)?,
        github_api_token: get_env_var ("GH_API_TOKEN", None)?,
        slack_hook_url: get_env_var ("SLACK_HOOK_URL", None)?,
        log_level: get_env_var ("LOGGING_LEVEL", Some (String::from ("info")))?,
        interval: 43200, // 12h
        // TODO : read from RON file
        user_to_id: hashmap! {
            "yenda" => "UHWKUD413",
            "jpmonettas" => "U018E11HXL3",
            "jbdtky" => "U018E11HXL3"
        }
    };

    env::set_var("RUST_LOG", &config.log_level);
    env_logger::init();

    info!("Running with {:#?}", &config);

    let query = RepoView::build_query(repo_view::Variables {
        owner: String::from (&config.owner),
        name: String::from (&config.name)
    });

    let client = Client::builder()
        .user_agent("graphql-rust/0.9.0")
        .build()
        .unwrap ();

    let mut interval = stream::interval(Duration::from_secs(config.interval));

    while interval.next().await.is_some () {
        nag_revieweres (&config, &client, &query)?;
    }

    Ok (())
}

fn nag_revieweres (config : &Config<'_>,
                   client : &Client,
                   query : &graphql_client::QueryBody<repo_view::Variables>)
                   -> Result<(), anyhow::Error>
{

    let Config { user_to_id, github_api_token, slack_hook_url, .. } = config;

    let response = client
        .post("https://api.github.com/graphql")
        .bearer_auth(github_api_token)
        .json(&query)
        .send()
        .unwrap ();

    response.error_for_status_ref().unwrap ();

    let response_body: Response<repo_view::ResponseData> = response.json().unwrap ();
    let response_data: repo_view::ResponseData = response_body.data.expect("missing response data");

    response_data
        .repository
        .expect("missing repository")
        .pull_requests
        .nodes
        .expect("missing pullRequests nodes")
        .into_iter()
        .for_each(| pull_request : Option<repo_view::RepoViewRepositoryPullRequestsNodes> |
                  // -> Result<String, Error>
                  {
                      let pull_request = pull_request.unwrap ();
                      let review_requests = pull_request.review_requests.unwrap ();
                      let reviews = pull_request.reviews.unwrap ();

                      let title = pull_request.title;
                      let url = pull_request.url;

                      if review_requests.total_count > 0 {

                          let requested_reviewers =
                              review_requests.nodes.unwrap ().into_iter().map (| review_request | {
                                  match review_request.unwrap ().requested_reviewer.unwrap () {
                                      repo_view::RepoViewRepositoryPullRequestsNodesReviewRequestsNodesRequestedReviewer::User(user) => {
                                          user.login
                                      },
                                      not_a_user => {
                                          error!("Unknown variant {:#?}", not_a_user);
                                          panic!("Unknown variant {:#?}", not_a_user);
                                      },
                                  }
                              }).collect::<HashSet<String>>();

                          let reviewers_reviewed =
                              reviews.nodes.unwrap ().into_iter().map (| review | {
                                  match review.unwrap ().author.unwrap ().on {
                                      repo_view::RepoViewRepositoryPullRequestsNodesReviewsNodesAuthorOn::User (user) => {
                                          user.login
                                      },
                                      other => {
                                          error!("Unknown variant {:#?}", other);
                                          panic!("Unknown variant {:#?}", other);
                                      },
                                  }
                              }).collect::<HashSet<String>>();

                          info!("Pull request title {:?}", title);
                          info!("All requested reviewers {:?}", requested_reviewers);
                          info!("Reviewers that reviewed: {:?}", reviewers_reviewed);
                          info!("{:?}", url);

                          requested_reviewers
                              .difference(&reviewers_reviewed)
                              .into_iter ()
                              .for_each (| user | {

                                  let user_id = user_to_id.get (&user.as_str ()).unwrap ();
                                  let body = make_request_body (&title, &url, &user_id);

                                  debug!("{:?}", body);

                                  let response = client
                                      .post(slack_hook_url)
                                      .json(&body)
                                      .send()
                                      .unwrap ();

                                  match response.error_for_status_ref() {
                                      Ok(_) => {
                                          info!("Succesfully posted a message to slack webhook")
                                      },
                                      Err(error) => {
                                          warn!("Error: error posting message to slack webhook: {:#?}", error)
                                      },

                                  }

                              });
                      }
                  });

    Ok (())
}

fn get_env_var (var : &str, default: Option<String> ) -> Result<String, anyhow::Error> {
    match env::var(var) {
        Ok (v) => Ok (v),
        Err (_) => {
            match default {
                None => {
                    error!("Missing ENV variable: {} not defined in environment", var);
                    panic! ("Missing ENV variable: {} not defined in environment", var);
                },
                Some (d) => Ok (d)
            }
        }
    }
}

fn make_request_body (title : &str, url : &str, user : &str) -> Value {
    json!({
        "blocks": [
            {
                "type":"header",
                "text": {
                    "type":"plain_text",
                    "text":"Review request",
                    "emoji": true
                }
            },
            {
                "type" : "section",
                "text" : {"type":"mrkdwn",
                          "text": format!("<@{}> you are requested as a reviewer for {}", user, title)
                },
                "accessory" : {
                    "type" : "button",
                    "text" : {
                        "type":"plain_text",
                        "text":"Review",
                        "emoji": true
                    },
                    "value": "click_me_123",
                    "url": url,
                    "action_id":"button-action"
                }
            }
        ]
    })
}

pub fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
