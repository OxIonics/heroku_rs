extern crate heroku_rs;
use super::print_response;
use heroku_rs::endpoints::account;
use heroku_rs::framework::apiclient::HerokuApiClient;

pub fn run<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    get_account(api_client);
    // patch_account(api_client);
    // delete_account(api_client); //Careful here :)
    // get_user_account(api_client);
    // patch_user_account(api_client);
    // delete_user_account(api_client); //Careful here :)

    // get_account_features(api_client);
    // get_account_feature(api_client);
    // patch_account_feature(api_client);

    // get_account_transfers(api_client);
    // get_account_transfer(api_client);
    // create_account_transfer(api_client);
    // patch_account_transfer(api_client);
    // delete_account_transfer(api_client);

    // create_account_credits(api_client);
    // get_account_credit(api_client);
    // get_account_credits(api_client);

    // reset_account_password(api_client);
    // confirm_password(api_client);
}

// Confirm password reset.
fn confirm_password<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    let password_id = String::from("123");
    let old_password = String::from("123");
    let new_password = String::from("123");
    let response = api_client.request(&account::PasswordResetConfirm {
        password_id,
        params: account::PasswordResetConfirmParams {
            password: old_password,
            password_confirmation: new_password,
        },
    });
    print_response(response);
}

// Reset password.
fn reset_account_password<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    let email = String::from("EMAIL");
    let response = api_client.request(&account::PasswordReset {
        params: account::PasswordResetParams { email },
    });
    print_response(response);
}

// Get account credits.
fn get_account_credits<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    let response = api_client.request(&account::AccountCreditList {});
    print_response(response);
}

// Get account credit.
fn get_account_credit<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    let credit_id = String::from("012abc");

    let response = api_client.request(&account::AccountCreditDetails { credit_id });
    print_response(response);
}

// Create account credits.
fn create_account_credits<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    let code1 = Some(String::from("012abc"));
    let code2 = Some(String::from("012abcd"));
    let response = api_client.request(&account::AccountCreditCreate {
        params: account::AccountCreditCreateParams { code1, code2 },
    });
    print_response(response);
}

// Delete heroku account app transfer.
fn delete_account_transfer<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    let response = api_client.request(&account::AppTransferDelete {
        transfer_id: String::from("ID_HERE"),
    });
    print_response(response);
}

// Patch heroku account app transfer.
fn patch_account_transfer<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    let response = api_client.request(&account::AppTransferUpdate {
        transfer_id: String::from("ID_HERE"),
        params: account::AppTransferUpdateParams {
            state: String::from("declined"),
        },
    });
    print_response(response);
}

// Create heroku account app transfer.
fn create_account_transfer<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    let response = api_client.request(&account::AppTransferCreate {
        params: account::AppTransferCreateParams {
            app: String::from("ID_OR_APPNAME_HERE"),
            recipient: String::from("ID_OR_EMAIL_HERE"),
            silent: Some(false),
        },
    });
    print_response(response);
}

// Get heroku account app transfer.
fn get_account_transfer<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    let response = api_client.request(&account::AppTransferDetails {
        transfer_id: String::from("ID"),
    });
    print_response(response);
}

// Get heroku account app transfers.
fn get_account_transfers<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    let response = api_client.request(&account::AppTransferList {});
    print_response(response);
}

// Patch a specidic heroku account feature.
fn patch_account_feature<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    let response = api_client.request(&account::AccountFeatureUpdate {
        feature_id: String::from("team-internal-routing"),
        params: account::AccountFeatureUpdateParams { enabled: false },
    });
    print_response(response);
}

// Get a specidic heroku account feature.
fn get_account_feature<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    let response = api_client.request(&account::AccountFeatureDetails {
        feature_id: String::from("team-internal-routing"),
    });
    print_response(response);
}

// Get heroku account features.
fn get_account_features<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    let response = api_client.request(&account::AccountFeatureList {});
    print_response(response);
}

// Delete heroku user account. NOTE that this action cannot be undone.
fn delete_user_account<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    let account_id = String::from("USER_ID_OR_EMAIL");

    let response = api_client.request(&account::UserAccountDelete { account_id });
    print_response(response);
}

// Patch heroku user account
fn patch_user_account<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    let account_id = String::from("USER_ID_OR_EMAIL");
    let response = api_client.request(&account::UserAccountUpdate {
        account_id,
        params: account::UserAccountUpdateParams {
            allow_tracking: Some(true),
            beta: Some(false),
            name: Some(String::from("Heroku-testing")),
        },
    });
    print_response(response);
}

// Get heroku user account by email or id
fn get_user_account<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    let account_id = String::from("USER_ID_OR_EMAIL");
    let response = api_client.request(&account::UserAccountDetails { account_id });
    print_response(response);
}

// Delete heroku account. NOTE that this action cannot be undone.
fn delete_account<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    let response = api_client.request(&account::AccountDelete {});
    print_response(response);
}

// Patch heroku account
fn patch_account<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    let response = api_client.request(&account::AccountUpdate {
        params: account::AccountUpdateParams {
            allow_tracking: Some(true),
            beta: Some(false),
            name: Some(String::from("Heroku-test")),
        },
    });
    print_response(response);
}

// Get heroku account
fn get_account<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    let response = api_client.request(&account::AccountDetails {});
    print_response(response);
}
