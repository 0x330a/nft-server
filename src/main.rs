use std::net::SocketAddr;

use axum::{
    extract::Path,
    http::StatusCode,
    Json,
    Router, routing::get,
};
use serde::Serialize;

#[derive(Serialize)]
struct Metadata {
    name: String,
    description: String,
    image: String,
    attributes: Vec<Attribute>
}

#[derive(Serialize)]
struct Attribute {
    trait_type: String,
    value: String
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/meta/:id", get(get_metadata));
    let addr = SocketAddr::from(([0, 0, 0, 0], 4200));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_metadata(
    Path(id): Path<u32>
) -> Result<Json<Metadata>, StatusCode> {

    let one_meta: Metadata = Metadata {
        name: "Token #1".to_string(),
        description: "The first token".to_string(),
        image: "data:image/svg+xml;base64,PD94bWwgdmVyc2lvbj0iMS4wIiBlbmNvZGluZz0iVVRGLTgiIHN0YW5kYWxvbmU9Im5vIj8+CjwhLS0gQ3JlYXRlZCB3aXRoIElua3NjYXBlIChodHRwOi8vd3d3Lmlua3NjYXBlLm9yZy8pIC0tPgoKPHN2ZwogICB3aWR0aD0iNTEyIgogICBoZWlnaHQ9IjUxMiIKICAgdmlld0JveD0iMCAwIDUxMiA1MTIiCiAgIHZlcnNpb249IjEuMSIKICAgaWQ9IlNWR1Jvb3QiCiAgIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIKICAgeG1sbnM6c3ZnPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyI+CiAgPGRlZnMKICAgICBpZD0iZGVmczU0NyI+CiAgICA8cmVjdAogICAgICAgeD0iOTUuNzQ1MDQxIgogICAgICAgeT0iMTU0LjUxNTU5IgogICAgICAgd2lkdGg9IjQwNC4zNTkwNCIKICAgICAgIGhlaWdodD0iMTQzLjQ3Mjg1IgogICAgICAgaWQ9InJlY3Q2MjEiIC8+CiAgPC9kZWZzPgogIDxnCiAgICAgaWQ9ImxheWVyMSIKICAgICBzdHlsZT0iZGlzcGxheTppbmxpbmUiCiAgICAgdHJhbnNmb3JtPSJ0cmFuc2xhdGUoMC44OTM2MzE0NiwxLjEzOTYyMzgpIj4KICAgIDxyZWN0CiAgICAgICBzdHlsZT0iZmlsbDojZDM1ZjVmIgogICAgICAgaWQ9InJlY3Q1NjUiCiAgICAgICB3aWR0aD0iNTEyIgogICAgICAgaGVpZ2h0PSI1MTIiCiAgICAgICB4PSItMC44OTM2MzE0NiIKICAgICAgIHk9Ii0xLjEzOTYyMzgiCiAgICAgICByeT0iMzIiIC8+CiAgICA8dGV4dAogICAgICAgeG1sOnNwYWNlPSJwcmVzZXJ2ZSIKICAgICAgIGlkPSJ0ZXh0NjE5IgogICAgICAgc3R5bGU9ImZvbnQtc2l6ZTo5NnB4O3doaXRlLXNwYWNlOnByZTtzaGFwZS1pbnNpZGU6dXJsKCNyZWN0NjIxKTtmaWxsOiNkMzVmNWYiCiAgICAgICB0cmFuc2Zvcm09InRyYW5zbGF0ZSg5Mi4xOTM0ODUsNjQuOTQ2MzU5KSI+PHRzcGFuCiAgICAgICAgIHg9Ijk1Ljc0NDE0MSIKICAgICAgICAgeT0iMjM5LjQ1MzEyIgogICAgICAgICBpZD0idHNwYW4xMjUyIj48dHNwYW4KICAgICAgICAgICBzdHlsZT0iZmlsbDojZmZmZmZmIgogICAgICAgICAgIGlkPSJ0c3BhbjEyNTAiPiMxPC90c3Bhbj48L3RzcGFuPjwvdGV4dD4KICA8L2c+Cjwvc3ZnPgo=".to_string(),
        attributes: vec![
            Attribute {
                trait_type: "Number".to_string(),
                value: "1".to_string(),
            },
            Attribute {
                trait_type: "IsLast".to_string(),
                value: "No".to_string(),
            },
        ],
    };

    let two_meta: Metadata = Metadata {
        name: "Token #2".to_string(),
        description: "The second token".to_string(),
        image: "data:image/svg+xml;base64,PD94bWwgdmVyc2lvbj0iMS4wIiBlbmNvZGluZz0iVVRGLTgiIHN0YW5kYWxvbmU9Im5vIj8+CjwhLS0gQ3JlYXRlZCB3aXRoIElua3NjYXBlIChodHRwOi8vd3d3Lmlua3NjYXBlLm9yZy8pIC0tPgoKPHN2ZwogICB3aWR0aD0iNTEyIgogICBoZWlnaHQ9IjUxMiIKICAgdmlld0JveD0iMCAwIDUxMiA1MTIiCiAgIHZlcnNpb249IjEuMSIKICAgaWQ9IlNWR1Jvb3QiCiAgIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIKICAgeG1sbnM6c3ZnPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyI+CiAgPGRlZnMKICAgICBpZD0iZGVmczU0NyI+CiAgICA8cmVjdAogICAgICAgeD0iOTUuNzQ1MDQxIgogICAgICAgeT0iMTU0LjUxNTU5IgogICAgICAgd2lkdGg9IjQwNC4zNTkwNCIKICAgICAgIGhlaWdodD0iMTQzLjQ3Mjg1IgogICAgICAgaWQ9InJlY3Q2MjEiIC8+CiAgPC9kZWZzPgogIDxnCiAgICAgaWQ9ImxheWVyMSIKICAgICBzdHlsZT0iZGlzcGxheTppbmxpbmUiCiAgICAgdHJhbnNmb3JtPSJ0cmFuc2xhdGUoMC44OTM2MzE0NiwxLjEzOTYyMzgpIj4KICAgIDxyZWN0CiAgICAgICBzdHlsZT0iZmlsbDojZDM1ZjVmIgogICAgICAgaWQ9InJlY3Q1NjUiCiAgICAgICB3aWR0aD0iNTEyIgogICAgICAgaGVpZ2h0PSI1MTIiCiAgICAgICB4PSItMC44OTM2MzE0NiIKICAgICAgIHk9Ii0xLjEzOTYyMzgiCiAgICAgICByeT0iMzIiIC8+CiAgICA8dGV4dAogICAgICAgeG1sOnNwYWNlPSJwcmVzZXJ2ZSIKICAgICAgIGlkPSJ0ZXh0NjE5IgogICAgICAgc3R5bGU9ImZvbnQtc2l6ZTo5NnB4O3doaXRlLXNwYWNlOnByZTtzaGFwZS1pbnNpZGU6dXJsKCNyZWN0NjIxKTtmaWxsOiNkMzVmNWYiCiAgICAgICB0cmFuc2Zvcm09InRyYW5zbGF0ZSg5Mi4xOTM0ODUsNjQuOTQ2MzU5KSI+PHRzcGFuCiAgICAgICAgIHg9Ijk1Ljc0NDE0MSIKICAgICAgICAgeT0iMjM5LjQ1MzEyIgogICAgICAgICBpZD0idHNwYW4xMzcyIj48dHNwYW4KICAgICAgICAgICBzdHlsZT0iZmlsbDojZmZmZmZmIgogICAgICAgICAgIGlkPSJ0c3BhbjEzNzAiPiMyPC90c3Bhbj48L3RzcGFuPjwvdGV4dD4KICA8L2c+Cjwvc3ZnPgo=".to_string(),
        attributes: vec![
            Attribute {
                trait_type: "Number".to_string(),
                value: "2".to_string(),
            },
            Attribute {
                trait_type: "IsLast".to_string(),
                value: "No".to_string(),
            },
        ],
    };

    let three_meta: Metadata = Metadata {
        name: "Token #3".to_string(),
        description: "The third token".to_string(),
        image: "data:image/svg+xml;base64,PD94bWwgdmVyc2lvbj0iMS4wIiBlbmNvZGluZz0iVVRGLTgiIHN0YW5kYWxvbmU9Im5vIj8+CjwhLS0gQ3JlYXRlZCB3aXRoIElua3NjYXBlIChodHRwOi8vd3d3Lmlua3NjYXBlLm9yZy8pIC0tPgoKPHN2ZwogICB3aWR0aD0iNTEyIgogICBoZWlnaHQ9IjUxMiIKICAgdmlld0JveD0iMCAwIDUxMiA1MTIiCiAgIHZlcnNpb249IjEuMSIKICAgaWQ9IlNWR1Jvb3QiCiAgIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIKICAgeG1sbnM6c3ZnPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyI+CiAgPGRlZnMKICAgICBpZD0iZGVmczU0NyI+CiAgICA8cmVjdAogICAgICAgeD0iOTUuNzQ1MDQxIgogICAgICAgeT0iMTU0LjUxNTU5IgogICAgICAgd2lkdGg9IjQwNC4zNTkwNCIKICAgICAgIGhlaWdodD0iMTQzLjQ3Mjg1IgogICAgICAgaWQ9InJlY3Q2MjEiIC8+CiAgPC9kZWZzPgogIDxnCiAgICAgaWQ9ImxheWVyMSIKICAgICBzdHlsZT0iZGlzcGxheTppbmxpbmUiCiAgICAgdHJhbnNmb3JtPSJ0cmFuc2xhdGUoMC44OTM2MzE0NiwxLjEzOTYyMzgpIj4KICAgIDxyZWN0CiAgICAgICBzdHlsZT0iZmlsbDojZDM1ZjVmIgogICAgICAgaWQ9InJlY3Q1NjUiCiAgICAgICB3aWR0aD0iNTEyIgogICAgICAgaGVpZ2h0PSI1MTIiCiAgICAgICB4PSItMC44OTM2MzE0NiIKICAgICAgIHk9Ii0xLjEzOTYyMzgiCiAgICAgICByeT0iMzIiIC8+CiAgICA8dGV4dAogICAgICAgeG1sOnNwYWNlPSJwcmVzZXJ2ZSIKICAgICAgIGlkPSJ0ZXh0NjE5IgogICAgICAgc3R5bGU9ImZvbnQtc2l6ZTo5NnB4O3doaXRlLXNwYWNlOnByZTtzaGFwZS1pbnNpZGU6dXJsKCNyZWN0NjIxKTtmaWxsOiNkMzVmNWYiCiAgICAgICB0cmFuc2Zvcm09InRyYW5zbGF0ZSg5Mi4xOTM0ODUsNjQuOTQ2MzU5KSI+PHRzcGFuCiAgICAgICAgIHg9Ijk1Ljc0NDE0MSIKICAgICAgICAgeT0iMjM5LjQ1MzEyIgogICAgICAgICBpZD0idHNwYW4xNTA4Ij48dHNwYW4KICAgICAgICAgICBzdHlsZT0iZmlsbDojZmZmZmZmIgogICAgICAgICAgIGlkPSJ0c3BhbjE1MDYiPiMzPC90c3Bhbj48L3RzcGFuPjwvdGV4dD4KICA8L2c+Cjwvc3ZnPgo=".to_string(),
        attributes: vec![
            Attribute {
                trait_type: "Number".to_string(),
                value: "3".to_string(),
            },
            Attribute {
                trait_type: "IsLast".to_string(),
                value: "No".to_string(),
            },
        ],
    };

    let four_meta: Metadata = Metadata {
        name: "Token #4".to_string(),
        description: "The fourth token".to_string(),
        image: "data:image/svg+xml;base64,PD94bWwgdmVyc2lvbj0iMS4wIiBlbmNvZGluZz0iVVRGLTgiIHN0YW5kYWxvbmU9Im5vIj8+CjwhLS0gQ3JlYXRlZCB3aXRoIElua3NjYXBlIChodHRwOi8vd3d3Lmlua3NjYXBlLm9yZy8pIC0tPgoKPHN2ZwogICB3aWR0aD0iNTEyIgogICBoZWlnaHQ9IjUxMiIKICAgdmlld0JveD0iMCAwIDUxMiA1MTIiCiAgIHZlcnNpb249IjEuMSIKICAgaWQ9IlNWR1Jvb3QiCiAgIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIKICAgeG1sbnM6c3ZnPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyI+CiAgPGRlZnMKICAgICBpZD0iZGVmczU0NyI+CiAgICA8cmVjdAogICAgICAgeD0iOTUuNzQ1MDQxIgogICAgICAgeT0iMTU0LjUxNTU5IgogICAgICAgd2lkdGg9IjQwNC4zNTkwNCIKICAgICAgIGhlaWdodD0iMTQzLjQ3Mjg1IgogICAgICAgaWQ9InJlY3Q2MjEiIC8+CiAgPC9kZWZzPgogIDxnCiAgICAgaWQ9ImxheWVyMSIKICAgICBzdHlsZT0iZGlzcGxheTppbmxpbmUiCiAgICAgdHJhbnNmb3JtPSJ0cmFuc2xhdGUoMC44OTM2MzE0NiwxLjEzOTYyMzgpIj4KICAgIDxyZWN0CiAgICAgICBzdHlsZT0iZmlsbDojZDM1ZjVmIgogICAgICAgaWQ9InJlY3Q1NjUiCiAgICAgICB3aWR0aD0iNTEyIgogICAgICAgaGVpZ2h0PSI1MTIiCiAgICAgICB4PSItMC44OTM2MzE0NiIKICAgICAgIHk9Ii0xLjEzOTYyMzgiCiAgICAgICByeT0iMzIiIC8+CiAgICA8dGV4dAogICAgICAgeG1sOnNwYWNlPSJwcmVzZXJ2ZSIKICAgICAgIGlkPSJ0ZXh0NjE5IgogICAgICAgc3R5bGU9ImZvbnQtc2l6ZTo5NnB4O3doaXRlLXNwYWNlOnByZTtzaGFwZS1pbnNpZGU6dXJsKCNyZWN0NjIxKTtmaWxsOiNkMzVmNWYiCiAgICAgICB0cmFuc2Zvcm09InRyYW5zbGF0ZSg5Mi4xOTM0ODUsNjQuOTQ2MzU5KSI+PHRzcGFuCiAgICAgICAgIHg9Ijk1Ljc0NDE0MSIKICAgICAgICAgeT0iMjM5LjQ1MzEyIgogICAgICAgICBpZD0idHNwYW4xNjE0Ij48dHNwYW4KICAgICAgICAgICBzdHlsZT0iZmlsbDojZmZmZmZmIgogICAgICAgICAgIGlkPSJ0c3BhbjE2MTIiPiM0PC90c3Bhbj48L3RzcGFuPjwvdGV4dD4KICA8L2c+Cjwvc3ZnPgo=".to_string(),
        attributes: vec![
            Attribute {
                trait_type: "Number".to_string(),
                value: "4".to_string(),
            },
            Attribute {
                trait_type: "IsLast".to_string(),
                value: "No".to_string(),
            },
        ],
    };

    let five_meta: Metadata = Metadata {
        name: "Token #5".to_string(),
        description: "The fifth token".to_string(),
        image: "data:image/svg+xml;base64,PD94bWwgdmVyc2lvbj0iMS4wIiBlbmNvZGluZz0iVVRGLTgiIHN0YW5kYWxvbmU9Im5vIj8+CjwhLS0gQ3JlYXRlZCB3aXRoIElua3NjYXBlIChodHRwOi8vd3d3Lmlua3NjYXBlLm9yZy8pIC0tPgoKPHN2ZwogICB3aWR0aD0iNTEyIgogICBoZWlnaHQ9IjUxMiIKICAgdmlld0JveD0iMCAwIDUxMiA1MTIiCiAgIHZlcnNpb249IjEuMSIKICAgaWQ9IlNWR1Jvb3QiCiAgIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIKICAgeG1sbnM6c3ZnPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyI+CiAgPGRlZnMKICAgICBpZD0iZGVmczU0NyI+CiAgICA8cmVjdAogICAgICAgeD0iOTUuNzQ1MDQxIgogICAgICAgeT0iMTU0LjUxNTU5IgogICAgICAgd2lkdGg9IjQwNC4zNTkwNCIKICAgICAgIGhlaWdodD0iMTQzLjQ3Mjg1IgogICAgICAgaWQ9InJlY3Q2MjEiIC8+CiAgPC9kZWZzPgogIDxnCiAgICAgaWQ9ImxheWVyMSIKICAgICBzdHlsZT0iZGlzcGxheTppbmxpbmUiCiAgICAgdHJhbnNmb3JtPSJ0cmFuc2xhdGUoMC44OTM2MzE0NiwxLjEzOTYyMzgpIj4KICAgIDxyZWN0CiAgICAgICBzdHlsZT0iZmlsbDojZDM1ZjVmIgogICAgICAgaWQ9InJlY3Q1NjUiCiAgICAgICB3aWR0aD0iNTEyIgogICAgICAgaGVpZ2h0PSI1MTIiCiAgICAgICB4PSItMC44OTM2MzE0NiIKICAgICAgIHk9Ii0xLjEzOTYyMzgiCiAgICAgICByeT0iMzIiIC8+CiAgICA8dGV4dAogICAgICAgeG1sOnNwYWNlPSJwcmVzZXJ2ZSIKICAgICAgIGlkPSJ0ZXh0NjE5IgogICAgICAgc3R5bGU9ImZvbnQtc2l6ZTo5NnB4O3doaXRlLXNwYWNlOnByZTtzaGFwZS1pbnNpZGU6dXJsKCNyZWN0NjIxKTtmaWxsOiNkMzVmNWYiCiAgICAgICB0cmFuc2Zvcm09InRyYW5zbGF0ZSg5Mi4xOTM0ODUsNjQuOTQ2MzU5KSI+PHRzcGFuCiAgICAgICAgIHg9Ijk1Ljc0NDE0MSIKICAgICAgICAgeT0iMjM5LjQ1MzEyIgogICAgICAgICBpZD0idHNwYW4xNzIwIj48dHNwYW4KICAgICAgICAgICBzdHlsZT0iZmlsbDojZmZmZmZmIgogICAgICAgICAgIGlkPSJ0c3BhbjE3MTgiPiM1PC90c3Bhbj48L3RzcGFuPjwvdGV4dD4KICA8L2c+Cjwvc3ZnPgo=".to_string(),
        attributes: vec![
            Attribute {
                trait_type: "Number".to_string(),
                value: "5".to_string(),
            },
            Attribute {
                trait_type: "IsLast".to_string(),
                value: "Yes".to_string(),
            },
        ],
    };

    match id {
        1 => Ok(Json(one_meta)),
        2 => Ok(Json(two_meta)),
        3 => Ok(Json(three_meta)),
        4 => Ok(Json(four_meta)),
        5 => Ok(Json(five_meta)),
        _ => Err(StatusCode::BAD_REQUEST)
    }
}