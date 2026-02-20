use axum::{
    body::{Body, to_bytes},
    http::{Request, StatusCode},
};
use serde_json::{Value, json};
use tower::ServiceExt;

#[tokio::test]
async fn pizzas_crud_smoke_test() {
    let app = pizza_app::api::router(pizza_app::api::AppState::default());

    // Create
    let res = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/pizzas")
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({
                      "name": "Margherita",
                      "size": "medium",
                      "toppings": ["basil"]
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::CREATED);
    let body = to_bytes(res.into_body(), usize::MAX).await.unwrap();
    let created: Value = serde_json::from_slice(&body).unwrap();
    let id = created
        .get("id")
        .and_then(|v| v.as_str())
        .unwrap()
        .to_string();

    // List
    let res = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/pizzas")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let body = to_bytes(res.into_body(), usize::MAX).await.unwrap();
    let list: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(list.as_array().unwrap().len(), 1);

    // Update
    let res = app
        .clone()
        .oneshot(
            Request::builder()
                .method("PATCH")
                .uri(format!("/pizzas/{id}"))
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({ "name": "Margherita (updated)" }).to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    // Delete
    let res = app
        .clone()
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(format!("/pizzas/{id}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::NO_CONTENT);
}
