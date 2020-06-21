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

pub struct StocksEquitiesApiClient<C: hyper::client::connect::Connect + Clone + Send + Sync> {
    configuration: Arc<configuration::Configuration<C>>,
}

impl<C: hyper::client::connect::Connect + Clone + Send + Sync + 'static> StocksEquitiesApiClient<C> {
    pub fn new(configuration: Arc<configuration::Configuration<C>>) -> StocksEquitiesApiClient<C> {
        StocksEquitiesApiClient {
            configuration: configuration,
        }
    }
}

pub trait StocksEquitiesApi {
    fn v1_last_quote_stocks_symbol_get(&self, symbol: &str) -> Box<Result<InlineResponse2008, Error<serde_json::Value>>>;
    fn v1_last_stocks_symbol_get(&self, symbol: &str) -> Box<Result<InlineResponse2007, Error<serde_json::Value>>>;
    fn v1_meta_conditions_ticktype_get(&self, ticktype: &str) -> Box<Result<ConditionTypeMap, Error<serde_json::Value>>>;
    fn v1_meta_exchanges_get(&self, ) -> Box<Result<Vec<Exchange>, Error<serde_json::Value>>>;
    fn v1_open_close_symbol_date_get(&self, symbol: &str, date: String) -> Box<Result<StocksOpenClose, Error<serde_json::Value>>>;
    fn v2_aggs_grouped_locale_locale_market_market_date_get(&self, locale: &str, market: &str, date: &str, unadjusted: bool) -> Box<Result<AggResponse, Error<serde_json::Value>>>;
    fn v2_aggs_ticker_ticker_prev_get(&self, ticker: &str, unadjusted: bool) -> Box<Result<AggResponse, Error<serde_json::Value>>>;
    fn v2_aggs_ticker_ticker_range_multiplier_timespan_from_to_get(&self, ticker: &str, multiplier: f32, timespan: &str, from: &str, to: &str, unadjusted: bool, sort: String) -> Box<Result<AggResponse, Error<serde_json::Value>>>;
    fn v2_snapshot_locale_us_markets_stocks_direction_get(&self, direction: &str) -> Box<Result<InlineResponse2009, Error<serde_json::Value>>>;
    fn v2_snapshot_locale_us_markets_stocks_tickers_get(&self, ) -> Box<Result<InlineResponse2009, Error<serde_json::Value>>>;
    fn v2_snapshot_locale_us_markets_stocks_tickers_ticker_get(&self, ticker: &str) -> Box<Result<InlineResponse20010, Error<serde_json::Value>>>;
    fn v2_ticks_stocks_nbbo_ticker_date_get(&self, ticker: &str, date: String, timestamp: i64, timestamp_limit: i64, reverse: bool, limit: i64) -> Box<Result<InlineResponse2006, Error<serde_json::Value>>>;
    fn v2_ticks_stocks_trades_ticker_date_get(&self, ticker: &str, date: String, timestamp: i64, timestamp_limit: i64, reverse: bool, limit: i64) -> Box<Result<InlineResponse2005, Error<serde_json::Value>>>;
}


impl<C: hyper::client::connect::Connect + Clone + Send + Sync + 'static>StocksEquitiesApi for StocksEquitiesApiClient<C> {
    fn v1_last_quote_stocks_symbol_get(&self, symbol: &str) -> Box<Result<InlineResponse2008, Error<serde_json::Value>>> {
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
        let uri_str = format!("{}/v1/last_quote/stocks/{symbol}?{}", configuration.base_path, query_string, symbol=symbol);

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

    fn v1_last_stocks_symbol_get(&self, symbol: &str) -> Box<Result<InlineResponse2007, Error<serde_json::Value>>> {
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
        let uri_str = format!("{}/v1/last/stocks/{symbol}?{}", configuration.base_path, query_string, symbol=symbol);

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

    fn v1_meta_conditions_ticktype_get(&self, ticktype: &str) -> Box<Result<ConditionTypeMap, Error<serde_json::Value>>> {
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
        let uri_str = format!("{}/v1/meta/conditions/{ticktype}?{}", configuration.base_path, query_string, ticktype=ticktype);

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

    fn v1_meta_exchanges_get(&self, ) -> Box<Result<Vec<Exchange>, Error<serde_json::Value>>> {
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
        let uri_str = format!("{}/v1/meta/exchanges?{}", configuration.base_path, query_string);

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

    fn v1_open_close_symbol_date_get(&self, symbol: &str, date: String) -> Box<Result<StocksOpenClose, Error<serde_json::Value>>> {
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
        let uri_str = format!("{}/v1/open-close/{symbol}/{date}?{}", configuration.base_path, query_string, symbol=symbol, date=date);

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

    fn v2_snapshot_locale_us_markets_stocks_direction_get(&self, direction: &str) -> Box<Result<InlineResponse2009, Error<serde_json::Value>>> {
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
        let uri_str = format!("{}/v2/snapshot/locale/us/markets/stocks/{direction}?{}", configuration.base_path, query_string, direction=direction);

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

    fn v2_snapshot_locale_us_markets_stocks_tickers_get(&self, ) -> Box<Result<InlineResponse2009, Error<serde_json::Value>>> {
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
        let uri_str = format!("{}/v2/snapshot/locale/us/markets/stocks/tickers?{}", configuration.base_path, query_string);

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

    fn v2_snapshot_locale_us_markets_stocks_tickers_ticker_get(&self, ticker: &str) -> Box<Result<InlineResponse20010, Error<serde_json::Value>>> {
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
        let uri_str = format!("{}/v2/snapshot/locale/us/markets/stocks/tickers/{ticker}?{}", configuration.base_path, query_string, ticker=ticker);

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

    fn v2_ticks_stocks_nbbo_ticker_date_get(&self, ticker: &str, date: String, timestamp: i64, timestamp_limit: i64, reverse: bool, limit: i64) -> Box<Result<InlineResponse2006, Error<serde_json::Value>>> {
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
            query.append_pair("timestamp", &timestamp.outline_print() );
            query.append_pair("timestampLimit", &timestamp_limit.outline_print() );
            query.append_pair("reverse", &reverse.outline_print() );
            query.append_pair("limit", &limit.outline_print() );
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/v2/ticks/stocks/nbbo/{ticker}/{date}?{}", configuration.base_path, query_string, ticker=ticker, date=date);

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

    fn v2_ticks_stocks_trades_ticker_date_get(&self, ticker: &str, date: String, timestamp: i64, timestamp_limit: i64, reverse: bool, limit: i64) -> Box<Result<InlineResponse2005, Error<serde_json::Value>>> {
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
            query.append_pair("timestamp", &timestamp.outline_print() );
            query.append_pair("timestampLimit", &timestamp_limit.outline_print() );
            query.append_pair("reverse", &reverse.outline_print() );
            query.append_pair("limit", &limit.outline_print() );
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/v2/ticks/stocks/trades/{ticker}/{date}?{}", configuration.base_path, query_string, ticker=ticker, date=date);

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
