use anyhow::Result;
use octocrab::Octocrab;
use std::collections::HashSet;

pub struct GitHubHandler {
    client: Octocrab,
    repo_owner: String,
    repo_name: String,
    known_contributors: HashSet<String>,
}

impl GitHubHandler {
    pub async fn new(token: String, repo_owner: String, repo_name: String) -> Result<Self> {
        let client = Octocrab::builder()
            .personal_token(token)
            .build()?;

        let mut handler = Self {
            client,
            repo_owner,
            repo_name,
            known_contributors: HashSet::new(),
        };

        handler.load_initial_contributors().await?;
        Ok(handler)
    }

    async fn load_initial_contributors(&mut self) -> Result<()> {
        let contributors = self.client
            .repos(&self.repo_owner, &self.repo_name)
            .list_commits()
            .send()
            .await?;

        for commit in contributors {
            if let Some(author) = commit.author {
                self.known_contributors.insert(author.login);
            }
        }
        Ok(())
    }

    pub async fn check_new_contributor(&mut self) -> Result<Option<ContributorInfo>> {
        let commits = self.client
            .repos(&self.repo_owner, &self.repo_name)
            .list_commits()
            .branch("master")
            .send()
            .await?;

        for commit in commits {
            if let Some(author) = commit.author {
                let login = author.login;
                if !self.known_contributors.contains(&login) {
                    self.known_contributors.insert(login.clone());

                    let message = commit.commit.message;
                    let link = commit.html_url.to_string();

                    return Ok(Some(ContributorInfo {
                        name: login,
                        message,
                        link,
                    }));
                }
            }
        }
        Ok(None)
    }

    pub async fn check_new_release(&self) -> Result<Option<ReleaseInfo>> {
        if let Ok(latest) = self.client
            .repos(&self.repo_owner, &self.repo_name)
            .releases()
            .get_latest()
            .await
        {
            Ok(Some(ReleaseInfo {
                version: latest.tag_name,
                link: latest.html_url.to_string(),
            }))
        } else {
            Ok(None)
        }
    }
}

#[derive(Debug)]
pub struct ContributorInfo {
    pub name: String,
    pub message: String,
    pub link: String,
}

#[derive(Debug)]
pub struct ReleaseInfo {
    pub version: String,
    pub link: String,
}