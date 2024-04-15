use actix_web::{
    http::{self, header::ContentType},
    test, App,
};
#[cfg(test)]
// TODO: Add steps or a bash script to run tests with a test DB
use videas::routes::*;

#[actix_web::test]
async fn test_index_get() {
    let app = test::init_service(App::new().service(index)).await;
    let req = test::TestRequest::default()
        .insert_header(ContentType::html())
        .to_request();
    let res = test::call_service(&app, req).await;

    assert!(res.status().is_success());
}

#[actix_web::test]
async fn test_show_non_existent_post_get() {
    let app = test::init_service(App::new().service(show_post)).await;
    let req = test::TestRequest::get()
        .uri("/this-post-doesnt-exist")
        .insert_header(ContentType::html())
        .to_request();
    let res = test::call_service(&app, req).await;

    assert!(res.status().is_client_error());
}

#[actix_web::test]
async fn test_show_post_get() {
    let app = test::init_service(App::new().service(show_post)).await;
    let req = test::TestRequest::get()
        // TODO: make sure you ran the seeds
        .uri("/using-github-copilot-vim")
        .insert_header(ContentType::html())
        .to_request();
    let res = test::call_service(&app, req).await;

    assert_eq!(res.status(), http::StatusCode::NOT_FOUND.as_u16());
}
