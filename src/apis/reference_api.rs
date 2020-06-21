/* 
 * Polygon API
 *
 * The future of fintech.
 *
 * OpenAPI spec version: 1.0.1

 */
#![allow(unused_imports)]
use std::sync::Arc;
use std::borrow::Borrow;
use std::borrow::Cow;
use std::collections::HashMap;

use hyper;
use serde_json;
use tokio::runtime::Runtime;
use futures;
use futures::{Future, Stream};
use bigdecimal::BigDecimal;

use hyper::Body;
use hyper::body::Bytes;
use hyper::body::HttpBody;
use std::str::FromStr;
use chrono::{NaiveDateTime, DateTime, FixedOffset, Utc, SecondsFormat};
use crate::OutlinePrint;
use crate::models::*;
use super::{Error, configuration};
use headers::{Authorization, Header};
use headers::authorization::Credentials;

pub struct ReferenceApiClient<C: hyper::client::connect::Connect + Clone + Send + Sync> {
    configuration: Arc<configuration::Configuration<C>>,
}

impl<C: hyper::client::connect::Connect + Clone + Send + Sync + 'static> ReferenceApiClient<C> {
    pub fn new(configuration: Arc<configuration::Configuration<C>>) -> ReferenceApiClient<C> {
        ReferenceApiClient {
            configuration: configuration,
        }
    }
}

pub trait ReferenceApi {
    fn v1_marketstatus_now_get(&self, ) -> Box<Result<MarketStatus, Error<serde_json::Value>>>;
    fn v1_marketstatus_upcoming_get(&self, ) -> Box<Result<Vec<MarketHoliday>, Error<serde_json::Value>>>;
    fn v1_meta_symbols_symbol_company_get(&self, symbol: &str) -> Box<Result<Company, Error<serde_json::Value>>>;
    fn v1_meta_symbols_symbol_news_get(&self, symbol: &str, perpage: f32, page: f32) -> Box<Result<Vec<News>, Error<serde_json::Value>>>;
    fn v2_reference_dividends_symbol_get(&self, symbol: &str) -> Box<Result<InlineResponse2003, Error<serde_json::Value>>>;
    fn v2_reference_financials_symbol_get(&self, symbol: &str, limit: f32, _type: &str, sort: &str) -> Box<Result<InlineResponse2004, Error<serde_json::Value>>>;
    fn v2_reference_locales_get(&self, ) -> Box<Result<InlineResponse2001, Error<serde_json::Value>>>;
    fn v2_reference_markets_get(&self, ) -> Box<Result<InlineResponse2001, Error<serde_json::Value>>>;
    fn v2_reference_splits_symbol_get(&self, symbol: &str) -> Box<Result<InlineResponse2002, Error<serde_json::Value>>>;
    fn v2_reference_tickers_get(&self, sort: &str, _type: &str, market: &str, locale: &str, search: &str, perpage: f32, page: f32, active: bool) -> Box<Result<Vec<Symbol>, Error<serde_json::Value>>>;
    fn v2_reference_types_get(&self, ) -> Box<Result<InlineResponse200, Error<serde_json::Value>>>;
}


