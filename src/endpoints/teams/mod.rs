use crate::framework::response::ApiResult;

pub mod delete;
pub mod get;
pub mod patch;
pub mod post;
pub mod put;

pub use delete::TeamDelete;
pub use get::{TeamAppDetails, TeamDetails, TeamList, TeamListByEA, TeamAppList};
pub use patch::{TeamAppUpdateLocked, TeamAppUpdateLockedParams, TeamUpdate, TeamUpdateParams, TeamAppTransfer, TeamAppTransferParams};
pub use post::{
    TeamAppCreate, TeamAppCreateParams, TeamCreate, TeamCreateByEA,
    TeamCreateByEAParams, TeamCreateOptionalParams, TeamCreateParams,
};

impl ApiResult for Team {}
impl ApiResult for Vec<Team> {}

impl ApiResult for TeamApp {}
impl ApiResult for Vec<TeamApp> {}

pub use team::Team;
pub use team_app::TeamApp;

mod team {
    use chrono::offset::Utc;
    use chrono::DateTime;

    /// Team
    ///
    /// Stability: development
    ///
    /// Teams allow you to manage access to a shared group of applications and other resources.
    ///
    /// [For more information please refer to the Heroku documentation](https://devcenter.heroku.com/articles/platform-api-reference#team)
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
    pub struct Team {
        pub id: String,
        /// when the team was created
        pub created_at: DateTime<Utc>,
        /// whether charges incurred by the team are paid by credit card.
        pub credit_card_collections: bool,
        /// whether to use this team when none is specified
        pub default: bool,
        /// Entererprise account associated with the Team
        pub enterprise_account: Option<EnterpriseAccount>,
        /// Identity Provider associated with the Team
        pub identity_provider: Option<IdentityProvider>,
        /// upper limit of members allowed in a team.
        pub membership_limit: Option<i64>,
        /// unique name of team
        pub name: String,
        /// whether the team is provisioned licenses by salesforce.
        pub provisioned_licenses: bool,
        /// role in the team
        /// one of:"admin" or "collaborator" or "member" or "owner" or null(None)
        pub role: Option<String>,
        /// type of team.
        /// one of:"enterprise" or "team"
        #[serde(rename = "type")]
        pub type_field: String,
        /// when the team was updated
        pub updated_at: DateTime<Utc>,
    }

    /// Entererprise account associated with the Team
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
    pub struct EnterpriseAccount {
        /// unique identifier of the enterprise account
        pub id: String,
        /// unique name of the enterprise account
        pub name: String,
    }

    /// Identity Provider associated with the Team
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
    pub struct IdentityProvider {
        /// unique identifier of this identity provider
        pub id: String,
        /// user-friendly unique identifier for this identity provider
        pub slug: String,
    }
}

mod team_app {
    use chrono::offset::Utc;
    use chrono::DateTime;

    /// Team App
    ///
    /// Stability: development
    ///
    /// A team app encapsulates the team specific functionality of Heroku apps.
    ///
    /// [For more information please refer to the Heroku documentation](https://devcenter.heroku.com/articles/platform-api-reference#team-app)
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
    pub struct TeamApp {
        /// when app was archived
        pub archived_at: Option<DateTime<Utc>>,
        /// description from buildpack of app
        pub buildpack_provided_description: Option<String>,
        /// build stack
        pub build_stack: BuildStack,
        /// when app was created
        pub created_at: DateTime<Utc>,
        /// git repo URL of app
        /// pattern: ^https://git\.heroku\.com/[a-z][a-z0-9-]{2,29}\.git$
        pub git_url: String,
        /// unique identifier
        pub id: String,
        /// describes whether a Private Spaces app is externally routable or not
        pub internal_routing: Option<bool>,
        /// is the current member a collaborator on this app.
        pub joined: bool,
        /// are other team members forbidden from joining this app.
        pub locked: bool,
        /// maintenance status of app
        pub maintenance: bool,
        /// name of app
        /// pattern: ^[a-z][a-z0-9-]{1,28}[a-z0-9]$
        pub name: String,
        /// team that owns this app
        pub team: Option<Team>,
        /// identity of app owner
        pub owner: Option<Owner>,
        /// A region represents a geographic location in which your application may run.
        pub region: Region,
        /// when app was released
        pub released_at: Option<DateTime<Utc>>,
        /// git repo size in bytes of app
        pub repo_size: Option<i64>,
        /// slug size in bytes of app
        pub slug_size: Option<i64>,
        /// identity of space
        pub space: Option<Space>,
        /// Stacks are the different application execution environments available in the Heroku platform.
        pub stack: Stack,
        /// when app was updated
        pub updated_at: DateTime<Utc>,
        /// web URL of app
        /// pattern: ^https?://[a-z][a-z0-9-]{3,30}\.herokuapp\.com/$
        pub web_url: String,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
    pub struct BuildStack {
        /// identifier of stack
        pub id: String,
        /// unique name
        pub name: String,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
    pub struct Team {
        /// unique name of team
        pub name: String,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
    pub struct Owner {
        /// unique email address
        pub email: String,
        /// identifier of an account
        pub id: String,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
    pub struct Region {
        /// unique identifier
        pub id: String,
        /// name of region
        pub name: String,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
    pub struct Space {
        /// unique identifier of space
        pub id: String,
        /// unique name of space
        /// pattern: `^[a-z0-9](?:[a-z0-9]
        pub name: String,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
    pub struct Stack {
        /// identifier of stack
        pub id: String,
        /// unique name
        pub name: String,
    }
}
