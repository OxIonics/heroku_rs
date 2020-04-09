//Anything related to POST requests for collaborators and it's properties goes here.
use super::{Collaborator, TeamCollaborator};

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Collaborator Create
///
/// Create a new collaborator.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#collaborator-create)
pub struct CollaboratorCreate {
    /// app_id can be the app name or the app id
    pub app_id: String,
    /// The parameters to pass to the Heroku API
    pub params: CollaboratorCreateParams,
}

impl CollaboratorCreate {
    pub fn new(app_id: String, user: String, silent: Option<bool>) -> CollaboratorCreate {
        CollaboratorCreate {
            app_id,
            params: CollaboratorCreateParams { user, silent },
        }
    }

    pub fn create(app_id: String, user: String) -> CollaboratorCreate {
        CollaboratorCreate {
            app_id,
            params: CollaboratorCreateParams {
                user: user,
                silent: None,
            },
        }
    }
}

/// Create collaborator with parameters.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#collaborator-create-required-parameters)
#[derive(Serialize, Clone, Debug)]
pub struct CollaboratorCreateParams {
    /// unique email address, identifier of an account or Implicit reference to currently authorized user
    pub user: String,
    /// whether to suppress email invitation when creating collaborator
    #[serde(skip_serializing_if = "Option::is_none")]
    pub silent: Option<bool>,
}

impl HerokuEndpoint<Collaborator, (), CollaboratorCreateParams> for CollaboratorCreate {
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!("apps/{}/collaborators", self.app_id)
    }
    fn body(&self) -> Option<CollaboratorCreateParams> {
        Some(self.params.clone())
    }
}

/// Team App Collaborator Create
///
/// Create a new collaborator on a team app. Use this endpoint instead of the /apps/{app_id_or_name}/collaborator endpoint when you want the collaborator to be granted permissions according to their role in the team.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#team-app-collaborator-create)
pub struct TeamCollaboratorCreate<'a> {
    /// app_id can be the app name or the app id
    pub app_id: &'a str,
    /// The parameters to pass to the Heroku API
    pub params: TeamCollaboratorCreateParams<'a>,
}

impl<'a> TeamCollaboratorCreate<'a> {
    pub fn new(
        app_id: &'a str,
        user: &'a str,
        silent: Option<bool>,
        permissions: Option<Vec<&'a str>>,
    ) -> TeamCollaboratorCreate<'a> {
        TeamCollaboratorCreate {
            app_id,
            params: TeamCollaboratorCreateParams {
                user,
                silent,
                permissions,
            },
        }
    }

    pub fn create(app_id: &'a str, user: &'a str) -> TeamCollaboratorCreate<'a> {
        TeamCollaboratorCreate {
            app_id,
            params: TeamCollaboratorCreateParams {
                user: user,
                silent: None,
                permissions: None,
            },
        }
    }
}

/// Create team app collaborator with parameters.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#team-app-collaborator-create-required-parameters)
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug)]
pub struct TeamCollaboratorCreateParams<'a> {
    /// unique email address, identifier of an account or Implicit reference to currently authorized user
    pub user: &'a str,
    /// whether to suppress email invitation when creating collaborator
    pub silent: Option<bool>,
    /// An array of permissions to give to the collaborator.
    pub permissions: Option<Vec<&'a str>>,
}

impl<'a> HerokuEndpoint<TeamCollaborator, (), TeamCollaboratorCreateParams<'a>>
    for TeamCollaboratorCreate<'a>
{
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!("teams/apps/{}/collaborators", self.app_id)
    }
    fn body(&self) -> Option<TeamCollaboratorCreateParams<'a>> {
        Some(self.params.clone())
    }
}
