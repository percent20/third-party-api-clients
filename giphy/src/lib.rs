//! A fully generated, opinionated API client library for Giphy.
//!
//! [![docs.rs](https://docs.rs/giphy-api/badge.svg)](https://docs.rs/giphy-api)
//!
//! ## API Details
//!
//! Giphy API
//!
//! [API Terms of Service](https://developers.giphy.com/)
//!
//! ### Contact
//!
//!
//! | email |
//! |----|
//! | support@giphy.com |
//!
//!
//!
//! ## Client Details
//!
//! This client is generated from the [Giphy OpenAPI
//! specs](https://github.com/APIs-guru/openapi-directory/tree/main/APIs/giphy.com) based on API spec version `1.0`. This way it will remain
//! up to date as features are added. The documentation for the crate is generated
//! along with the code to make this library easy to use.
//!
//!
//! To install the library, add the following to your `Cargo.toml` file.
//!
//! ```toml
//! [dependencies]
//! giphy-api = "0.3.0"
//! ```
//!
//! ## Basic example
//!
//! Typical use will require intializing a `Client`. This requires
//! a user agent string and set of credentials.
//!
//! ```
//! use giphy_api::Client;
//!
//! let giphy = Client::new(String::from("api-key"));
//! ```
//!
//! Alternatively, the library can search for most of the variables required for
//! the client in the environment:
//!
//! - `GIPHY_API_KEY`
//!
//! And then you can create a client from the environment.
//!
//! ```
//! use giphy_api::Client;
//!
//! let giphy = Client::new_from_env();
//! ```
#![allow(clippy::too_many_arguments)]
#![allow(clippy::nonstandard_macro_braces)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::tabs_in_doc_comments)]
#![allow(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod gifs;
pub mod stickers;
#[cfg(test)]
mod tests;
pub mod types;
#[doc(hidden)]
pub mod utils;

use anyhow::{anyhow, Error, Result};

pub const DEFAULT_HOST: &str = "https://api.giphy.com/v1";

mod progenitor_support {
    use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

    const PATH_SET: &AsciiSet = &CONTROLS
        .add(b' ')
        .add(b'"')
        .add(b'#')
        .add(b'<')
        .add(b'>')
        .add(b'?')
        .add(b'`')
        .add(b'{')
        .add(b'}');

    #[allow(dead_code)]
    pub(crate) fn encode_path(pc: &str) -> String {
        utf8_percent_encode(pc, PATH_SET).to_string()
    }
}

use std::env;

/// Entrypoint for interacting with the API client.
#[derive(Clone)]
pub struct Client {
    host: String,
    token: String,

    client: reqwest_middleware::ClientWithMiddleware,
}

impl Client {
    /// Create a new Client struct. It takes a type that can convert into
    /// an &str (`String` or `Vec<u8>` for example). As long as the function is
    /// given a valid API key your requests will work.
    pub fn new<T>(token: T) -> Self
    where
        T: ToString,
    {
        let client = reqwest::Client::builder().build();
        let retry_policy =
            reqwest_retry::policies::ExponentialBackoff::builder().build_with_max_retries(3);
        match client {
            Ok(c) => {
                let client = reqwest_middleware::ClientBuilder::new(c)
                    // Trace HTTP requests. See the tracing crate to make use of these traces.
                    .with(reqwest_tracing::TracingMiddleware::default())
                    // Retry failed requests.
                    .with(reqwest_conditional_middleware::ConditionalMiddleware::new(
                        reqwest_retry::RetryTransientMiddleware::new_with_policy(retry_policy),
                        |req: &reqwest::Request| req.try_clone().is_some(),
                    ))
                    .build();

                Client {
                    host: DEFAULT_HOST.to_string(),
                    token: token.to_string(),

                    client,
                }
            }
            Err(e) => panic!("creating reqwest client failed: {:?}", e),
        }
    }

    /// Override the default host for the client.
    pub fn with_host<H>(&self, host: H) -> Self
    where
        H: ToString,
    {
        let mut c = self.clone();
        c.host = host.to_string();
        c
    }

    /// Create a new Client struct from environment variables. It
    /// takes a type that can convert into
    /// an &str (`String` or `Vec<u8>` for example). As long as the function is
    /// given a valid API key and your requests will work.
    /// We pass in the token and refresh token to the client so if you are storing
    /// it in a database, you can get it first.
    pub fn new_from_env() -> Self {
        let token = env::var("GIPHY_API_KEY").expect("must set GIPHY_API_KEY");

        Client::new(token)
    }

