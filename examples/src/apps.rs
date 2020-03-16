#![allow(dead_code)] // Just warns about un-used methods until they're used.
use heroku_rs::client::{Executor, Heroku};
use heroku_rs::defaults::{
    AppPatch, AppPost, BuildPackUpdate, BuildPost, BuildpackInstallation, EnableFeature,
    NewCollaborator, PatchAddons, PatchSniEndpoints, PatchSslEndpoints, SourceBlob, WebhookPost,
};
use heroku_rs::errors::Error;
use heroku_rs::{HeaderMap, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::Value;
// Uncomment methods to run them.
pub fn run(client: Heroku) {
    // let app_name = "heroku-rs-tests";
    // get_apps(&client);
    // get_app_features(&client);
    // patch_app(&client);
    // patch_feature(&client);
    // post_app(&client);
    // delete_app(&client);
    // get_webhooks(&client);
    // delete_webhook(&client);
    // post_webhook(&client);
    // patch_webhook(&client);
    // get_webhook_deliveries(&client);
    // get_webhook_events(&client);
    // get_builds(&client);
    // post_build(&client);
    // delete_build_cache(&client);
    // put_buildpack_installations(&client);
    // get_buildpack_installations(&client);
    // post_collaborator(&client);
    // get_collaborators(&client);
    // delete_collaborators(&client);
    // get_config_vars(&client);
    // patch_config_vars(&client);
    // post_new_domain(&client);
    // get_domains(&client);
    // delete_domain(&client);
    // get_dynos(&client);
    // post_dynos(&client);
    // delete_dyno(&client);
    // post_dyno_stop(&client);
    // get_app_formation(&client, app_name);
    // patch_app_formation(&client, app_name);
    // post_log_drains(&client, app_name);
    // delete_log_drain(&client, app_name);
    // get_log_drains(&client, app_name);
    // post_log_sessions(&client, app_name);
    // get_app_addons(&client, app_name);
    // get_app_addon_attachments(&client, app_name);
    // get_app_pipeline_couplings(&client, app_name);
    // get_app_review(&client, app_name);
    // get_app_slug(&client, app_name);
    // get_app_sni_endpoints(&client, app_name);
    // get_app_ssl_endpoints(&client, app_name);
    // patch_sni_endpoints(&client, app_name);
    // patch_ssl_endpoints(&client, app_name);
    // patch_addons(&client, app_name);
    // patch_acm(&client, app_name);
    // delete_acm(&client, app_name);
    // delete_sni_endpoints(&client, app_name);
    // delete_ssl_endpoints(&client, app_name);
}

// == Getting an app ==
// Requires only the Heroku client to get all the apps
// If you want to get a specific app you can do so..
// by quering .app_name("NAME_HERE") or .app_id("ID_HERE")

fn get_apps(client: &Heroku) {
    let me = client.get().apps().execute::<Value>();
    log_response(me);
}
// == Getting app features ==
// Requires the client and the app name
//  .app_features() returns all the features on this app
// or get a specific feature by name e.g. .feature_name("web-auto-scaling") OR can by id: feature_id("47d1998e-f8f4-432d-b4cc-f105f4d76407")

fn get_app_features(client: &Heroku) {
    let me = client
        .get()
        .apps()
        .app_name("APP_NAME")
        .app_features()
        .feature_name("web-auto-scaling")
        .execute::<Value>();

    log_response(me);
}

// == Patching an app ==
// https://devcenter.heroku.com/articles/platform-api-reference#app-update

fn patch_app(client: &Heroku) {
    // create the patch object, these are optional
    let app_patch = AppPatch {
        build_stack: String::from("BUILD_STACK"), // unique name or identifier of stack, you can get build_stack id from the get method
        maintenance: true,                        // maintenance status of app
        name: String::from("APP_NAME"),           //name of app
    };

    let result = client
        .patch(app_patch)
        .apps()
        .app_name("APP_NAME") //must specify exactly which app you want to patch
        .execute::<Value>();

    log_response(result);
}
// == Patching a feature ==
// https://devcenter.heroku.com/articles/platform-api-reference#app-feature-update
// Library provides a default struct EnableFeature which has only 1 prop, boolean, should enable the feature or not

fn patch_feature(client: &Heroku) {
    let enable = EnableFeature { enabled: true };
    let result = client
        .patch(enable)
        .apps()
        .app_name("APP_NAME")
        .app_features()
        .feature_name("web-auto-scaling")
        .execute::<Value>();

    log_response(result);
}

// == Creating an app ==
// https://devcenter.heroku.com/articles/platform-api-reference#app-create
// Library provides a default struct AppPost to create a simple app.
fn post_app(client: &Heroku) {
    let region = String::from("us"); //unique identifier or name of region e.g.	"01234567-89ab-cdef-0123-456789abcdef" or "us"
    let stack = String::from("heroku-18"); // unique name or identifier of stack e.g. heroku-18
    let name = String::from("mynewcoolapp"); // name of app e.g. mynewcoolapp
    let app_to_create = AppPost {
        region,
        stack,
        name,
    };
    let me = client.post(app_to_create).apps().execute::<Value>();
    log_response(me);
}

// == Deleting an app ==
// https://devcenter.heroku.com/articles/platform-api-reference#app-delete
fn delete_app(client: &Heroku) {
    let name = String::from("mynewcoolapp"); // name or id of the app to delete e.g. mynewcoolapp
    let me = client
        .delete_empty()
        .apps()
        .app_name(&name)
        .execute::<Value>();

    log_response(me);
}

// == Getting all webhooks and specific ones by id ==
// Requires only the Heroku client to get all the webhooks
// If you want to get a specific webhooks you can do so..
// by quering .webhook_id("ID_HERE")
// https://devcenter.heroku.com/articles/platform-api-reference#app-webhook-info
fn get_webhooks(client: &Heroku) {
    let me = client
        .get()
        .apps()
        .app_name("APP_NAME")
        .app_webhooks() //get all webhooks
        .webhook_id("Hook-Id-here") // get a specific webhook by id
        .execute::<Value>();

    log_response(me);
}

// == Delete specific webhook by id ==
// Requires only the Heroku client & webhook id
// https://devcenter.heroku.com/articles/platform-api-reference#app-webhook-delete

fn delete_webhook(client: &Heroku) {
    let name = String::from("APP_NAME");
    let me = client
        .delete_empty()
        .apps()
        .app_name(&name)
        .app_webhooks()
        .webhook_id("ID_HERE")
        .execute::<Value>();

    log_response(me);
}

// == Create a new webhook  ==
// https://devcenter.heroku.com/articles/platform-api-reference#app-webhook-create

fn post_webhook(client: &Heroku) {
    let include: Vec<String> = vec!["api:release".to_string()];
    let level = String::from("notify");
    let url = String::from("https://crates.io/crates/heroku_rs"); //the URL where the webhook’s notification requests are sent
    let new_webhook = WebhookPost {
        include,
        level,
        url,
    };
    let me = client
        .post(new_webhook)
        .apps()
        .app_name("APP_NAME")
        .app_webhooks()
        .execute::<Value>();

    log_response(me);
}

// == Update existing webhook  ==
// https://devcenter.heroku.com/articles/platform-api-reference#app-webhook-update
// In this example, i'm just updating webhook level to `sync`
fn patch_webhook(client: &Heroku) {
    #[derive(Serialize, Deserialize)]
    struct UpdateLevel {
        level: String,
    };
    let update_level = UpdateLevel {
        level: "sync".to_string(),
    };
    let me = client
        .patch(update_level)
        .apps()
        .app_name("APP_NAME")
        .app_webhooks()
        .webhook_id("Hook-Id-here")
        .execute::<Value>();

    log_response(me);
}

// == Getting webhook deliveries ==
// Endpoint: https://devcenter.heroku.com/articles/platform-api-reference#app-webhook-delivery-info
// Requires the Heroku client and an App to get all the webhook deliveries
// get a specific webhook delivery by doing: .webhook_delivery_id("ID_HERE")
fn get_webhook_deliveries(client: &Heroku) {
    let me = client
        .get()
        .apps()
        .app_name("APP_NAME")
        .app_webhook_deliveries()
        .webhook_delivery_id("ID_HERE")
        .execute::<Value>();

    log_response(me);
}

// == Getting webhook events ==
// Endpoint: https://devcenter.heroku.com/articles/platform-api-reference#app-webhook-event-info
// Requires the Heroku client and an App to get all the webhook events
// get a specific webhook event by doing: .webhook_event_id("ID_HERE")
fn get_webhook_events(client: &Heroku) {
    let me = client
        .get()
        .apps()
        .app_name("APP_NAME")
        .app_webhook_events()
        .execute::<Value>();

    log_response(me);
}

// == Getting app builds  ==
// Endpoint: https://devcenter.heroku.com/articles/platform-api-reference#build
// Requires the Heroku client and an App to get all the app builds
// get a specific build by doing: .app_builds().build_id("ID_HERE")
fn get_builds(client: &Heroku) {
    let me = client
        .get()
        .apps()
        .app_name("APP_NAME")
        .app_builds()
        .execute::<Value>();

    log_response(me);
}

// == Creating an app build  ==
// Endpoint: https://devcenter.heroku.com/articles/platform-api-reference#build-create
// Requires the Heroku client, the app you want to create a build for
// Library provides a default struct BuildPost to create a simple build

fn post_build(client: &Heroku) {
    let url = String::from("URL");
    let version = String::from("VERSION_NUMBER");
    let blob = SourceBlob {
        checksum: None,
        url: url,
        version: Some(version),
    };
    let build = BuildPost {
        buildpacks: None,
        source_blob: blob,
    };
    let me = client
        .post(build)
        .apps()
        .app_name("APP_NAME")
        .app_build()
        .execute::<Value>();

    log_response(me);
}

// == DELETE app build cache ==
// Endpoint: https://devcenter.heroku.com/articles/platform-api-reference#build-delete-cache
// Requires the Heroku client, the app you want to delete the cache build for
fn delete_build_cache(client: &Heroku) {
    let me = client
        .delete_empty()
        .apps()
        .app_name("APP_NAME")
        .app_build_cache()
        .execute::<Value>();
    log_response(me);
}

// == Update buildpack installations ==
// Endpoint: https://devcenter.heroku.com/articles/platform-api-reference#buildpack-installations-update
// Requires the Heroku client, the app you want to update buildpack for
fn put_buildpack_installations(client: &Heroku) {
    let bp_update = BuildPackUpdate {
        buildpack: String::from("https://github.com/heroku/heroku-buildpack-python"),
    };

    let bp_install = BuildpackInstallation {
        updates: vec![bp_update],
    };

    let me = client
        .put(bp_install)
        .apps()
        .app_name("APP_NAME")
        .app_buildpack_installation()
        .execute::<Value>();

    log_response(me);
}

// == Get buildpack installations ==
// Endpoint: https://devcenter.heroku.com/articles/platform-api-reference#buildpack-installations-list
// Requires the Heroku client, the app you want to list the builpacks for
fn get_buildpack_installations(client: &Heroku) {
    let me = client
        .get()
        .apps()
        .app_name("APP_NAME")
        .app_buildpack_installations()
        .execute::<Value>();

    log_response(me);
}

// == Add a new collaborator ==
// Endpoint: https://devcenter.heroku.com/articles/platform-api-reference#collaborator-create
// Requires the Heroku client, the app you want to add a collaborator
fn post_collaborator(client: &Heroku) {
    let user = String::from("EMAIL_HERE"); // the email of the collaborator to invite
    let new_collaborator = NewCollaborator {
        silent: Some(false),
        user: user,
    };
    let me = client
        .post(new_collaborator)
        .apps()
        .app_name("APP_NAME")
        .app_collaborator()
        .execute::<Value>();

    log_response(me);
}

// == Get collaborators ==
// Endpoint: https://devcenter.heroku.com/articles/platform-api-reference#collaborator-info
// Requires the Heroku client, the app you want to get the collaborators of
// get a specific collaborator by id by doing: .app_collaborators().collaborator_id("ID_HERE")
// or by email : .app_collaborators().collaborator_email("EMAIL_HERE")
fn get_collaborators(client: &Heroku) {
    let me = client
        .get()
        .apps()
        .app_name("APP_NAME")
        .app_collaborators()
        .execute::<Value>();

    log_response(me);
}

// == Delete specific collaborator by id or email ==
// Requires only the Heroku client & collaborator id or email
// Endpoint: https://devcenter.heroku.com/articles/platform-api-reference#collaborator-delete
fn delete_collaborators(client: &Heroku) {
    let me = client
        .delete_empty()
        .apps()
        .app_name("APP_NAME")
        .app_collaborators()
        .collaborator_email("COLLABORATOR_EMAIL_HERE")
        .execute::<Value>();

    log_response(me);
}

// == Get config vars ==
// Endpoint: https://devcenter.heroku.com/articles/platform-api-reference#config-vars
// Requires the Heroku client, the app you want to get the config vars of
// You can get all config vars for an app or for a specific release
// All config vars for an app: .get().apps().app_name("NAME_HERE").app_config_vars()
// All config vars for a specific app release. See below:
fn get_config_vars(client: &Heroku) {
    let me = client
        .get()
        .apps()
        .app_name("APP_NAME")
        .app_releases() // get all releases
        .release_version("2") // get version e.g. 2
        .release_config_vars() // get all config vars for that version 2 release
        .execute::<Value>();

    log_response(me);
}

// == Patch config vars ==
// Endpoint: https://devcenter.heroku.com/articles/platform-api-reference#config-vars-update
// Requires the Heroku client, the app you want to patch the config vars of
// You can get all config vars for an app or for a specific release
// Patch config vars for an app: .patch(obj).apps().app_name("NAME_HERE").app_config_vars()
fn patch_config_vars(client: &Heroku) {
    //Patch takes an object, this is just an example struct
    #[derive(Serialize, Deserialize)]
    struct AddConfig {
        config: String,
    };
    let new_config = AddConfig {
        config: String::from("My_New_Config"),
    };

    let me = client
        .patch(new_config)
        .apps()
        .app_name("APP_NAME")
        .app_config_vars()
        .execute::<Value>();

    log_response(me);
}

// == POST new domain ==
// Endpoint: https://devcenter.heroku.com/articles/platform-api-reference#domain-create
// Requires the Heroku client, the app you want to create a domain for and the uri
fn post_new_domain(client: &Heroku) {
    //Post takes an object, this is just an example struct
    #[derive(Serialize, Deserialize)]
    struct NewDomain {
        hostname: String,
    };
    let new_domain = NewDomain {
        hostname: String::from("my-excellent-app.herokuapp.com"), // the full hostname
    };
    let me = client
        .post(new_domain)
        .apps()
        .app_name("APP_NAME")
        .app_domain()
        .execute::<Value>();

    log_response(me);
}

// == GET app domains  ==
// Endpoint: https://devcenter.heroku.com/articles/platform-api-reference#domain-info
// Requires the Heroku client and an App to get all the app domains
// get a specific domain by id: .app_domains().domain_id("ID_HERE")
//  or by hostname: .app_domains().domain_hostname("HOSTNAME_HERE")
fn get_domains(client: &Heroku) {
    let me = client
        .get()
        .apps()
        .app_name("APP_NAME")
        .app_domains()
        .execute::<Value>();

    log_response(me);
}
// == DELETE app domain  ==
// Endpoint: https://devcenter.heroku.com/articles/platform-api-reference#domain-delete
// Requires the Heroku client, an App which has domain and a domain id or hostname
fn delete_domain(client: &Heroku) {
    let me = client
        .delete_empty()
        .apps()
        .app_name("APP_NAME")
        .app_domains()
        .domain_hostname("my-excellent-app.herokuapp.com")
        .execute::<Value>();

    log_response(me);
}

// == GET app dynos  ==
// Endpoint: https://devcenter.heroku.com/articles/platform-api-reference#dyno-create
// Requires the Heroku client and an App to get the app dynos
// get a specific dyno by id: .app_dynos().dyno_id("ID_HERE")
//  or by name: .app_dynos().dyno_name("NAME_HERE")
fn get_dynos(client: &Heroku) {
    let me = client
        .get()
        .apps()
        .app_name("APP_NAME")
        .app_dynos()
        .dyno_id("a61ed193-66fd-4a34-9ac5-697ba12b01ba")
        .execute::<Value>();

    log_response(me);
}

// == POST new dyno ==
// Endpoint: https://devcenter.heroku.com/articles/platform-api-reference#dyno-create
// Requires the Heroku client, the app you want to create a new dyno for
fn post_dynos(client: &Heroku) {
    #[derive(Serialize, Deserialize)]
    struct DynoCmd {
        command: String,
    };

    let cmd = DynoCmd {
        command: String::from("bash"),
    };
    let me = client
        .post(cmd)
        .apps()
        .app_name("APP_NAME")
        .app_dyno()
        .execute::<Value>();
    log_response(me);
}

// == DELETE dyno ==
// Endpoint: https://devcenter.heroku.com/articles/platform-api-reference#dyno-restart-all
// Requires the Heroku client, the app you want to delete the dyno for
// To delete a dyno by name .app_dynos().dyno_name("Dyno_name_here")
// To delete a dyno by id .app_dynos().dyno_id("Dyno_id_here")
// To delete all dynos .app_dynos()
fn delete_dyno(client: &Heroku) {
    let me = client
        .delete_empty()
        .apps()
        .app_name("APP_NAME")
        .app_dynos()
        .execute::<Value>();

    log_response(me);
}

// == POST dyno stop ==
// Endpoint: https://devcenter.heroku.com/articles/platform-api-reference#dyno-stop
// POST /apps/{app_id_or_name}/dynos/{dyno_id_or_name}/actions/stop
// Requires the Heroku client, the app you want to create a new dyno for
fn post_dyno_stop(client: &Heroku) {
    let me = client
        .post_empty()
        .apps()
        .app_name("APP_NAME")
        .app_dyno()
        .dyno_name("DYNO_NAME_HERE")
        .dyno_action()
        .action_stop()
        .execute::<Value>();

    log_response(me);
}

// == GET app formations  ==
// Endpoint: https://devcenter.heroku.com/articles/platform-api-reference#formation-info
// Requires the Heroku client and an App to get the formation for
// get a specific formation by id: .app_formations().formation_id("ID_HERE")
//  or by type: .app_formations().formation_type("TYPE_HERE")
fn get_app_formation(client: &Heroku, app_name: &str) {
    let me = client
        .get()
        .apps()
        .app_name(app_name)
        .app_formations()
        .execute::<Value>();

    log_response(me);
}

// == PATCH app formation ==
// Endpoint: https://devcenter.heroku.com/articles/platform-api-reference#formation-update
// Requires the Heroku client, the app you want to patch the formations of
// You can patch specific formations by type or id or app formations
// Patch formations .patch(obj).apps().app_name("NAME_HERE").app_formations()
// Patch formation by type: .patch(obj).apps().app_name("NAME_HERE").app_formations().formation_type("TYPE_HERE")
fn patch_app_formation(client: &Heroku, app_name: &str) {
    //Patch takes an object, this is just an example struct
    #[derive(Serialize, Deserialize)]
    struct FormationUpdate {
        quantity: u32,
        size: String,
    };
    let formation_patch = FormationUpdate {
        quantity: 1,
        size: String::from("standard-1X"),
    };

    let me = client
        .patch(formation_patch)
        .apps()
        .app_name(app_name)
        .app_formations()
        .formation_id("formation_id")
        .execute::<Value>();

    log_response(me);
}

// == POST log drains ==
// Endpoint: https://devcenter.heroku.com/articles/platform-api-reference#log-drain-create
// Requires the Heroku client and the app you want to create a log drain for
fn post_log_drains(client: &Heroku, app_name: &str) {
    #[derive(Serialize, Deserialize)]
    struct DrainCreate {
        url: String,
    };

    let cmd = DrainCreate {
        url: String::from("https://example.com/drain"),
    };
    let me = client
        .post(cmd)
        .apps()
        .app_name(app_name)
        .app_log_drain()
        .execute::<Value>();
    log_response(me);
}

// == DELETE log drains ==
// Endpoint: https://devcenter.heroku.com/articles/platform-api-reference#log-drain-delete
// Requires the Heroku client and the app you want to delete a log drain for and drain id/token/url
fn delete_log_drain(client: &Heroku, app_name: &str) {
    let me = client
        .delete_empty()
        .apps()
        .app_name(app_name)
        .app_log_drains()
        .log_drain_url("https://example.com/drain")
        .execute::<Value>();
    log_response(me);
}
// == GET app log drains  ==
// Endpoint: https://devcenter.heroku.com/articles/platform-api-reference#log-drain-info
// Requires the Heroku client and an App to get the log drains for
// get a specific log drain by id: .app_log_drains().log_drain_id("ID_HERE")
// get a specific log drain by url: .app_log_drains().log_drain_url("URL_HERE")
// get a specific log drain by token: .app_log_drains().log_drain_token("TOKEN_HERE")
//  or get a list of log drains like below:
fn get_log_drains(client: &Heroku, app_name: &str) {
    let me = client
        .get()
        .apps()
        .app_name(app_name)
        .app_log_drains()
        .execute::<Value>();

    log_response(me);
}

// == POST log sessions  ==
// Endpoint: https://devcenter.heroku.com/articles/platform-api-reference#log-session-create
// Requires the Heroku client and an App to create a new log session.
fn post_log_sessions(client: &Heroku, app_name: &str) {
    #[derive(Serialize, Deserialize)]
    pub struct CreateLogSession {
        pub dyno: String,
        pub lines: i64,
        pub source: String,
        pub tail: bool,
    }
    let obj = CreateLogSession {
        dyno: String::from("web.1"),
        lines: 10,
        source: String::from("app"),
        tail: true,
    };
    let me = client
        .post(obj)
        .apps()
        .app_name(app_name)
        .app_log_session()
        .execute::<Value>();

    log_response(me);
}

// == GET app addons ==
// Endpoint: https://devcenter.heroku.com/articles/platform-api-reference#add-on-list-by-app
// Requires the Heroku client and an App to get the addons
// get a specific addon by id: .app_addons().addon_id("ID_HERE")
// get a specific addon by name: .app_addons().addon_name("NAME_HERE")
fn get_app_addons(client: &Heroku, app_name: &str) {
    let me = client
        .get()
        .apps()
        .app_name(app_name)
        .app_addons()
        .execute::<Value>();

    log_response(me);
}

// == GET app addon attachments ==
// Endpoint: https://devcenter.heroku.com/articles/platform-api-reference#add-on-attachment-list-by-app
// Requires the Heroku client and the app name to get the addon attachments for
// get a specific addon by id: .app_addon_attachments().addon_attachment_id("ID_HERE")
// get a specific addon by name: .app_addon_attachments().addon_attachment_name("NAME_HERE")
fn get_app_addon_attachments(client: &Heroku, app_name: &str) {
    let me = client
        .get()
        .apps()
        .app_name(app_name)
        .app_addon_attachments()
        .execute::<Value>();

    log_response(me);
}

// == GET app pipeline couplings ==
// Endpoint: https://devcenter.heroku.com/articles/platform-api-reference#pipeline-coupling-info-by-app
// Requires the Heroku client and the app name to get the pipeline couplings
fn get_app_pipeline_couplings(client: &Heroku, app_name: &str) {
    let me = client
        .get()
        .apps()
        .app_name(app_name)
        .app_pipeline_couplings()
        .execute::<Value>();

    log_response(me);
}

// == GET app review ==
// Endpoint: https://devcenter.heroku.com/articles/platform-api-reference#review-app-get-review-app-by-app_id
// Requires the Heroku client and the app name to get the app review
// GET /apps/{app_id_or_name}/review-app
fn get_app_review(client: &Heroku, app_name: &str) {
    let me = client
        .get()
        .apps()
        .app_name(app_name)
        .app_review()
        .execute::<Value>();

    log_response(me);
}

// == GET app slug ==
// Endpoint: https://devcenter.heroku.com/articles/platform-api-reference#slug-info
// Requires the Heroku client, the app name and the slug id
// GET /apps/{app_id_or_name}/slugs/{slug_id}
fn get_app_slug(client: &Heroku, app_name: &str) {
    let me = client
        .get()
        .apps()
        .app_name(app_name)
        .app_slug()
        .slug_id("ID_HERE")
        .execute::<Value>();

    log_response(me);
}

// == GET app SNI endpoints ==
// Endpoint: https://devcenter.heroku.com/articles/platform-api-reference#sni-endpoint-info
// Requires the Heroku client, the app name
// get a specific SNI endpoint by id: .app_sni_endpoints().sni_endpoint_id("ID_HERE")
// get a specific SNI endpoint by name: .app_sni_endpoints().sni_endpoint_name("NAME_HERE")
// GET /apps/{app_id_or_name}/sni-endpoints/{sni_endpoint_id_or_name}
fn get_app_sni_endpoints(client: &Heroku, app_name: &str) {
    let me = client
        .get()
        .apps()
        .app_name(app_name)
        .app_sni_endpoints()
        .execute::<Value>();

    log_response(me);
}

// == GET app SSL endpoints ==
// Endpoint: https://devcenter.heroku.com/articles/platform-api-reference#ssl-endpoint-info
// Requires the Heroku client, the app name
// get a specific SSL endpoint by id: .app_ssl_endpoints().ssl_endpoint_id("ID_HERE")
// get a specific SSL endpoint by name: .app_ssl_endpoints().ssl_endpoint_name("NAME_HERE")
// GET /apps/{app_id_or_name}/ssl-endpoints/{ssl_endpoint_id_or_name}
fn get_app_ssl_endpoints(client: &Heroku, app_name: &str) {
    let me = client
        .get()
        .apps()
        .app_name(app_name)
        .app_ssl_endpoints()
        .execute::<Value>();

    log_response(me);
}

// == PATCH app SNI endpoints ==
// Endpoint: https://devcenter.heroku.com/articles/platform-api-reference#sni-endpoint-update
// Requires the Heroku client, the app name and the sni endpoint id or name to patch
// patch a specific SNI endpoint by name: .app_sni_endpoints().sni_endpoint_name("NAME_HERE")
// PATCH /apps/{app_id_or_name}/sni-endpoints/{sni_endpoint_id_or_name}
fn patch_sni_endpoints(client: &Heroku, app_name: &str) {
    let patch = PatchSniEndpoints {
        certificate_chain: String::from("-----BEGIN CERTIFICATE----- ..."),
        private_key: String::from("-----BEGIN RSA PRIVATE KEY----- ..."),
    };
    let me = client
        .patch(patch)
        .apps()
        .app_name(app_name)
        .app_sni_endpoints()
        .sni_endpoint_id("Id_HERE")
        .execute::<Value>();

    log_response(me);
}

// == PATCH app SSL endpoints ==
// Endpoint: https://devcenter.heroku.com/articles/platform-api-reference#ssl-endpoint-update
// Requires the Heroku client, the app name and the ssl endpoint id or name to patch
// patch a specific SNI endpoint by name: .app_ssl_endpoints().ssl_endpoint_name("NAME_HERE")
// PATCH /apps/{app_id_or_name}/ssl-endpoints/{ssl_endpoint_id_or_name}
fn patch_ssl_endpoints(client: &Heroku, app_name: &str) {
    let patch = PatchSslEndpoints {
        certificate_chain: Some(String::from("-----BEGIN CERTIFICATE----- ...")),
        private_key: Some(String::from("-----BEGIN RSA PRIVATE KEY----- ...")),
        preprocess: Some(false),
    };
    let me = client
        .patch(patch)
        .apps()
        .app_name(app_name)
        .app_ssl_endpoints()
        .ssl_endpoint_id("Id_HERE")
        .execute::<Value>();

    log_response(me);
}

// == PATCH app ADDONS ==
// Endpoint: https://devcenter.heroku.com/articles/platform-api-reference#add-on-update
// Requires the Heroku client, the app name and the addon id or name to patch
// patch a specific addon by name: .app_addons().addon_name("NAME_HERE")
// PATCH /apps/{app_id_or_name}/addons/{add_on_id_or_name}
fn patch_addons(client: &Heroku, app_name: &str) {
    let patch = PatchAddons {
        plan: String::from("heroku-postgresql:dev"),
        name: None,
    };
    let me = client
        .patch(patch)
        .apps()
        .app_name(app_name)
        .app_addons()
        .addon_id("Id_HERE")
        .execute::<Value>();

    log_response(me);
}

// == PATCH app ACM ==
// Endpoint: https://devcenter.heroku.com/articles/platform-api-reference#app-refresh-acm
// Requires the Heroku client, the app name 
// PATCH /apps/{app_id_or_name}/acm
fn patch_acm(client: &Heroku, app_name: &str) {
    let me = client
        .patch_empty()
        .apps()
        .app_name(app_name)
        .app_acm()
        .execute::<Value>();

    log_response(me);
}

// == DELETE app ACM ==
// Endpoint: https://devcenter.heroku.com/articles/platform-api-reference#app-disable-acm
// Requires the Heroku client, the app name 
// DELETE /apps/{app_id_or_name}/acm
fn delete_acm(client: &Heroku, app_name: &str) {
    let me = client
        .delete_empty()
        .apps()
        .app_name(app_name)
        .app_acm()
        .execute::<Value>();

    log_response(me);
}

// == DELETE app SNI endpoints ==
// Endpoint: https://devcenter.heroku.com/articles/platform-api-reference#sni-endpoint-delete
// Requires the Heroku client, the app name and the sni endpoint id or name to delete
// delete a specific SNI endpoint by name: .app_sni_endpoints().sni_endpoint_name("NAME_HERE")
// DELETE /apps/{app_id_or_name}/sni-endpoints/{sni_endpoint_id_or_name}
fn delete_sni_endpoints(client: &Heroku, app_name: &str) {
    let me = client
        .delete_empty()
        .apps()
        .app_name(app_name)
        .app_sni_endpoints()
        .sni_endpoint_id("Id_HERE")
        .execute::<Value>();

    log_response(me);
}

// == DELETE app SSL endpoints ==
// Endpoint:https://devcenter.heroku.com/articles/platform-api-reference#ssl-endpoint-delete
// Requires the Heroku client, the app name and the ssl endpoint id or name to delete
// delete a specific SNI endpoint by name: .app_ssl_endpoints().ssl_endpoint_name("NAME_HERE")
// DELETE /apps/{app_id_or_name}/ssl-endpoints/{ssl_endpoint_id_or_name}
fn delete_ssl_endpoints(client: &Heroku, app_name: &str) {
    let me = client
        .delete_empty()
        .apps()
        .app_name(app_name)
        .app_ssl_endpoints()
        .ssl_endpoint_id("Id_HERE")
        .execute::<Value>();

    log_response(me);
}

//a generic method to log heroku responses and avoid code duplication
fn log_response<T>(me: Result<(HeaderMap, StatusCode, Option<T>), Error>)
where
    T: ToString,
{
    match me {
        Ok((headers, status, json)) => {
            println!("Headers: {:#?}", headers);
            println!("Status: {}", status);
            if let Some(json) = json {
                println!("Response: {}", json.to_string());
            }
        }
        Err(e) => println!("Err {}", e),
    }
}
