//Anything related to POST requests for spaces goes here.
use super::Space;

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Space Create
///
/// Create a new space.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#space-create)
pub struct SpaceCreate<'a> {
    /// The parameters to pass to the Heroku API
    pub params: SpaceCreateParams<'a>,
}

impl<'a> SpaceCreate<'a> {
    pub fn new(name: &'a str, team: &'a str) -> SpaceCreate<'a> {
        SpaceCreate {
            params: SpaceCreateParams {
                name: name,
                team: team,
                cidr: None,
                data_cidr: None,
                region: None,
                shield: None,
            },
        }
    }

    pub fn region(&mut self, regions: &'a str) -> &mut Self {
        self.params.region = Some(regions);
        self
    }
    pub fn cidr(&mut self, _cidr: &'a str) -> &mut Self {
        self.params.cidr = Some(_cidr);
        self
    }
    pub fn data_cidr(&mut self, _data_cidr: &'a str) -> &mut Self {
        self.params.data_cidr = Some(_data_cidr);
        self
    }
    pub fn shield(&mut self, _shield: bool) -> &mut Self {
        self.params.shield = Some(_shield);
        self
    }
    pub fn build(&self) -> SpaceCreate<'a> {
        SpaceCreate {
            params: SpaceCreateParams {
                name: self.params.name,
                team: self.params.team,
                cidr: self.params.cidr,
                data_cidr: self.params.data_cidr,
                region: self.params.region,
                shield: self.params.shield,
            },
        }
    }
}

/// Create a new space with parameters
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#space-create-required-parameters)
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug)]
pub struct SpaceCreateParams<'a> {
    /// unique name of space
    ///  pattern: `^[a-z0-9](?:[a-z0-9]
    pub name: &'a str,
    /// unique name of team
    pub team: &'a str,
    /// The RFC-1918 CIDR the Private Space will use. It must be a /16 in 10.0.0.0/8, 172.16.0.0/12 or 192.168.0.0/16
    ///  default: "10.0.0.0/16"
    ///  pattern: `^((?:10
    pub cidr: Option<&'a str>,
    /// The RFC-1918 CIDR that the Private Space will use for the Heroku-managed peering connection that’s automatically created when using Heroku Data add-ons.
    /// It must be between a /16 and a /20
    pub data_cidr: Option<&'a str>,
    /// unique identifier or name of region
    pub region: Option<&'a str>,
    /// true if this space has shield enabled
    pub shield: Option<bool>,
}

impl<'a> HerokuEndpoint<Space, (), SpaceCreateParams<'a>> for SpaceCreate<'a> {
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!("spaces")
    }
    fn body(&self) -> Option<SpaceCreateParams<'a>> {
        Some(self.params.clone())
    }
}
