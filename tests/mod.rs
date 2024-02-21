#[cfg(test)]

use videas::routes::index;
use actix_web::{http::header::ContentType, test, App};

#[actix_web::test]
async fn test_index_get() {
    let app = test::init_service(App::new().service(index)).await;
    let req = test::TestRequest::default()
                .insert_header(ContentType::html())
                .to_request();
    let res = test::call_service(&app, req).await;

    assert!(res.status().is_success());
}
