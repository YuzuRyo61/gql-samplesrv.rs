#[cfg(test)]
mod tests {
    extern crate gql_samplesrv;

    use actix_web::{test, App};
    use gql_samplesrv::routes;

    #[actix_rt::test]
    async fn get_index() {
        let mut app = test::init_service(
            App::new()
                .configure(routes::core)
        ).await;
        let req = test::TestRequest::get().uri("/").to_request();
        let res = test::call_service(&mut app, req).await;
        assert!(res.status().is_success());
    }
}
