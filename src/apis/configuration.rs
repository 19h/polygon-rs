/* 
 * Polygon API
 *
 * The future of fintech.
 *
 * OpenAPI spec version: 1.0.1
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */
use hyper;
use std::collections::HashMap;
use hyper::{Body, Client};

pub struct Configuration<C: hyper::client::connect::Connect + Clone + Send + Sync> {
  pub base_path: String,
  pub user_agent: Option<String>,
  pub client: hyper::client::Client<C, Body>,
  pub basic_auth: Option<BasicAuth>,
  pub oauth_access_token: Option<String>,
  pub api_key: Option<ApiKey>,
  // TODO: take an oauth2 token source, similar to the go one
}

pub type BasicAuth = (String, Option<String>);

pub struct ApiKey {
  pub prefix: Option<String>,
  pub key: String,
}

impl<C: hyper::client::connect::Connect + Clone + Send + Sync> Configuration<C> {
  pub fn new(client: hyper::client::Client<C, Body>) -> Configuration<C> {
    Configuration {
      base_path: "https://api.polygon.io/".to_owned(),
      user_agent: Some("Swagger-Codegen/1.0.1/rust".to_owned()),
      client: client,
      basic_auth: None,
      oauth_access_token: None,
      api_key: None,
    }
  }
}
