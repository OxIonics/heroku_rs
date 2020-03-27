//Anything related to POST requests for oauth authorizations and it's properties goes here.
use super::{OAuth, OAuthClient, OAuthToken};

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// OAuth Authorization Create
///
/// Create a new OAuth authorization.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#oauth-authorization-create)
pub struct OAuthCreate {
    /// The parameters to pass to the Heroku API
    pub params: OAuthCreateParams,
}

/// Create a new authorization with parameters.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#oauth-authorization-create-required-parameters)
#[derive(Serialize, Clone, Debug)]
pub struct OAuthCreateParams {
    /// The scope of access OAuth authorization allows
    pub scope: Vec<String>,
    /// unique identifier of this OAuth client
    pub client: Option<String>,
    /// human-friendly description of this OAuth authorization
    pub description: Option<String>,
    /// seconds until OAuth token expires; may be null for tokens with indefinite lifetime
    pub expires_in: Option<u32>,
}

impl HerokuEndpoint<OAuth, (), OAuthCreateParams> for OAuthCreate {
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!("oauth/authorizations")
    }
    fn body(&self) -> Option<OAuthCreateParams> {
        Some(self.params.clone())
    }
}

/// OAuth Authorization Regenerate
///
/// Regenerate OAuth tokens. This endpoint is only available to direct authorizations or privileged OAuth clients.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#oauth-authorization-regenerate)
pub struct OAuthRegenerate {
    /// unique identifier of OAuth authorization
    pub oauth_id: String,
}

impl HerokuEndpoint<OAuth, (), ()> for OAuthRegenerate {
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!(
            "oauth/authorizations/{}/actions/regenerate-tokens",
            self.oauth_id
        )
    }
}

/// OAuth Client Create
///
/// Create a new OAuth client.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#oauth-client-create)
pub struct OAuthClientCreate {
    /// The parameters to pass to the Heroku API
    pub params: OAuthClientCreateParams,
}

/// Create a new oauth client with parameters.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#oauth-client-create-required-parameters)
#[derive(Serialize, Clone, Debug)]
pub struct OAuthClientCreateParams {
    /// OAuth client name
    pub name: String,
    /// endpoint for redirection after authorization with OAuth client
    pub redirect_uri: String,
}

impl HerokuEndpoint<OAuthClient, (), OAuthClientCreateParams> for OAuthClientCreate {
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!("oauth/clients")
    }
    fn body(&self) -> Option<OAuthClientCreateParams> {
        Some(self.params.clone())
    }
}

/// OAuth Client Rotate Credentials
///
/// Rotate credentials for an OAuth client
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#oauth-client-rotate-credentials)
pub struct OAuthClientRotateCredentials {
    /// unique identifier of OAuth Client authorization
    pub client_id: String,
}

impl HerokuEndpoint<OAuthClient, (), ()> for OAuthClientRotateCredentials {
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!(
            "oauth/clients/{}/actions/rotate-credentials",
            self.client_id
        )
    }
}

/// OAuth Token Create
///
/// Create a new OAuth token.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#oauth-token-create)
pub struct OAuthTokenCreate {
    /// The parameters to pass to the Heroku API
    pub params: OAuthTokenCreateParams,
}

/// Create a new oauth token with parameters.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#oauth-token-create-required-parameters)
#[derive(Serialize, Clone, Debug)]
pub struct OAuthTokenCreateParams {
    /// OAuth client
    pub client: Client,
    /// OAuth grant
    pub grant: Grant,
    /// endpoint for redirection after authorization with OAuth client
    pub refresh_token: RefreshToken,
}

// TODO(ben): Find a better solution than this
///RefreshToken
#[derive(Serialize, Clone, Debug)]
pub struct RefreshToken {
    /// contents of the token to be used for authorization
    pub token: String,
}

// TODO(ben): Find a better solution than this
/// Grant
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Grant {
    /// grant code received from OAuth web application authorization
    pub code: String,
    /// type of grant requested, one of authorization_code or refresh_token
    #[serde(rename = "type")]
    pub type_field: String,
}

// TODO(ben): Find a better solution than this
/// OAuth client secret used to obtain token
#[derive(Serialize, Clone, Debug)]
pub struct Client {
    /// secret used to obtain OAuth authorizations under this client
    pub secret: String,
}

impl HerokuEndpoint<OAuthToken, (), OAuthTokenCreateParams> for OAuthTokenCreate {
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!("oauth/tokens")
    }
    fn body(&self) -> Option<OAuthTokenCreateParams> {
        Some(self.params.clone())
    }
}