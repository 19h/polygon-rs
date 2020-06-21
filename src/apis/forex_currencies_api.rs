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

pub struct ForexCurrenciesApiClient<C: hyper::client::connect::Connect + Clone + Send + Sync> {
    configuration: Arc<configuration::Configuration<C>>,
}

impl<C: hyper::client::connect::Connect + Clone + Send + Sync + 'static> ForexCurrenciesApiClient<C> {
    pub fn new(configuration: Arc<configuration::Configuration<C>>) -> ForexCurrenciesApiClient<C> {
        ForexCurrenciesApiClient {
            configuration: configuration,
        }
    }
}

pub trait ForexCurrenciesApi {
    fn v1_conversion_from_to_get(&self, from: &str, to: &str, amount: i64, precision: i64) -> Box<Result<InlineResponse20012, Error<serde_json::Value>>>;
    fn v1_historic_forex_from_to_date_get(&self, from: &str, to: &str, date: String, offset: i64, limit: i64) -> Box<Result<InlineResponse20011, Error<serde_json::Value>>>;
    fn v1_last_quote_currencies_from_to_get(&self, from: &str, to: &str) -> Box<Result<InlineResponse20013, Error<serde_json::Value>>>;
    fn v2_aggs_grouped_locale_locale_market_market_date_get(&self, locale: &str, market: &str, date: &str, unadjusted: bool) -> Box<Result<AggResponse, Error<serde_json::Value>>>;
    fn v2_aggs_ticker_ticker_prev_get(&self, ticker: &str, unadjusted: bool) -> Box<Result<AggResponse, Error<serde_json::Value>>>;
    fn v2_aggs_ticker_ticker_range_multiplier_timespan_from_to_get(&self, ticker: &str, multiplier: f32, timespan: &str, from: &str, to: &str, unadjusted: bool, sort: String) -> Box<Result<AggResponse, Error<serde_json::Value>>>;
    fn v2_snapshot_locale_global_markets_forex_direction_get(&self, direction: &str) -> Box<Result<InlineResponse20014, Error<serde_json::Value>>>;
    fn v2_snapshot_locale_global_markets_forex_tickers_get(&self, ) -> Box<Result<InlineResponse20014, Error<serde_json::Value>>>;
}


impl<C: hyper::client::connect::Connect + Clone + Send + Sync + 'static>ForexCurrenciesApi for ForexCurrenciesApiClient<C> {
    fn v1_conversion_from_to_get(&self, from: &str, to: &str, amount: i64, precision: i64) -> Box<Result<InlineResponse20012, Error<serde_json::Value>>> {
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
            query.append_pair("amount", &amount.outline_print() );
            query.append_pair("precision", &precision.outline_print() );
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/v1/conversion/{from}/{to}?{}", configuration.base_path, query_string, from=from, to=to);

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

    fn v1_historic_forex_from_to_date_get(&self, from: &str, to: &str, date: String, offset: i64, limit: i64) -> Box<Result<InlineResponse20011, Error<serde_json::Value>>> {
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
        let uri_str = format!("{}/v1/historic/forex/{from}/{to}/{date}?{}", configuration.base_path, query_string, from=from, to=to, date=date);

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

    fn v1_last_quote_currencies_from_to_get(&self, from: &str, to: &str) -> Box<Result<InlineResponse20013, Error<serde_json::Value>>> {
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
        let uri_str = format!("{}/v1/last_quote/currencies/{from}/{to}?{}", configuration.base_path, query_string, from=from, to=to);

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

    fn v2_snapshot_locale_global_markets_forex_direction_get(&self, direction: &str) -> Box<Result<InlineResponse20014, Error<serde_json::Value>>> {
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
        let uri_str = format!("{}/v2/snapshot/locale/global/markets/forex/{direction}?{}", configuration.base_path, query_string, direction=direction);

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

    fn v2_snapshot_locale_global_markets_forex_tickers_get(&self, ) -> Box<Result<InlineResponse20014, Error<serde_json::Value>>> {
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
        let uri_str = format!("{}/v2/snapshot/locale/global/markets/forex/tickers?{}", configuration.base_path, query_string);

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