impl<C: hyper::client::connect::Connect + Clone + Send + Sync + 'static>ReferenceApi for ReferenceApiClient<C> {
    fn v1_marketstatus_now_get(&self, ) -> Box<Result<MarketStatus, Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref apikey) = configuration.api_key {
            let key = apikey.key.clone();
            let val = match apikey.prefix {
                Some(ref prefix) => format!("{} {}", prefix, key),
                None => key,
            };
            auth_query.insert("apiKey".to_owned(), val);
        };
        let method = hyper::Method::GET;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/v1/marketstatus/now?{}", configuration.base_path, query_string);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        //dbg!(&uri_str);
        let uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::builder()
            .method(method)
            .uri(uri);

        let headers = req.headers_mut().unwrap();
        if let Some(ref user_agent) = configuration.user_agent {
            headers.insert(hyper::header::USER_AGENT, user_agent.parse().unwrap());
        }


        for (key, val) in auth_headers {
            headers.insert(hyper::header::HeaderName::from_str(key.as_ref()).unwrap(), val.parse().unwrap());
        }

        let somebody = Body::empty();

        let req = req.body(somebody).unwrap();

        let mut r = Runtime::new().unwrap();
        // send request
        let ass = async {
            let a = configuration.client.request(req)
                                 .await
                                 .map_err(|e| -> Error<serde_json::Value> { Error::from(e) });

            let mut a = a?;

            let status = a.status();
            let mut resbd: Vec<u8> = vec![];

            while let Some(chunk) = a.body_mut().data().await {
                let mut uppered = chunk.unwrap().to_vec();
                resbd.append(uppered.as_mut());
            }

            let a = if status.is_success() {
                        Ok(resbd)
                    } else {
                        Err(Error::from((status, resbd.borrow())))
                    };

            let a = a?;

            let a = serde_json::from_slice(a.borrow())
                .map_err(|e| -> Error<serde_json::Value> { Error::from(e) });
            a
        };
        let a = r.block_on(ass);
        Box::new(a)

    }

    fn v1_marketstatus_upcoming_get(&self, ) -> Box<Result<Vec<MarketHoliday>, Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref apikey) = configuration.api_key {
            let key = apikey.key.clone();
            let val = match apikey.prefix {
                Some(ref prefix) => format!("{} {}", prefix, key),
                None => key,
            };
            auth_query.insert("apiKey".to_owned(), val);
        };
        let method = hyper::Method::GET;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/v1/marketstatus/upcoming?{}", configuration.base_path, query_string);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        //dbg!(&uri_str);
        let uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::builder()
            .method(method)
            .uri(uri);

        let headers = req.headers_mut().unwrap();
        if let Some(ref user_agent) = configuration.user_agent {
            headers.insert(hyper::header::USER_AGENT, user_agent.parse().unwrap());
        }


        for (key, val) in auth_headers {
            headers.insert(hyper::header::HeaderName::from_str(key.as_ref()).unwrap(), val.parse().unwrap());
        }

        let somebody = Body::empty();

        let req = req.body(somebody).unwrap();

        let mut r = Runtime::new().unwrap();
        // send request
        let ass = async {
            let a = configuration.client.request(req)
                                 .await
                                 .map_err(|e| -> Error<serde_json::Value> { Error::from(e) });

            let mut a = a?;

            let status = a.status();
            let mut resbd: Vec<u8> = vec![];

            while let Some(chunk) = a.body_mut().data().await {
                let mut uppered = chunk.unwrap().to_vec();
                resbd.append(uppered.as_mut());
            }

            let a = if status.is_success() {
                        Ok(resbd)
                    } else {
                        Err(Error::from((status, resbd.borrow())))
                    };

            let a = a?;

            let a = serde_json::from_slice(a.borrow())
                .map_err(|e| -> Error<serde_json::Value> { Error::from(e) });
            a
        };
        let a = r.block_on(ass);
        Box::new(a)

    }

    fn v1_meta_symbols_symbol_company_get(&self, symbol: &str) -> Box<Result<Company, Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref apikey) = configuration.api_key {
            let key = apikey.key.clone();
            let val = match apikey.prefix {
                Some(ref prefix) => format!("{} {}", prefix, key),
                None => key,
            };
            auth_query.insert("apiKey".to_owned(), val);
        };
        let method = hyper::Method::GET;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/v1/meta/symbols/{symbol}/company?{}", configuration.base_path, query_string, symbol=symbol);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        //dbg!(&uri_str);
        let uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::builder()
            .method(method)
            .uri(uri);

        let headers = req.headers_mut().unwrap();
        if let Some(ref user_agent) = configuration.user_agent {
            headers.insert(hyper::header::USER_AGENT, user_agent.parse().unwrap());
        }


        for (key, val) in auth_headers {
            headers.insert(hyper::header::HeaderName::from_str(key.as_ref()).unwrap(), val.parse().unwrap());
        }

        let somebody = Body::empty();

        let req = req.body(somebody).unwrap();

        let mut r = Runtime::new().unwrap();
        // send request
        let ass = async {
            let a = configuration.client.request(req)
                                 .await
                                 .map_err(|e| -> Error<serde_json::Value> { Error::from(e) });

            let mut a = a?;

            let status = a.status();
            let mut resbd: Vec<u8> = vec![];

            while let Some(chunk) = a.body_mut().data().await {
                let mut uppered = chunk.unwrap().to_vec();
                resbd.append(uppered.as_mut());
            }

            let a = if status.is_success() {
                        Ok(resbd)
                    } else {
                        Err(Error::from((status, resbd.borrow())))
                    };

            let a = a?;

            let a = serde_json::from_slice(a.borrow())
                .map_err(|e| -> Error<serde_json::Value> { Error::from(e) });
            a
        };
        let a = r.block_on(ass);
        Box::new(a)

    }

    fn v1_meta_symbols_symbol_news_get(&self, symbol: &str, perpage: f32, page: f32) -> Box<Result<Vec<News>, Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref apikey) = configuration.api_key {
            let key = apikey.key.clone();
            let val = match apikey.prefix {
                Some(ref prefix) => format!("{} {}", prefix, key),
                None => key,
            };
            auth_query.insert("apiKey".to_owned(), val);
        };
        let method = hyper::Method::GET;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("perpage", &perpage.outline_print() );
            query.append_pair("page", &page.outline_print() );
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/v1/meta/symbols/{symbol}/news?{}", configuration.base_path, query_string, symbol=symbol);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        //dbg!(&uri_str);
        let uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::builder()
            .method(method)
            .uri(uri);

        let headers = req.headers_mut().unwrap();
        if let Some(ref user_agent) = configuration.user_agent {
            headers.insert(hyper::header::USER_AGENT, user_agent.parse().unwrap());
        }


        for (key, val) in auth_headers {
            headers.insert(hyper::header::HeaderName::from_str(key.as_ref()).unwrap(), val.parse().unwrap());
        }

        let somebody = Body::empty();

        let req = req.body(somebody).unwrap();

        let mut r = Runtime::new().unwrap();
        // send request
        let ass = async {
            let a = configuration.client.request(req)
                                 .await
                                 .map_err(|e| -> Error<serde_json::Value> { Error::from(e) });

            let mut a = a?;

            let status = a.status();
            let mut resbd: Vec<u8> = vec![];

            while let Some(chunk) = a.body_mut().data().await {
                let mut uppered = chunk.unwrap().to_vec();
                resbd.append(uppered.as_mut());
            }

            let a = if status.is_success() {
                        Ok(resbd)
                    } else {
                        Err(Error::from((status, resbd.borrow())))
                    };

            let a = a?;

            let a = serde_json::from_slice(a.borrow())
                .map_err(|e| -> Error<serde_json::Value> { Error::from(e) });
            a
        };
        let a = r.block_on(ass);
        Box::new(a)

    }

    fn v2_reference_dividends_symbol_get(&self, symbol: &str) -> Box<Result<InlineResponse2003, Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref apikey) = configuration.api_key {
            let key = apikey.key.clone();
            let val = match apikey.prefix {
                Some(ref prefix) => format!("{} {}", prefix, key),
                None => key,
            };
            auth_query.insert("apiKey".to_owned(), val);
        };
        let method = hyper::Method::GET;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/v2/reference/dividends/{symbol}?{}", configuration.base_path, query_string, symbol=symbol);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        //dbg!(&uri_str);
        let uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::builder()
            .method(method)
            .uri(uri);

        let headers = req.headers_mut().unwrap();
        if let Some(ref user_agent) = configuration.user_agent {
            headers.insert(hyper::header::USER_AGENT, user_agent.parse().unwrap());
        }


        for (key, val) in auth_headers {
            headers.insert(hyper::header::HeaderName::from_str(key.as_ref()).unwrap(), val.parse().unwrap());
        }

        let somebody = Body::empty();

        let req = req.body(somebody).unwrap();

        let mut r = Runtime::new().unwrap();
        // send request
        let ass = async {
            let a = configuration.client.request(req)
                                 .await
                                 .map_err(|e| -> Error<serde_json::Value> { Error::from(e) });

            let mut a = a?;

            let status = a.status();
            let mut resbd: Vec<u8> = vec![];

            while let Some(chunk) = a.body_mut().data().await {
                let mut uppered = chunk.unwrap().to_vec();
                resbd.append(uppered.as_mut());
            }

            let a = if status.is_success() {
                        Ok(resbd)
                    } else {
                        Err(Error::from((status, resbd.borrow())))
                    };

            let a = a?;

            let a = serde_json::from_slice(a.borrow())
                .map_err(|e| -> Error<serde_json::Value> { Error::from(e) });
            a
        };
        let a = r.block_on(ass);
        Box::new(a)

    }

    fn v2_reference_financials_symbol_get(&self, symbol: &str, limit: f32, _type: &str, sort: &str) -> Box<Result<InlineResponse2004, Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref apikey) = configuration.api_key {
            let key = apikey.key.clone();
            let val = match apikey.prefix {
                Some(ref prefix) => format!("{} {}", prefix, key),
                None => key,
            };
            auth_query.insert("apiKey".to_owned(), val);
        };
        let method = hyper::Method::GET;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("limit", &limit.outline_print() );
            query.append_pair("type", &_type.outline_print() );
            query.append_pair("sort", &sort.outline_print() );
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/v2/reference/financials/{symbol}?{}", configuration.base_path, query_string, symbol=symbol);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        //dbg!(&uri_str);
        let uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::builder()
            .method(method)
            .uri(uri);

        let headers = req.headers_mut().unwrap();
        if let Some(ref user_agent) = configuration.user_agent {
            headers.insert(hyper::header::USER_AGENT, user_agent.parse().unwrap());
        }


        for (key, val) in auth_headers {
            headers.insert(hyper::header::HeaderName::from_str(key.as_ref()).unwrap(), val.parse().unwrap());
        }

        let somebody = Body::empty();

        let req = req.body(somebody).unwrap();

        let mut r = Runtime::new().unwrap();
        // send request
        let ass = async {
            let a = configuration.client.request(req)
                                 .await
                                 .map_err(|e| -> Error<serde_json::Value> { Error::from(e) });

            let mut a = a?;

            let status = a.status();
            let mut resbd: Vec<u8> = vec![];

            while let Some(chunk) = a.body_mut().data().await {
                let mut uppered = chunk.unwrap().to_vec();
                resbd.append(uppered.as_mut());
            }

            let a = if status.is_success() {
                        Ok(resbd)
                    } else {
                        Err(Error::from((status, resbd.borrow())))
                    };

            let a = a?;

            let a = serde_json::from_slice(a.borrow())
                .map_err(|e| -> Error<serde_json::Value> { Error::from(e) });
            a
        };
        let a = r.block_on(ass);
        Box::new(a)

    }

    fn v2_reference_locales_get(&self, ) -> Box<Result<InlineResponse2001, Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref apikey) = configuration.api_key {
            let key = apikey.key.clone();
            let val = match apikey.prefix {
                Some(ref prefix) => format!("{} {}", prefix, key),
                None => key,
            };
            auth_query.insert("apiKey".to_owned(), val);
        };
        let method = hyper::Method::GET;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/v2/reference/locales?{}", configuration.base_path, query_string);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        //dbg!(&uri_str);
        let uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::builder()
            .method(method)
            .uri(uri);

        let headers = req.headers_mut().unwrap();
        if let Some(ref user_agent) = configuration.user_agent {
            headers.insert(hyper::header::USER_AGENT, user_agent.parse().unwrap());
        }


        for (key, val) in auth_headers {
            headers.insert(hyper::header::HeaderName::from_str(key.as_ref()).unwrap(), val.parse().unwrap());
        }

        let somebody = Body::empty();

        let req = req.body(somebody).unwrap();

        let mut r = Runtime::new().unwrap();
        // send request
        let ass = async {
            let a = configuration.client.request(req)
                                 .await
                                 .map_err(|e| -> Error<serde_json::Value> { Error::from(e) });

            let mut a = a?;

            let status = a.status();
            let mut resbd: Vec<u8> = vec![];

            while let Some(chunk) = a.body_mut().data().await {
                let mut uppered = chunk.unwrap().to_vec();
                resbd.append(uppered.as_mut());
            }

            let a = if status.is_success() {
                        Ok(resbd)
                    } else {
                        Err(Error::from((status, resbd.borrow())))
                    };

            let a = a?;

            let a = serde_json::from_slice(a.borrow())
                .map_err(|e| -> Error<serde_json::Value> { Error::from(e) });
            a
        };
        let a = r.block_on(ass);
        Box::new(a)

    }

    fn v2_reference_markets_get(&self, ) -> Box<Result<InlineResponse2001, Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref apikey) = configuration.api_key {
            let key = apikey.key.clone();
            let val = match apikey.prefix {
                Some(ref prefix) => format!("{} {}", prefix, key),
                None => key,
            };
            auth_query.insert("apiKey".to_owned(), val);
        };
        let method = hyper::Method::GET;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/v2/reference/markets?{}", configuration.base_path, query_string);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        //dbg!(&uri_str);
        let uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::builder()
            .method(method)
            .uri(uri);

        let headers = req.headers_mut().unwrap();
        if let Some(ref user_agent) = configuration.user_agent {
            headers.insert(hyper::header::USER_AGENT, user_agent.parse().unwrap());
        }


        for (key, val) in auth_headers {
            headers.insert(hyper::header::HeaderName::from_str(key.as_ref()).unwrap(), val.parse().unwrap());
        }

        let somebody = Body::empty();

        let req = req.body(somebody).unwrap();

        let mut r = Runtime::new().unwrap();
        // send request
        let ass = async {
            let a = configuration.client.request(req)
                                 .await
                                 .map_err(|e| -> Error<serde_json::Value> { Error::from(e) });

            let mut a = a?;

            let status = a.status();
            let mut resbd: Vec<u8> = vec![];

            while let Some(chunk) = a.body_mut().data().await {
                let mut uppered = chunk.unwrap().to_vec();
                resbd.append(uppered.as_mut());
            }

            let a = if status.is_success() {
                        Ok(resbd)
                    } else {
                        Err(Error::from((status, resbd.borrow())))
                    };

            let a = a?;

            let a = serde_json::from_slice(a.borrow())
                .map_err(|e| -> Error<serde_json::Value> { Error::from(e) });
            a
        };
        let a = r.block_on(ass);
        Box::new(a)

    }

    fn v2_reference_splits_symbol_get(&self, symbol: &str) -> Box<Result<InlineResponse2002, Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref apikey) = configuration.api_key {
            let key = apikey.key.clone();
            let val = match apikey.prefix {
                Some(ref prefix) => format!("{} {}", prefix, key),
                None => key,
            };
            auth_query.insert("apiKey".to_owned(), val);
        };
        let method = hyper::Method::GET;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/v2/reference/splits/{symbol}?{}", configuration.base_path, query_string, symbol=symbol);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        //dbg!(&uri_str);
        let uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::builder()
            .method(method)
            .uri(uri);

        let headers = req.headers_mut().unwrap();
        if let Some(ref user_agent) = configuration.user_agent {
            headers.insert(hyper::header::USER_AGENT, user_agent.parse().unwrap());
        }


        for (key, val) in auth_headers {
            headers.insert(hyper::header::HeaderName::from_str(key.as_ref()).unwrap(), val.parse().unwrap());
        }

        let somebody = Body::empty();

        let req = req.body(somebody).unwrap();

        let mut r = Runtime::new().unwrap();
        // send request
        let ass = async {
            let a = configuration.client.request(req)
                                 .await
                                 .map_err(|e| -> Error<serde_json::Value> { Error::from(e) });

            let mut a = a?;

            let status = a.status();
            let mut resbd: Vec<u8> = vec![];

            while let Some(chunk) = a.body_mut().data().await {
                let mut uppered = chunk.unwrap().to_vec();
                resbd.append(uppered.as_mut());
            }

            let a = if status.is_success() {
                        Ok(resbd)
                    } else {
                        Err(Error::from((status, resbd.borrow())))
                    };

            let a = a?;

            let a = serde_json::from_slice(a.borrow())
                .map_err(|e| -> Error<serde_json::Value> { Error::from(e) });
            a
        };
        let a = r.block_on(ass);
        Box::new(a)

    }

    fn v2_reference_tickers_get(&self, sort: &str, _type: &str, market: &str, locale: &str, search: &str, perpage: f32, page: f32, active: bool) -> Box<Result<Vec<Symbol>, Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref apikey) = configuration.api_key {
            let key = apikey.key.clone();
            let val = match apikey.prefix {
                Some(ref prefix) => format!("{} {}", prefix, key),
                None => key,
            };
            auth_query.insert("apiKey".to_owned(), val);
        };
        let method = hyper::Method::GET;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("sort", &sort.outline_print() );
            query.append_pair("type", &_type.outline_print() );
            query.append_pair("market", &market.outline_print() );
            query.append_pair("locale", &locale.outline_print() );
            query.append_pair("search", &search.outline_print() );
            query.append_pair("perpage", &perpage.outline_print() );
            query.append_pair("page", &page.outline_print() );
            query.append_pair("active", &active.outline_print() );
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/v2/reference/tickers?{}", configuration.base_path, query_string);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        //dbg!(&uri_str);
        let uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::builder()
            .method(method)
            .uri(uri);

        let headers = req.headers_mut().unwrap();
        if let Some(ref user_agent) = configuration.user_agent {
            headers.insert(hyper::header::USER_AGENT, user_agent.parse().unwrap());
        }


        for (key, val) in auth_headers {
            headers.insert(hyper::header::HeaderName::from_str(key.as_ref()).unwrap(), val.parse().unwrap());
        }

        let somebody = Body::empty();

        let req = req.body(somebody).unwrap();

        let mut r = Runtime::new().unwrap();
        // send request
        let ass = async {
            let a = configuration.client.request(req)
                                 .await
                                 .map_err(|e| -> Error<serde_json::Value> { Error::from(e) });

            let mut a = a?;

            let status = a.status();
            let mut resbd: Vec<u8> = vec![];

            while let Some(chunk) = a.body_mut().data().await {
                let mut uppered = chunk.unwrap().to_vec();
                resbd.append(uppered.as_mut());
            }

            let a = if status.is_success() {
                        Ok(resbd)
                    } else {
                        Err(Error::from((status, resbd.borrow())))
                    };

            let a = a?;

            let a = serde_json::from_slice(a.borrow())
                .map_err(|e| -> Error<serde_json::Value> { Error::from(e) });
            a
        };
        let a = r.block_on(ass);
        Box::new(a)

    }

    fn v2_reference_types_get(&self, ) -> Box<Result<InlineResponse200, Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref apikey) = configuration.api_key {
            let key = apikey.key.clone();
            let val = match apikey.prefix {
                Some(ref prefix) => format!("{} {}", prefix, key),
                None => key,
            };
            auth_query.insert("apiKey".to_owned(), val);
        };
        let method = hyper::Method::GET;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/v2/reference/types?{}", configuration.base_path, query_string);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        //dbg!(&uri_str);
        let uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::builder()
            .method(method)
            .uri(uri);

        let headers = req.headers_mut().unwrap();
        if let Some(ref user_agent) = configuration.user_agent {
            headers.insert(hyper::header::USER_AGENT, user_agent.parse().unwrap());
        }


        for (key, val) in auth_headers {
            headers.insert(hyper::header::HeaderName::from_str(key.as_ref()).unwrap(), val.parse().unwrap());
        }

        let somebody = Body::empty();

        let req = req.body(somebody).unwrap();

        let mut r = Runtime::new().unwrap();
        // send request
        let ass = async {
            let a = configuration.client.request(req)
                                 .await
                                 .map_err(|e| -> Error<serde_json::Value> { Error::from(e) });

            let mut a = a?;

            let status = a.status();
            let mut resbd: Vec<u8> = vec![];

            while let Some(chunk) = a.body_mut().data().await {
                let mut uppered = chunk.unwrap().to_vec();
                resbd.append(uppered.as_mut());
            }

            let a = if status.is_success() {
                        Ok(resbd)
                    } else {
                        Err(Error::from((status, resbd.borrow())))
                    };

            let a = a?;

            let a = serde_json::from_slice(a.borrow())
                .map_err(|e| -> Error<serde_json::Value> { Error::from(e) });
            a
        };
        let a = r.block_on(ass);
        Box::new(a)

    }

}
