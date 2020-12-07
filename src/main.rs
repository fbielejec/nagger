use graphql_client::{GraphQLQuery, Response};
use serde::*;
use std::error::Error;
use reqwest;
use log::{debug, info, error};
use std::env;

// use graphql_client::*;
// use structopt::StructOpt;
// use prettytable::*;
// use anyhow::*;

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
struct Config {
    owner: String,
    name: String,
    github_api_token: String,
    log_level: String,
}

fn main() -> Result<(), anyhow::Error> {

    let config = Config {
        owner: get_env_var ("OWNER",  None)?,
        name: get_env_var ("NAME", None)?,
        github_api_token: get_env_var ("GH_API_TOKEN", None)?,
        log_level: get_env_var ("LOGGING_LEVEL", Some (String::from ("info")))?
    };

    env::set_var("RUST_LOG", config.log_level);
    env_logger::init();

    let query = RepoView::build_query(repo_view::Variables {
        owner: config.owner,
        name: config.name
    });

    let client = reqwest::blocking::Client::builder()
        .user_agent("graphql-rust/0.9.0")
        .build()?;

    let response = client
        .post("https://api.github.com/graphql")
        .bearer_auth(config.github_api_token)
        .json(&query)
        .send()?;

    // info! ("GH response: {:#?}", response);
    response.error_for_status_ref()?;

    let response_body: Response<repo_view::ResponseData> = response.json()?;
    // info!("{:?}", response_body);

    let response_data: repo_view::ResponseData = response_body.data.expect("missing response data");

    &response_data
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
                      if review_requests.total_count > 0 {

                          info!("{:?}", pull_request.title);

                          info!("Requested reviewers");

                          // let mut requested_reviewers: HashSet::new() =
                          review_requests.nodes.unwrap ().into_iter().for_each (| review_request | {
                              match review_request.unwrap ().requested_reviewer.unwrap () {
                                  repo_view::RepoViewRepositoryPullRequestsNodesReviewRequestsNodesRequestedReviewer::User(user) => {
                                      info!("{:?}", user.login);
                                      // user.login
                                  },
                                  not_a_user => {
                                      error!("Unknown variant {:#?}", not_a_user);
                                      panic!("Unknown variant {:#?}", not_a_user);
                                  },
                              }
                          });//.collect ();

                          info!("Reviewers that reviewed:");

                          reviews.nodes.unwrap ().into_iter().for_each (| review | {
                              match review.unwrap ().author.unwrap ().on {
                                  repo_view::RepoViewRepositoryPullRequestsNodesReviewsNodesAuthorOn::User (user) => {
                                      info!("{:?}", user.login);
                                  },
                                  other => {
                                      error!("Unknown variant {:#?}", other);
                                      panic!("Unknown variant {:#?}", other);
                                  },
                              }
                          });
                      }
                  });

    // println!("Hello, world!");
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

pub fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
