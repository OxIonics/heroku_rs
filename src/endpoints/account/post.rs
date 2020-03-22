//Anything related to POST requests for account and it's properties goes here.
use super::{AppTransfer, Credit};

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// App Transfer Create
/// https://devcenter.heroku.com/articles/platform-api-reference#app-transfer-create
pub struct AppTransferCreate {
    pub params: AppTransferCreateParams,
}

/// Update account with parameters.
/// https://devcenter.heroku.com/articles/platform-api-reference#app-transfer-create-required-parameters
#[derive(Serialize, Clone, Debug)]
pub struct AppTransferCreateParams {
    /// unique identifier or name of app
    pub app: String,
    /// unique email address, identifier of an account or implicit reference to currently authorized user
    pub recipient: String,
    /// whether to suppress email notification when transferring apps
    pub silent: Option<bool>,
}

impl HerokuEndpoint<AppTransfer, (), AppTransferCreateParams> for AppTransferCreate {
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!("account/app-transfers")
    }
    fn body(&self) -> Option<AppTransferCreateParams> {
        Some(self.params.clone())
    }
}

/// Credit Create
/// Create a new credit.
/// https://devcenter.heroku.com/articles/platform-api-reference#credit-create
pub struct AccountCreditCreate {
    pub params: AccountCreditCreateParams,
}

/// Update account credits with parameters.
/// https://devcenter.heroku.com/articles/platform-api-reference#credit-create-optional-parameters
#[derive(Serialize, Clone, Debug)]
pub struct AccountCreditCreateParams {
    /// first code from a discount card
    pub code1: Option<String>,
    /// second code from a discount card
    pub code2: Option<String>,
}

impl HerokuEndpoint<Credit, (), AccountCreditCreateParams> for AccountCreditCreate {
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!("account/credits")
    }
    fn body(&self) -> Option<AccountCreditCreateParams> {
        Some(self.params.clone())
    }
}