    async fn url_and_auth(&self, uri: &str) -> Result<(reqwest::Url, Option<String>)> {
        let parsed_url = uri.parse::<reqwest::Url>();
        let auth = format!("Bearer {}", self.token);
        parsed_url.map(|u| (u, Some(auth))).map_err(Error::from)
    }

    async fn request_raw(
        &self,
        method: reqwest::Method,
        uri: &str,
        body: Option<reqwest::Body>,
    ) -> Result<reqwest::Response> {
        let u = if uri.starts_with("https://") {
            uri.to_string()
        } else {
            (self.host.clone() + uri).to_string()
        };
        let (url, auth) = self.url_and_auth(&u).await?;
        let instance = <&Client>::clone(&self);
        let mut req = instance.client.request(method.clone(), url);
        // Set the default headers.
        req = req.header(
            reqwest::header::ACCEPT,
            reqwest::header::HeaderValue::from_static("application/json"),
        );
        req = req.header(
            reqwest::header::CONTENT_TYPE,
            reqwest::header::HeaderValue::from_static("application/json"),
        );

        if let Some(auth_str) = auth {
            req = req.header(http::header::AUTHORIZATION, &*auth_str);
        }
        if let Some(body) = body {
            log::debug!(
                "body: {:?}",
                String::from_utf8(body.as_bytes().unwrap().to_vec()).unwrap()
            );
            req = req.body(body);
        }
        Ok(req.send().await?)
    }

    async fn request<Out>(
        &self,
        method: reqwest::Method,
        uri: &str,
        body: Option<reqwest::Body>,
    ) -> Result<Out>
    where
        Out: serde::de::DeserializeOwned + 'static + Send,
    {
        let response = self.request_raw(method, uri, body).await?;

        let status = response.status();

        let response_body = response.bytes().await?;

        if status.is_success() {
            log::debug!(
                "response payload {}",
                String::from_utf8_lossy(&response_body)
            );
            let parsed_response = if status == http::StatusCode::NO_CONTENT
                || std::any::TypeId::of::<Out>() == std::any::TypeId::of::<()>()
            {
                serde_json::from_str("null")
            } else {
                serde_json::from_slice::<Out>(&response_body)
            };
            parsed_response.map_err(Error::from)
        } else {
            let error = if response_body.is_empty() {
                anyhow!("code: {}, empty response", status)
            } else {
                anyhow!(
                    "code: {}, error: {:?}",
                    status,
                    String::from_utf8_lossy(&response_body),
                )
            };

            Err(error)
        }
    }

    async fn request_with_links<Out>(
        &self,
        method: http::Method,
        uri: &str,
        body: Option<reqwest::Body>,
    ) -> Result<(Option<hyperx::header::Link>, Out)>
    where
        Out: serde::de::DeserializeOwned + 'static + Send,
    {
        let response = self.request_raw(method, uri, body).await?;

        let status = response.status();
        let link = response
            .headers()
            .get(http::header::LINK)
            .and_then(|l| l.to_str().ok())
            .and_then(|l| l.parse().ok());

        let response_body = response.bytes().await?;

        if status.is_success() {
            log::debug!(
                "response payload {}",
                String::from_utf8_lossy(&response_body)
            );

            let parsed_response = if status == http::StatusCode::NO_CONTENT
                || std::any::TypeId::of::<Out>() == std::any::TypeId::of::<()>()
            {
                serde_json::from_str("null")
            } else {
                serde_json::from_slice::<Out>(&response_body)
            };
            parsed_response.map(|out| (link, out)).map_err(Error::from)
        } else {
            let error = if response_body.is_empty() {
                anyhow!("code: {}, empty response", status)
            } else {
                anyhow!(
                    "code: {}, error: {:?}",
                    status,
                    String::from_utf8_lossy(&response_body),
                )
            };
            Err(error)
        }
    }

