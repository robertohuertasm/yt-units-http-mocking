use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    name: String,
}

pub struct UserRepo {
    base_url: String,
    client: reqwest::Client,
}

impl Default for UserRepo {
    fn default() -> Self {
        Self::new("https://jsonplaceholder.typicode.com".to_string())
    }
}

impl UserRepo {
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            client: reqwest::Client::new(),
        }
    }

    pub async fn get_first(&self) -> anyhow::Result<User> {
        let url = format!("{}/users/1", self.base_url);
        let user = self.client.get(&url).send().await?.json().await?;
        Ok(user)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn it_works() {
        let repo = UserRepo::default();
        let user = repo.get_first().await.unwrap();
        println!("{:?}", user);
        assert!(!user.name.is_empty());
    }

    #[tokio::test]
    async fn it_works_with_httpmock() {
        const NAME: &'static str = "John";
        let server = httpmock::MockServer::start();
        let mock = server.mock(|when, then| {
            when.path("/users/1");
            then.body(format!(r#"{{"name": "{}"}}"#, NAME));
        });
        let repo = UserRepo::new(server.base_url());
        let user = repo.get_first().await.unwrap();
        println!("{:?}", user);

        mock.assert();
        assert!(!user.name.is_empty());
        assert_eq!(&user.name, NAME);
    }
}
