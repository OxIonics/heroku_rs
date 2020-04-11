extern crate heroku_rs;
use super::print_response;
use heroku_rs::endpoints::collaborators;
use heroku_rs::framework::apiclient::HerokuApiClient;

pub fn run<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    let app_name = String::from("heroku-rs-tests");

    // create_app_collaborate(api_client, app_name);
    get_app_collaborators(api_client, app_name);
    // get_app_collaborator(api_client, app_name);
    // delete_app_collaborator(api_client, app_name); // Careful here :)

    // create_team_app_collaborator(api_client, &app_name);
    // get_team_app_collaborators(api_client, &app_name);
    // get_team_app_collaborator(api_client, &app_name);
    // update_team_app_collaborator(api_client, &app_name);
}

/// Delete app collaborator
fn update_team_app_collaborator<T: HerokuApiClient>(api_client: &T, app_id: &str) {
    let permissions = vec!["view"];
    let collaborator_id = "123";
    // let response = api_client.request(&collaborators::TeamCollaboratorUpdate {
    //     app_id,
    //     collaborator_id,
    //     params: collaborators::TeamCollaboratorUpdateParams { permissions },
    // });

    //or
    let response = api_client.request(&collaborators::TeamCollaboratorUpdate::new(
        app_id,
        collaborator_id,
        permissions,
    ));
    print_response(response);
}

/// Get a team app collaborator
fn get_team_app_collaborator<T: HerokuApiClient>(api_client: &T, app_id: &str) {
    let collaborator_id = "Collab_id";
    // let response = api_client.request(&collaborators::TeamCollaboratorDetails {
    //     app_id,
    //     collaborator_id,
    // });

    //or
    let response = api_client.request(&collaborators::TeamCollaboratorDetails::new(
        app_id,
        collaborator_id,
    ));
    print_response(response);
}

/// Get a list of team app collaborators
fn get_team_app_collaborators<T: HerokuApiClient>(api_client: &T, app_id: &str) {
    let response = api_client.request(&collaborators::TeamCollaboratorList { app_id });
    //or
    // let response = api_client.request(&collaborators::TeamCollaboratorList::new(app_id));
    print_response(response);
}

/// Create team app collaborator
fn create_team_app_collaborator<T: HerokuApiClient>(api_client: &T, app_id: &str) {
    let user = "EMAIL_or_ID_HERE";
    let silent = Some(false);
    let permissions = Some(vec!["view"]);
    //The old way of creating a request, still works
    // let response = api_client.request(&collaborators::TeamCollaboratorCreate {
    //     app_id,
    //     params: collaborators::TeamCollaboratorCreateParams {
    //         user,
    //         silent,
    //         permissions,
    //     },
    // });

    //New way to create a request where you can pass optional parameters
    let response = api_client.request(&collaborators::TeamCollaboratorCreate::new(
        app_id,
        user,
        silent,
        permissions,
    ));

    //New way to create a request where you can pass only the required parameters
    // let response = api_client.request(&collaborators::TeamCollaboratorCreate::create(app_id, user));

    print_response(response);
}

/// Delete app collaborator
fn delete_app_collaborator<T: HerokuApiClient>(api_client: &T, app_id: String) {
    let collaborator_id = String::from("COLLAB_EMAIL_OR_ID");
    let response = api_client.request(&collaborators::CollaboratorDelete {
        app_id,
        collaborator_id,
    });
    print_response(response);
}

/// Get app collaborator
fn get_app_collaborator<T: HerokuApiClient>(api_client: &T, app_id: String) {
    let collaborator_id = String::from("COLLAB_EMAIL_OR_ID");
    let response = api_client.request(&collaborators::CollaboratorDetails {
        app_id,
        collaborator_id,
    });
    print_response(response);
}

/// Get a list of app collaborators
fn get_app_collaborators<T: HerokuApiClient>(api_client: &T, app_id: String) {
    let response = api_client.request(&collaborators::CollaboratorList { app_id });
    print_response(response);
}

/// Create app collaborator
fn create_app_collaborate<T: HerokuApiClient>(api_client: &T, app_id: String) {
    let user = String::from("EMAIL_or_ID_HERE");
    let silent = Some(false);
    let response = api_client.request(&collaborators::CollaboratorCreate {
        app_id,
        params: collaborators::CollaboratorCreateParams { user, silent },
    });
    print_response(response);
}
