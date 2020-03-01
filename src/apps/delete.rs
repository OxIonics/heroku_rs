//! Access the Apps portion of the Heroku API
imports!();
use crate::client::DeleteQueryBuilder;

// Declaration of types representing the various items under apps
new_type!(
    App
    Apps
    Webhooks
    WebhookId
    BuildCache
    Collaborator
    CollaboratorEmail
    CollaboratorId
    Domain
    DomainId
    DomainHostname
    Dyno
    DynoId
    DynoName

);

// From implementations for conversion
from!(
    @DeleteQueryBuilder
        -> Apps = "apps"
    @Apps
        => App
    @App
        -> Webhooks = "webhooks"
        -> BuildCache = "build-cache"
        -> Collaborator = "collaborators"
        -> Domain = "domains"
        -> Dyno = "dynos"
    @Webhooks
        => WebhookId
    @Collaborator
        => CollaboratorEmail
        => CollaboratorId
    @Domain
        => DomainHostname
        => DomainId
    @Dyno
        => DynoName
        => DynoId
      
);

// impls of each type
impl_macro!(
    @Apps
        |
        |=> app_name -> App = app_name_str
        |=> app_id -> App = app_id_str
    @App
        |=> app_webhooks -> Webhooks
        |=> app_build_cache -> BuildCache
        |=> app_collaborators -> Collaborator
        |=> app_domains -> Domain
        |=> app_dynos -> Dyno
        |
    @Webhooks
        |
        |=> webhook_id -> WebhookId = id
    @Collaborator
        |
        |=> collaborator_email -> CollaboratorEmail = email
        |=> collaborator_id -> CollaboratorId = id
    @Domain
        |
        |=> domain_hostname -> DomainHostname = hostname
        |=> domain_id -> DomainId = id
    @Dyno
        |
        |=> dyno_name -> DynoName = name
        |=> dyno_id -> DynoId = id
);

exec!(App);
exec!(Apps);
exec!(Webhooks);
exec!(WebhookId);
exec!(BuildCache);
exec!(Collaborator);
exec!(CollaboratorEmail);
exec!(CollaboratorId);
exec!(Domain);
exec!(DomainId);
exec!(DomainHostname);
exec!(Dyno);
exec!(DynoId);
exec!(DynoName);
