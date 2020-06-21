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

pub struct CryptoApiClient<C: hyper::client::connect::Connect + Clone + Send + Sync> {
    configuration: Arc<configuration::Configuration<C>>,
}

impl<C: hyper::client::connect::Connect + Clone + Send + Sync + 'static> CryptoApiClient<C> {
    pub fn new(configuration: Arc<configuration::Configuration<C>>) -> CryptoApiClient<C> {
        CryptoApiClient {
            configuration: configuration,
        }
    }
}

pub trait CryptoApi {
    fn v1_historic_crypto_from_to_date_get(&self, from: &str, to: &str, date: String, offset: i64, limit: i64) -> Box<Result<InlineResponse20017, Error<serde_json::Value>>>;
    fn v1_last_crypto_from_to_get(&self, from: &str, to: &str) -> Box<Result<InlineResponse20015, Error<serde_json::Value>>>;
    fn v1_meta_crypto_exchanges_get(&self, ) -> Box<Result<Vec<CryptoExchange>, Error<serde_json::Value>>>;
    fn v1_open_close_crypto_from_to_date_get(&self, from: &str, to: &str, date: String) -> Box<Result<InlineResponse20016, Error<serde_json::Value>>>;
    fn v2_aggs_grouped_locale_locale_market_market_date_get(&self, locale: &str, market: &str, date: &str, unadjusted: bool) -> Box<Result<AggResponse, Error<serde_json::Value>>>;
    fn v2_aggs_ticker_ticker_prev_get(&self, ticker: &str, unadjusted: bool) -> Box<Result<AggResponse, Error<serde_json::Value>>>;
    fn v2_aggs_ticker_ticker_range_multiplier_timespan_from_to_get(&self, ticker: &str, multiplier: f32, timespan: &str, from: &str, to: &str, unadjusted: bool, sort: String) -> Box<Result<AggResponse, Error<serde_json::Value>>>;
    fn v2_snapshot_locale_global_markets_crypto_direction_get(&self, direction: &str) -> Box<Result<InlineResponse20018, Error<serde_json::Value>>>;
    fn v2_snapshot_locale_global_markets_crypto_tickers_get(&self, ) -> Box<Result<InlineResponse20018, Error<serde_json::Value>>>;
    fn v2_snapshot_locale_global_markets_crypto_tickers_ticker_book_get(&self, ticker: &str) -> Box<Result<InlineResponse20020, Error<serde_json::Value>>>;
    fn v2_snapshot_locale_global_markets_crypto_tickers_ticker_get(&self, ticker: &str) -> Box<Result<InlineResponse20019, Error<serde_json::Value>>>;
}


impl<C: hyper::client::connect::Connect + Clone + Send + Sync + 'static>CryptoApi for CryptoApiClient<C> {
    fn v1_historic_crypto_from_to_date_get(&self, from: &str, to: &str, date: String, offset: i64, limit: i64) -> Box<Result<InlineResponse20017, Error<serde_json::Value>>> {
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
            query.append_pair("offset", &offset.outline_print() );
            query.append_pair("limit", &limit.outline_print() );
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/v1/historic/crypto/{from}/{to}/{date}?{}", configuration.base_path, query_string, from=from, to=to, date=date);

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

    fn v1_last_crypto_from_to_get(&self, from: &str, to: &str) -> Box<Result<InlineResponse20015, Error<serde_json::Value>>> {
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
        let uri_str = format!("{}/v1/last/crypto/{from}/{to}?{}", configuration.base_path, query_string, from=from, to=to);

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

    fn v1_meta_crypto_exchanges_get(&self, ) -> Box<Result<Vec<CryptoExchange>, Error<serde_json::Value>>> {
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
        let uri_str = format!("{}/v1/meta/crypto-exchanges?{}", configuration.base_path, query_string);

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

    fn v1_open_close_crypto_from_to_date_get(&self, from: &str, to: &str, date: String) -> Box<Result<InlineResponse20016, Error<serde_json::Value>>> {
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
        let uri_str = format!("{}/v1/open-close/crypto/{from}/{to}/{date}?{}", configuration.base_path, query_string, from=from, to=to, date=date);

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

    fn v2_aggs_grouped_locale_locale_market_market_date_get(&self, locale: &str, market: &str, date: &str, unadjusted: bool) -> Box<Result<AggResponse, Error<serde_json::Value>>> {
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
            query.append_pair("unadjusted", &unadjusted.outline_print() );
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/v2/aggs/grouped/locale/{locale}/market/{market}/{date}?{}", configuration.base_path, query_string, locale=locale, market=market, date=date);

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

    fn v2_aggs_ticker_ticker_prev_get(&self, ticker: &str, unadjusted: bool) -> Box<Result<AggResponse, Error<serde_json::Value>>> {
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
            query.append_pair("unadjusted", &unadjusted.outline_print() );
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/v2/aggs/ticker/{ticker}/prev?{}", configuration.base_path, query_string, ticker=ticker);

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

    fn v2_aggs_ticker_ticker_range_multiplier_timespan_from_to_get(&self, ticker: &str, multiplier: f32, timespan: &str, from: &str, to: &str, unadjusted: bool, sort: String) -> Box<Result<AggResponse, Error<serde_json::Value>>> {
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
            query.append_pair("unadjusted", &unadjusted.outline_print() );
            query.append_pair("sort", &serde_json::to_string( &sort ).unwrap());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/v2/aggs/ticker/{ticker}/range/{multiplier}/{timespan}/{from}/{to}?{}", configuration.base_path, query_string, ticker=ticker, multiplier=multiplier, timespan=timespan, from=from, to=to);

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

    fn v2_snapshot_locale_global_markets_crypto_direction_get(&self, direction: &str) -> Box<Result<InlineResponse20018, Error<serde_json::Value>>> {
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
        let uri_str = format!("{}/v2/snapshot/locale/global/markets/crypto/{direction}?{}", configuration.base_path, query_string, direction=direction);

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

    fn v2_snapshot_locale_global_markets_crypto_tickers_get(&self, ) -> Box<Result<InlineResponse20018, Error<serde_json::Value>>> {
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
        let uri_str = format!("{}/v2/snapshot/locale/global/markets/crypto/tickers?{}", configuration.base_path, query_string);

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

    fn v2_snapshot_locale_global_markets_crypto_tickers_ticker_book_get(&self, ticker: &str) -> Box<Result<InlineResponse20020, Error<serde_json::Value>>> {
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
        let uri_str = format!("{}/v2/snapshot/locale/global/markets/crypto/tickers/{ticker}/book?{}", configuration.base_path, query_string, ticker=ticker);

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

    fn v2_snapshot_locale_global_markets_crypto_tickers_ticker_get(&self, ticker: &str) -> Box<Result<InlineResponse20019, Error<serde_json::Value>>> {
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
        let uri_str = format!("{}/v2/snapshot/locale/global/markets/crypto/tickers/{ticker}?{}", configuration.base_path, query_string, ticker=ticker);

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
