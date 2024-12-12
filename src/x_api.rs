use anyhow::Result;
use twitter_v2::authorization::BearerToken;
use twitter_v2::TwitterApi;

pub struct XHandler {
    client: TwitterApi<BearerToken>,
}

impl XHandler {
    pub fn new(bearer_token: String) -> Result<Self> {
        let client = TwitterApi::new(BearerToken::new(bearer_token));
        Ok(Self { client })
    }

    pub async fn post_new_contributor(&self, info: &super::github::ContributorInfo) -> Result<()> {
        let text = format!(
            "Delta got a new contributor {}!\n\nDetails: {}\n\nLink: {}",
            info.name, info.message, info.link
        );
        
        self.client.post_tweet().text(text).send().await?;
        Ok(())
    }

    pub async fn post_new_release(&self, info: &super::github::ReleaseInfo) -> Result<()> {
        let text = format!(
            "New release ({}) of Delta out! 🎉\n\nLink to release notes: {}",
            info.version, info.link
        );
        
        self.client.post_tweet().text(text).send().await?;
        Ok(())
    }
} 