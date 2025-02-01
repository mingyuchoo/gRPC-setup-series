// service.rs
use crate::db_connection::establish_connection;
use crate::user::user_service_server::UserService;
use crate::user::{CreateUserReply, CreateUserRequest, DeleteUserReply, Empty, UpdateUserReply, UpdateUserRequest, UserReply, UserRequest, Users};
use tonic::{Request, Response, Status};
use uuid::Uuid;

#[derive(Default)]
pub struct User {}

#[tonic::async_trait]
impl UserService for User {
    async fn get_user(
        &self,
        request: Request<UserRequest>,
    ) -> Result<Response<UserReply>, Status> {
        println!("Got a request: {:#?}", &request);
        let UserRequest {
            id,
        } = &request.into_inner();

        let mut conn = establish_connection();

        let rows = conn
            .query("SELECT * FROM users WHERE id = $1", &[&id])
            .unwrap();

        if rows.is_empty() {
            return Err(Status::not_found("User not found"));
        }

        let row = rows.get(0).expect("Row should exist");

        let date_of_birth: String = row.get(3);

        let reply = UserReply {
            id: row.get(0),
            first_name: row.get(1),
            last_name: row.get(2),
            date_of_birth,
        };

        Ok(Response::new(reply))
    }

    async fn list_users(
        &self,
        request: Request<Empty>,
    ) -> Result<Response<Users>, Status> {
        println!("Got a request: {:#?}", &request);
        let mut conn = establish_connection();
        let mut v: Vec<UserReply> = Vec::new();

        for row in conn.query("SELECT * FROM users", &[]).unwrap() {
            let date_of_birth: String = row.get(3);
            let user = UserReply {
                id: row.get(0),
                first_name: row.get(1),
                last_name: row.get(2),
                date_of_birth,
            };
            v.push(user);
        }

        let reply = Users {
            users: v
        };

        Ok(Response::new(reply))
    }

    async fn create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<CreateUserReply>, Status> {
        println!("Got a request: {:#?}", &request);
        let user_id = Uuid::new_v4().hyphenated().to_string();
        let CreateUserRequest {
            first_name,
            last_name,
            date_of_birth,
        } = &request.into_inner();
        let serialize_date_of_birth = date_of_birth.clone();

        let mut conn = establish_connection();
        let number_of_rows_affected = conn
            .execute(
                "INSERT INTO users (id, first_name, last_name, date_of_birth) VALUES ($1, $2, $3, $4)",
                &[&user_id, &first_name, &last_name, &serialize_date_of_birth],
            )
            .unwrap();

        let reply = if number_of_rows_affected == 0 {
            CreateUserReply {
                message: format!("Fail to create user with id {}.", &user_id),
            }
        } else {
            CreateUserReply {
                message: format!(
                    "Create {} user with id {}.",
                    &number_of_rows_affected, &user_id
                ),
            }
        };

        Ok(Response::new(reply))
    }

    async fn update_user(
        &self,
        request: Request<UpdateUserRequest>,
    ) -> Result<Response<UpdateUserReply>, Status> {
        println!("Got a request: {:#?}", &request);
        let UpdateUserRequest {
            id,
            first_name,
            last_name,
            date_of_birth,
        } = &request.into_inner();

        let serialize_date_of_birth = date_of_birth.clone();

        let mut conn = establish_connection();

        let number_of_rows_affected = conn
            .execute(
                "UPDATE users SET first_name = $2, last_name = $3, date_of_birth = $4 WHERE id = $1",
                &[&id, &first_name, &last_name, &serialize_date_of_birth],
            )
            .unwrap();

        let reply = if number_of_rows_affected == 0 {
            UpdateUserReply {
                message: format!("Fail to update the user with id {}.", id),
            }
        } else {
            UpdateUserReply {
                message: format!(
                    "Update {} user with id {}.",
                    &number_of_rows_affected, &id
                ),
            }
        };

        Ok(Response::new(reply))
    }

    async fn delete_user(
        &self,
        request: Request<UserRequest>,
    ) -> Result<Response<DeleteUserReply>, Status> {
        println!("Got a request: {:#?}", &request);
        let UserRequest {
            id,
        } = &request.into_inner();
        let mut conn = establish_connection();

        let number_of_rows_affected = conn
            .execute("DELETE FROM users WHERE id = $1", &[&id])
            .unwrap();

        let reply = if number_of_rows_affected == 0 {
            DeleteUserReply {
                message: format!("Fail to delete the user with id {}.", id),
            }
        } else {
            DeleteUserReply {
                message: format!("Remove the user with id {}.", id),
            }
        };

        Ok(Response::new(reply))
    }

    async fn delete_users(
        &self,
        request: Request<Empty>,
    ) -> Result<Response<DeleteUserReply>, Status> {
        println!("Got a request: {:#?}", &request);
        let mut conn = establish_connection();

        let number_of_rows_affected =
            conn.execute("DELETE FROM users", &[]).unwrap();

        let reply = DeleteUserReply {
            message: format!(
                "Remove {} user data from the database.",
                number_of_rows_affected
            ),
        };

        Ok(Response::new(reply))
    }
}
