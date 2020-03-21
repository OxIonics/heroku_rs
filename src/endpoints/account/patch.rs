//Anything related to PATCH requests for account and it's properties goes here.
use super::{Account, AccountFeature};

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Update account.
/// https://devcenter.heroku.com/articles/platform-api-reference#account-update
pub struct AccountUpdate {
    pub params: AccountUpdateParams,
}

/// Update account with parameters.
/// All three paramemters are optional.
/// https://devcenter.heroku.com/articles/platform-api-reference#account-update-optional-parameters
#[derive(Serialize, Clone, Debug)]
pub struct AccountUpdateParams {
    /// whether to allow third party web activity tracking, by default: true
    pub allow_tracking: Option<bool>,
    /// whether allowed to utilize beta Heroku features
    pub beta: Option<bool>,
    /// full name of the account owner
    pub name: Option<String>,
}

impl HerokuEndpoint<Account, (), AccountUpdateParams> for AccountUpdate {
    fn method(&self) -> Method {
        Method::Patch
    }
    fn path(&self) -> String {
        format!("account",)
    }
    fn body(&self) -> Option<AccountUpdateParams> {
        Some(self.params.clone())
    }
}

/// Update user account.
/// https://devcenter.heroku.com/articles/platform-api-reference#account-update-by-user
pub struct UserAccountUpdate {
    /// identifier can be the account email or id.
    pub account_identifier: String,
    /// The parameters to pass to the Heroku API
    pub params: UserAccountUpdateParams,
}

/// Update user account with parameters.
/// All three paramemters are optional.
/// https://devcenter.heroku.com/articles/platform-api-reference#account-update-by-user-optional-parameters
#[derive(Serialize, Clone, Debug)]
pub struct UserAccountUpdateParams {
    /// whether to allow third party web activity tracking, by default: true
    pub allow_tracking: Option<bool>,
    /// whether allowed to utilize beta Heroku features
    pub beta: Option<bool>,
    /// full name of the account owner
    pub name: Option<String>,
}

impl HerokuEndpoint<Account, (), UserAccountUpdateParams> for UserAccountUpdate {
    fn method(&self) -> Method {
        Method::Patch
    }
    fn path(&self) -> String {
        format!("users/{}", self.account_identifier)
    }
    fn body(&self) -> Option<UserAccountUpdateParams> {
        Some(self.params.clone())
    }
}

/// Update account feature.
/// https://devcenter.heroku.com/articles/platform-api-reference#account-feature-update
pub struct AccountFeatureUpdate {
    /// identifier can be the feature name or id.
    pub account_feature_id: String,
    /// The parameters to pass to the Heroku API
    pub params: AccountFeatureUpdateParams,
}

/// Update account feature with parameters.
/// https://devcenter.heroku.com/articles/platform-api-reference#account-feature-update-required-parameters
#[derive(Serialize, Clone, Debug)]
pub struct AccountFeatureUpdateParams {
    /// whether or not account feature has been enabled
    pub enabled: bool,
}

impl HerokuEndpoint<AccountFeature, (), AccountFeatureUpdateParams> for AccountFeatureUpdate {
    fn method(&self) -> Method {
        Method::Patch
    }
    fn path(&self) -> String {
        format!("account/features/{}", self.account_feature_id)
    }
    fn body(&self) -> Option<AccountFeatureUpdateParams> {
        Some(self.params.clone())
    }
}