    /* TODO: make this more DRY */
    #[allow(dead_code)]
    async fn post_form<Out>(&self, uri: &str, form: reqwest::multipart::Form) -> Result<Out>
    where
        Out: serde::de::DeserializeOwned + 'static + Send,
    {
        let u = if uri.starts_with("https://") {
            uri.to_string()
        } else {
            (self.host.clone() + uri).to_string()
        };
        let (url, auth) = self.url_and_auth(&u).await?;

        let instance = <&Client>::clone(&self);

        let mut req = instance.client.request(http::Method::POST, url);

        // Set the default headers.
        req = req.header(
            reqwest::header::ACCEPT,
            reqwest::header::HeaderValue::from_static("application/json"),
        );

        if let Some(auth_str) = auth {
            req = req.header(http::header::AUTHORIZATION, &*auth_str);
        }

        log::debug!("form: {:?}", form);
        req = req.multipart(form);

        let response = req.send().await?;

        let status = response.status();

        let response_body = response.bytes().await?;

        if status.is_success() {
            log::debug!(
                "response payload {}",
                String::from_utf8_lossy(&response_body)
            );
            let parsed_response = if status == http::StatusCode::NO_CONTENT
                || std::any::TypeId::of::<Out>() == std::any::TypeId::of::<()>()
            {
                serde_json::from_str("null")
            } else if std::any::TypeId::of::<Out>() == std::any::TypeId::of::<String>() {
                // Parse the output as a string.
                serde_json::from_value(serde_json::json!(&String::from_utf8(
                    response_body.to_vec()
                )?))
            } else {
                serde_json::from_slice::<Out>(&response_body)
            };
            parsed_response.map_err(Error::from)
        } else {
            let error = if response_body.is_empty() {
                anyhow!("code: {}, empty response", status)
            } else {
                anyhow!(
                    "code: {}, error: {:?}",
                    status,
                    String::from_utf8_lossy(&response_body),
                )
            };

            Err(error)
        }
    }

    /* TODO: make this more DRY */
    #[allow(dead_code)]
    async fn request_with_accept_mime<Out>(
        &self,
        method: reqwest::Method,
        uri: &str,
        accept_mime_type: &str,
    ) -> Result<Out>
    where
        Out: serde::de::DeserializeOwned + 'static + Send,
    {
        let u = if uri.starts_with("https://") {
            uri.to_string()
        } else {
            (self.host.clone() + uri).to_string()
        };
        let (url, auth) = self.url_and_auth(&u).await?;

        let instance = <&Client>::clone(&self);

        let mut req = instance.client.request(method, url);

        // Set the default headers.
        req = req.header(
            reqwest::header::ACCEPT,
            reqwest::header::HeaderValue::from_str(accept_mime_type)?,
        );

        if let Some(auth_str) = auth {
            req = req.header(http::header::AUTHORIZATION, &*auth_str);
        }

        let response = req.send().await?;

        let status = response.status();

        let response_body = response.bytes().await?;

        if status.is_success() {
            log::debug!(
                "response payload {}",
                String::from_utf8_lossy(&response_body)
            );
            let parsed_response = if status == http::StatusCode::NO_CONTENT
                || std::any::TypeId::of::<Out>() == std::any::TypeId::of::<()>()
            {
                serde_json::from_str("null")
            } else if std::any::TypeId::of::<Out>() == std::any::TypeId::of::<String>() {
                // Parse the output as a string.
                serde_json::from_value(serde_json::json!(&String::from_utf8(
                    response_body.to_vec()
                )?))
            } else {
                serde_json::from_slice::<Out>(&response_body)
            };
            parsed_response.map_err(Error::from)
        } else {
            let error = if response_body.is_empty() {
                anyhow!("code: {}, empty response", status)
            } else {
                anyhow!(
                    "code: {}, error: {:?}",
                    status,
                    String::from_utf8_lossy(&response_body),
                )
            };

            Err(error)
        }
    }

