//Anything related to GET requests for pipelines and it's properties goes here.
use super::{Pipeline, PipelineBuild, PipelineCoupling, PipelineDeployment, PipelinePromotion};

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Pipeline Info
///
/// Info for existing pipeline.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#pipeline-info)
pub struct PipelineDetails {
    /// unique pipeline identifier.
    pub pipeline_id: String,
}

impl HerokuEndpoint<Pipeline> for PipelineDetails {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("pipelines/{}", self.pipeline_id)
    }
}

/// Pipeline List
///
/// List existing pipelines.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#pipeline-list)
pub struct PipelineList {}

impl HerokuEndpoint<Vec<Pipeline>> for PipelineList {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("pipelines")
    }
}

/// Pipeline Build List
///
/// List latest builds for each app in a pipeline
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#pipeline-build-list)
pub struct PipelineLatestBuildsList {
    /// unique pipeline identifier.
    pub pipeline_id: String,
}

impl HerokuEndpoint<Vec<PipelineBuild>> for PipelineLatestBuildsList {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("pipelines/{}/latest-builds", self.pipeline_id)
    }
}

/// Pipeline Coupling List By Pipeline
///
/// List couplings for a pipeline
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#pipeline-coupling-list-by-pipeline)
pub struct PipelineCouplingByPipelineList {
    /// unique pipeline identifier.
    pub pipeline_id: String,
}

impl HerokuEndpoint<Vec<PipelineCoupling>> for PipelineCouplingByPipelineList {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("pipelines/{}/pipeline-couplings", self.pipeline_id)
    }
}

/// Pipeline Coupling List By Current User
///
/// List pipeline couplings for the current user.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#pipeline-coupling-list-by-current-user)
pub struct PipelineCouplingByUserList {}
impl HerokuEndpoint<Vec<PipelineCoupling>> for PipelineCouplingByUserList {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("users/~/pipeline-couplings")
    }
}

/// Pipeline Coupling List By Team
///
/// List pipeline couplings for a team.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#pipeline-coupling-list-by-team)
pub struct PipelineCouplingByTeamList {
    /// unique team identifier.
    pub team_id: String,
}
impl HerokuEndpoint<Vec<PipelineCoupling>> for PipelineCouplingByTeamList {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("teams/{}/pipeline-couplings", self.team_id)
    }
}

/// Pipeline Coupling Info By App
///
/// Info for an existing pipeline coupling.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#pipeline-coupling-info-by-app)
pub struct PipelineCouplingByAppDetails {
    /// unique app identifier.
    pub app_id: String,
}
impl HerokuEndpoint<PipelineCoupling> for PipelineCouplingByAppDetails {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("apps/{}/pipeline-couplings", self.app_id)
    }
}

/// Pipeline Coupling List
///
/// List pipeline couplings.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#pipeline-coupling-list)
pub struct PipelineCouplingList {}
impl HerokuEndpoint<Vec<PipelineCoupling>> for PipelineCouplingList {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("pipeline-couplings")
    }
}

/// Pipeline Coupling Info
///
/// Info for an existing pipeline coupling.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#pipeline-coupling-info)
pub struct PipelineCouplingDetails {
    /// unique pipeline coupling identifier.
    pub coupling_id: String,
}

impl HerokuEndpoint<PipelineCoupling> for PipelineCouplingDetails {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("pipeline-couplings/{}", self.coupling_id)
    }
}

/// Pipeline Deployment List
///
/// List latest slug releases for each app in a pipeline
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#pipeline-deployment-list)
pub struct PipelineDeploymentList {
    /// unique pipeline identifier.
    pub pipeline_id: String,
}

impl HerokuEndpoint<Vec<PipelineDeployment>> for PipelineDeploymentList {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("pipelines/{}/latest-deployments", self.pipeline_id)
    }
}

/// Pipeline Promotion Info
///
/// Info for existing pipeline promotion.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#pipeline-promotion-info)
pub struct PipelinePromotionDetails {
    /// unique pipeline identifier.
    pub promotion_id: String,
}

impl HerokuEndpoint<PipelinePromotion> for PipelinePromotionDetails {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("pipeline-promotions/{}", self.promotion_id)
    }
}
