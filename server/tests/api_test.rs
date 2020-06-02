#[cfg(test)]
mod tests {
    use rolecall::api::Api;
    use rolecall::db::DbManager;
    use std::sync::Arc;
    use std::collections::HashMap;
    use std::thread;

    #[tokio::test]
    async fn test_user_api() {
        dotenv::dotenv().unwrap();
        let db = DbManager::new().await.unwrap();
        db.clear_tables().await.unwrap();
        db.create_tables().await.unwrap();
        let db = Arc::new(db);
        let api = Api::new(db.clone());
        thread::spawn(move || api.start());

        // Post a new user
        let email = "post@email.com";
        let pw = "password";
        let mut user_map = HashMap::new();
        user_map.insert("email", email);
        user_map.insert("password", pw);

        let client = reqwest::Client::new();
        let res: rolecall::api::UserResponse = client.post("http://localhost:8000/api/users")
            .json(&user_map)
            .send()
            .await.unwrap()
            .json()
            .await.unwrap();
        assert!(res.status);
        assert!(res.msg.is_none());
        assert!(res.token.is_none());

        // Create a test user
        let email = "test@email.com";
        let pw = "password";
        let token = db.create_user(email, pw).await.unwrap();
        db.confirm_user(email, &token).await.unwrap();

        // Attempt to authenticate
        let mut user_map = HashMap::new();
        user_map.insert("email", email);
        user_map.insert("password", pw);
        let res: rolecall::api::UserResponse = client.post("http://localhost:8000/api/users/auth")
            .json(&user_map)
            .send()
            .await.unwrap()
            .json()
            .await.unwrap();
        assert!(res.status);
        assert!(res.msg.is_none());
        assert!(res.token.is_some());
        let host_token = res.token.unwrap();

        // Try with incorrect password
        let pw = "not-password";
        let mut user_map = HashMap::new();
        user_map.insert("email", email);
        user_map.insert("password", pw);
        let res: rolecall::api::UserResponse = client.post("http://localhost:8000/api/users/auth")
            .json(&user_map)
            .send()
            .await.unwrap()
            .json()
            .await.unwrap();
        assert!(!res.status);
        assert!(res.msg.is_some());
        assert!(res.token.is_none());

        // Try with incorrect email
        let email = "not-email@email.com";
        let mut user_map = HashMap::new();
        user_map.insert("email", email);
        user_map.insert("password", pw);
        let res: rolecall::api::UserResponse = client.post("http://localhost:8000/api/users/auth")
            .json(&user_map)
            .send()
            .await.unwrap()
            .json()
            .await.unwrap();
        assert!(!res.status);
        assert!(res.msg.is_some());
        assert!(res.token.is_none());

        // Try to create a game
        let mut game_map = HashMap::new();
        game_map.insert("token", host_token);
        game_map.insert("name", "Test Game".to_string());
        let res: rolecall::api::GameResponse = client.post("http://localhost:8000/api/games")
            .json(&game_map)
            .send()
            .await.unwrap()
            .json()
            .await.unwrap();
        assert!(res.status);
        assert!(res.msg.is_none());
        assert!(res.token.is_some());
        let game_token = res.token.unwrap();

        // Log in as player
        let mut user_map = HashMap::new();
        user_map.insert("email", "player");
        user_map.insert("password", "password");
        let res: rolecall::api::UserResponse = client.post("http://localhost:8000/api/users/auth")
            .json(&user_map)
            .send()
            .await.unwrap()
            .json()
            .await.unwrap();
        let player_token = res.token.unwrap();

        // Attempt to join game
        let mut req_map = HashMap::new();
        req_map.insert("token", player_token);
        req_map.insert("nick", "nickname".to_string());
        let addr = format!("http://localhost:8000/api/games/{}/join", game_token);
        let res: rolecall::api::JoinResponse = client.post(&addr)
            .json(&req_map)
            .send()
            .await.unwrap()
            .json()
            .await.unwrap();
        assert!(res.status);
        assert!(res.msg.is_none());
    }
}