    /* TODO: make this more DRY */
    #[allow(dead_code)]
    async fn request_with_mime<Out>(
        &self,
        method: reqwest::Method,
        uri: &str,
        content: &[u8],
        mime_type: &str,
    ) -> Result<Out>
    where
        Out: serde::de::DeserializeOwned + 'static + Send,
    {
        let u = if uri.starts_with("https://") {
            uri.to_string()
        } else {
            (self.host.clone() + uri).to_string()
        };
        let (url, auth) = self.url_and_auth(&u).await?;

        let instance = <&Client>::clone(&self);

        let mut req = instance.client.request(method, url);

        // Set the default headers.
        req = req.header(
            reqwest::header::ACCEPT,
            reqwest::header::HeaderValue::from_static("application/json"),
        );
        req = req.header(
            reqwest::header::CONTENT_TYPE,
            reqwest::header::HeaderValue::from_bytes(mime_type.as_bytes()).unwrap(),
        );
        // We are likely uploading a file so add the right headers.
        req = req.header(
            reqwest::header::HeaderName::from_static("x-upload-content-type"),
            reqwest::header::HeaderValue::from_static("application/octet-stream"),
        );
        req = req.header(
            reqwest::header::HeaderName::from_static("x-upload-content-length"),
            reqwest::header::HeaderValue::from_bytes(format!("{}", content.len()).as_bytes())
                .unwrap(),
        );

        if let Some(auth_str) = auth {
            req = req.header(http::header::AUTHORIZATION, &*auth_str);
        }

        if content.len() > 1 {
            let b = bytes::Bytes::copy_from_slice(content);
            // We are uploading a file so add that as the body.
            req = req.body(b);
        }

        let response = req.send().await?;

        let status = response.status();

        let response_body = response.bytes().await?;

        if status.is_success() {
            log::debug!(
                "response payload {}",
                String::from_utf8_lossy(&response_body)
            );
            let parsed_response = if status == http::StatusCode::NO_CONTENT
                || std::any::TypeId::of::<Out>() == std::any::TypeId::of::<()>()
            {
                serde_json::from_str("null")
            } else {
                serde_json::from_slice::<Out>(&response_body)
            };
            parsed_response.map_err(Error::from)
        } else {
            let error = if response_body.is_empty() {
                anyhow!("code: {}, empty response", status)
            } else {
                anyhow!(
                    "code: {}, error: {:?}",
                    status,
                    String::from_utf8_lossy(&response_body),
                )
            };

            Err(error)
        }
    }

    async fn request_entity<D>(
        &self,
        method: http::Method,
        uri: &str,
        body: Option<reqwest::Body>,
    ) -> Result<D>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        let r = self.request(method, uri, body).await?;
        Ok(r)
    }

    #[allow(dead_code)]
    async fn get<D>(&self, uri: &str, message: Option<reqwest::Body>) -> Result<D>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request_entity(http::Method::GET, &(self.host.to_string() + uri), message)
            .await
    }

    #[allow(dead_code)]
    async fn get_all_pages<D>(&self, uri: &str, _message: Option<reqwest::Body>) -> Result<Vec<D>>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        // TODO: implement this.
        self.unfold(uri).await
    }

    /// "unfold" paginated results of a vector of items
    #[allow(dead_code)]
    async fn unfold<D>(&self, uri: &str) -> Result<Vec<D>>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        let mut global_items = Vec::new();
        let (new_link, mut items) = self.get_pages(uri).await?;
        let mut link = new_link;
        while !items.is_empty() {
            global_items.append(&mut items);
            // We need to get the next link.
            if let Some(url) = link.as_ref().and_then(crate::utils::next_link) {
                let url = reqwest::Url::parse(&url)?;
                let (new_link, new_items) = self.get_pages_url(&url).await?;
                link = new_link;
                items = new_items;
            }
        }

        Ok(global_items)
    }

    #[allow(dead_code)]
    async fn get_pages<D>(&self, uri: &str) -> Result<(Option<hyperx::header::Link>, Vec<D>)>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request_with_links(http::Method::GET, &(self.host.to_string() + uri), None)
            .await
    }

    #[allow(dead_code)]
    async fn get_pages_url<D>(
        &self,
        url: &reqwest::Url,
    ) -> Result<(Option<hyperx::header::Link>, Vec<D>)>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request_with_links(http::Method::GET, url.as_str(), None)
            .await
    }

    #[allow(dead_code)]
    async fn post<D>(&self, uri: &str, message: Option<reqwest::Body>) -> Result<D>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request_entity(http::Method::POST, &(self.host.to_string() + uri), message)
            .await
    }

    #[allow(dead_code)]
    async fn patch<D>(&self, uri: &str, message: Option<reqwest::Body>) -> Result<D>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request_entity(http::Method::PATCH, &(self.host.to_string() + uri), message)
            .await
    }

    #[allow(dead_code)]
    async fn put<D>(&self, uri: &str, message: Option<reqwest::Body>) -> Result<D>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request_entity(http::Method::PUT, &(self.host.to_string() + uri), message)
            .await
    }

    #[allow(dead_code)]
    async fn delete<D>(&self, uri: &str, message: Option<reqwest::Body>) -> Result<D>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request_entity(
            http::Method::DELETE,
            &(self.host.to_string() + uri),
            message,
        )
        .await
    }

    /// Return a reference to an interface that provides access to gifs operations.
    pub fn gifs(&self) -> gifs::Gifs {
        gifs::Gifs::new(self.clone())
    }

    /// Return a reference to an interface that provides access to stickers operations.
    pub fn stickers(&self) -> stickers::Stickers {
        stickers::Stickers::new(self.clone())
    }
}
