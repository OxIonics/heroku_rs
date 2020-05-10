//Anything related to GET requests for oauth authorizations and it's properties goes here.
use super::{OAuth, OAuthClient};

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// OAuth Authorization Info
///
/// Info for an OAuth authorization.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#oauth-authorization-info)
pub struct OAuthDetails<'a> {
    /// oauth_id is the unique identifier.
    pub oauth_id: &'a str,
}

impl<'a> OAuthDetails<'a> {
    pub fn new(oauth_id: &'a str) -> OAuthDetails<'a> {
        OAuthDetails { oauth_id }
    }
}

impl<'a> HerokuEndpoint<OAuth> for OAuthDetails<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("oauth/authorizations/{}", self.oauth_id)
    }
}

/// OAuth Authorization List
///
/// List OAuth authorizations.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#oauth-authorization-list)
pub struct OAuthList {}

impl OAuthList {
    pub fn new() -> OAuthList {
        OAuthList {}
    }
}

impl HerokuEndpoint<Vec<OAuth>> for OAuthList {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("oauth/authorizations")
    }
}

/// OAuth Client Info
///
/// Info for an OAuth client
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#oauth-client-info)
pub struct OAuthClientDetails<'a> {
    /// unique identifier of OAuth Client authorization
    pub client_id: &'a str,
}

impl<'a> OAuthClientDetails<'a> {
    pub fn new(client_id: &'a str) -> OAuthClientDetails<'a> {
        OAuthClientDetails { client_id }
    }
}

impl<'a> HerokuEndpoint<OAuthClient> for OAuthClientDetails<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("oauth/clients/{}", self.client_id)
    }
}

/// OAuth Client List
///
/// List OAuth clients
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#oauth-client-list)
pub struct OAuthClientList {}

impl OAuthClientList {
    pub fn new() -> OAuthClientList {
        OAuthClientList {}
    }
}

impl HerokuEndpoint<Vec<OAuthClient>> for OAuthClientList {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("oauth/clients")
    }
}
