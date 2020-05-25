//Anything related to DELETE requests for config vars and it's variations goes here.

use crate::framework::endpoint::{HerokuEndpoint, Method};
use std::collections::HashMap;

/// Config Vars DELETE
///
/// Delete config-vars for an app. You delete the config vars by setting the value to `None`.
///
/// There is no endpoint for this DELETE request, because it's done through a [PATCH](https://devcenter.heroku.com/articles/platform-api-reference#config-vars-update) request, by just setting the `value` to null/None. Separated into it's own file for clarity sakes.
/// 
/// # Example:
///
/// AppConfigVarDelete takes two required parameters, app_id and delete_key, and returns a `HashMap<String, Option<String>`.
/// ```rust
/// use heroku_rs::prelude::*;
/// 
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let response = api_client.request(&AppConfigVarDelete::create(
///     app_id,
///     String::from("key_to_delete"),
/// ));
///
///match response {
///     Ok(success) => println!("Success: {:#?}", success),
///     Err(e) => println!("Error: {}", e),
///}
//
/// ```
/// See how to create the Heroku [`api_client`][httpApiClientConfig].
///
/// [httpApiClientConfig]: ../../../framework/struct.HttpApiClient.html
pub struct AppConfigVarDelete<'a> {
    /// app_id is the unique app identifier.
    pub app_id: &'a str,
    /// The parameters to pass to the Heroku API
    pub params: HashMap<String, Option<String>>,
}

#[cfg(feature = "builder")]
impl<'a> AppConfigVarDelete<'a> {
    pub fn new(app_id: &'a str, params: HashMap<String, Option<String>>) -> AppConfigVarDelete {
        AppConfigVarDelete { app_id, params }
    }

    pub fn create(app_id: &'a str, delete_key: String) -> AppConfigVarDelete<'a> {
        let mut params = HashMap::new();
        params.insert(delete_key, None);

        AppConfigVarDelete { app_id, params }
    }
}

impl<'a> HerokuEndpoint<HashMap<String, Option<String>>, (), HashMap<String, Option<String>>>
    for AppConfigVarDelete<'a>
{
    fn method(&self) -> Method {
        Method::Patch
    }
    fn path(&self) -> String {
        format!("apps/{}/config-vars", self.app_id)
    }
    fn body(&self) -> Option<HashMap<String, Option<String>>> {
        Some(self.params.clone())
    }
}

/// Pipeline Config Vars DELETE
///
/// Delete config-vars for a pipeline. You delete the config vars by setting the value to `None`.
///
/// There is no endpoint for this DELETE request, because it's done through a [PATCH](https://devcenter.heroku.com/articles/platform-api-reference#pipeline-config-vars-update) request, by just setting the `value` to null/None. Separated into it's own file for clarity sakes.
/// 
/// # Example:
///
/// PipelineConfigVarDelete takes three required parameters, app_id, stage_id and delete_key, and returns a `HashMap<String, Option<String>`.
/// ```rust
/// use heroku_rs::prelude::*;
/// use std::collections::HashMap;
/// 
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// 
/// let response = api_client.request(&PipelineConfigVarDelete::create(
///     "PIPELINE_ID",
///     "STAGE_ID",
///     String::from("key_to_delete"),
/// ));
///
///match response {
///     Ok(success) => println!("Success: {:#?}", success),
///     Err(e) => println!("Error: {}", e),
///}
//
/// ```
/// See how to create the Heroku [`api_client`][httpApiClientConfig].
///
/// [httpApiClientConfig]: ../../../framework/struct.HttpApiClient.html
pub struct PipelineConfigVarDelete<'a> {
    /// pipeline_id is the unique pipeline identifier.
    pub pipeline_id: &'a str,
    /// pipeline coupling stage
    pub stage_id: &'a str,
    /// The parameters to pass to the Heroku API
    pub params: HashMap<String, Option<String>>,
}

#[cfg(feature = "builder")]
impl<'a> PipelineConfigVarDelete<'a> {
    pub fn new(
        pipeline_id: &'a str,
        stage_id: &'a str,
        params: HashMap<String, Option<String>>,
    ) -> PipelineConfigVarDelete<'a> {
        PipelineConfigVarDelete {
            pipeline_id,
            stage_id,
            params,
        }
    }

    pub fn create(
        pipeline_id: &'a str,
        stage_id: &'a str,
        delete_key: String,
    ) -> PipelineConfigVarDelete<'a> {
        let mut params = HashMap::new();
        params.insert(delete_key, None);

        PipelineConfigVarDelete {
            pipeline_id,
            stage_id,
            params,
        }
    }
}

impl<'a> HerokuEndpoint<HashMap<String, Option<String>>, (), HashMap<String, Option<String>>>
    for PipelineConfigVarDelete<'a>
{
    fn method(&self) -> Method {
        Method::Patch
    }
    fn path(&self) -> String {
        format!(
            "pipelines/{}/stage/{}/config-vars",
            self.pipeline_id, self.stage_id
        )
    }
    fn body(&self) -> Option<HashMap<String, Option<String>>> {
        Some(self.params.clone())
    }
}
