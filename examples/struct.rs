use feignhttp::feign;

const GITHUB_URL: &str = "https://api.github.com";

struct Github;

#[feign(url = GITHUB_URL)]
impl Github {
    #[get]
    async fn home() -> feignhttp::Result<String> {}

    #[get("/repos/{owner}/{repo}")]
    async fn repository(
        #[path("owner")] user: &str,
        #[path] repo: &str,
    ) -> feignhttp::Result<String> {}

    #[get(path = "/repos/{owner}/{repo}/contributors")]
    async fn contributors(
        #[path] owner: &str,
        #[path] repo: &str,
        #[query] page: u32,
    ) -> feignhttp::Result<String> {}

    #[get("/repos/{owner}/{repo}/commits")]
    async fn commits(
        #[header] accept: &str,
        #[path] owner: &str,
        #[path] repo: &str,
        #[query] page: u32,
        #[query] per_page: u32,
    ) -> feignhttp::Result<String> {}

    // Structure method still send request.
    #[get(path = "/repos/{owner}/{repo}/languages")]
    async fn languages(
        &self,
        #[path] owner: &str,
        #[path] repo: &str,
    ) -> feignhttp::Result<String> {}
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let r = Github::home().await?;
    println!("github result: {}\n", r);

    let r = Github::repository("dxx", "feignhttp").await?;
    println!("repository result: {}\n", r);

    let r = Github::contributors("dxx", "feignhttp", 1).await?;
    println!("contributors result: {}\n", r);

    let r = Github::commits(
        "application/vnd.github.v3+json",
        "dxx",
        "feignhttp",
        1,
        1,
    )
    .await?;
    println!("commits result: {}\n", r);

    let r = Github {}.languages("dxx", "feignhttp").await?;
    println!("languages result: {}\n", r);

    Ok(())
}
