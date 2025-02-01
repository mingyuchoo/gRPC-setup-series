use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use warp::Filter;

#[derive(Serialize, Deserialize, Debug)]
struct UserRequest {
    id: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct UserReply {
    id:            String,
    first_name:    String,
    last_name:     String,
    date_of_birth: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct CreateUserRequest {
    first_name:    String,
    last_name:     String,
    date_of_birth: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct CreateUserReply {
    message: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct UpdateUserRequest {
    id:            String,
    first_name:    String,
    last_name:     String,
    date_of_birth: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct UpdateUserReply {
    message: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct DeleteUserReply {
    message: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Users {
    users: Vec<UserReply>,
}

#[tokio::main]
async fn main() {
    let client = Arc::new(Client::new());

    let get_user = {
        let client = Arc::clone(&client);
        warp::path!("user" / String).and(warp::get()).and_then(
            move |id: String| {
                let client = Arc::clone(&client);
                async move {
                    let req = UserRequest {
                        id,
                    };
                    let res = client
                        .post("http://[::1]:50051/user")
                        .json(&req)
                        .send()
                        .await
                        .unwrap();

                    let user: UserReply = res.json().await.unwrap();
                    Ok::<_, warp::Rejection>(warp::reply::json(&user))
                }
            },
        )
    };

    let list_users = {
        let client = Arc::clone(&client);
        warp::path("users").and(warp::get()).and_then(move || {
            let client = Arc::clone(&client);
            async move {
                let res = client
                    .post("http://[::1]:50051/users")
                    .send()
                    .await
                    .unwrap();

                let users: Users = res.json().await.unwrap();
                Ok::<_, warp::Rejection>(warp::reply::json(&users))
            }
        })
    };

    let create_user = {
        let client = Arc::clone(&client);
        warp::path("user")
            .and(warp::post())
            .and(warp::body::json())
            .and_then(move |new_user: CreateUserRequest| {
                let client = Arc::clone(&client);
                async move {
                    let res = client
                        .post("http://[::1]:50051/user/create")
                        .json(&new_user)
                        .send()
                        .await
                        .unwrap();

                    let reply: CreateUserReply = res.json().await.unwrap();
                    Ok::<_, warp::Rejection>(warp::reply::json(&reply))
                }
            })
    };

    let update_user = {
        let client = Arc::clone(&client);
        warp::path("user")
            .and(warp::put())
            .and(warp::body::json())
            .and_then(move |update_user: UpdateUserRequest| {
                let client = Arc::clone(&client);
                async move {
                    let res = client
                        .post("http://[::1]:50051/user/update")
                        .json(&update_user)
                        .send()
                        .await
                        .unwrap();

                    let reply: UpdateUserReply = res.json().await.unwrap();
                    Ok::<_, warp::Rejection>(warp::reply::json(&reply))
                }
            })
    };

    let delete_user = {
        let client = Arc::clone(&client);
        warp::path!("user" / String).and(warp::delete()).and_then(
            move |id: String| {
                let client = Arc::clone(&client);
                async move {
                    let req = UserRequest {
                        id,
                    };
                    let res = client
                        .post("http://[::1]:50051/user/delete")
                        .json(&req)
                        .send()
                        .await
                        .unwrap();

                    let reply: DeleteUserReply = res.json().await.unwrap();
                    Ok::<_, warp::Rejection>(warp::reply::json(&reply))
                }
            },
        )
    };

    let delete_users = {
        let client = Arc::clone(&client);
        warp::path("users").and(warp::delete()).and_then(move || {
            let client = Arc::clone(&client);
            async move {
                let res = client
                    .post("http://[::1]:50051/users/delete")
                    .send()
                    .await
                    .unwrap();

                let reply: DeleteUserReply = res.json().await.unwrap();
                Ok::<_, warp::Rejection>(warp::reply::json(&reply))
            }
        })
    };

    let routes = get_user
        .or(list_users)
        .or(create_user)
        .or(update_user)
        .or(delete_user)
        .or(delete_users);

    warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;
}
