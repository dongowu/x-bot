// use reqwest::blocking::Client;
// use serde::Deserialize;
// use std::collections::HashSet;
// use std::fs;
//
// const GITHUB_API: &str = "https://api.github.com";
// const GITHUB_TOKEN: &str = ""; // 替换为你的 GitHub Token
// const OWNER: &str = "dongowu";         // 替换为存储库所有者
// const REPO: &str = "project_rust";          // 替换为存储库名称
// const BRANCH: &str = "main";                   // 主分支名称 ("main" 或 "master")
// const VERSION_FILE: &str = "latest_version.txt"; // 存储最新版本的文件
//
// #[derive(Deserialize, Debug)]
// struct Contributor {
//     login: String,
//     avatar_ur: String,
//     contributions: i16,
// }
//
// #[derive(Deserialize, Debug)]
// struct Commit {
//     sha: String,
//     commit: CommitDetail,
//     author: Option<Author>,
// }
//
// #[derive(Deserialize, Debug)]
// struct CommitDetail {
//     message: String,
//     author: CommitAuthor,
// }
//
// #[derive(Deserialize, Debug)]
// struct CommitAuthor {
//     date: String,
// }
//
// #[derive(Deserialize, Debug)]
// struct Author {
//     login: String,
// }
//
// #[derive(Deserialize, Debug)]
// struct Release {
//     tag_name: String,
//     published_at: String,
//     name: Option<String>,
// }
//
// fn read_stored_version() -> Option<String> {
//     fs::read_to_string(VERSION_FILE).ok()
// }
//
// fn write_stored_version(version: &str) {
//     fs::write(VERSION_FILE, version).unwrap();
// }
//
// fn get_contributors(client: &Client) -> HashSet<String> {
//     let url = format!("{GITHUB_API}/repos/{OWNER}/{REPO}/contributors");
//     let response = client
//         .get(&url)
//         .header("Authorization", format!("token {GITHUB_TOKEN}"))
//         .header("User-Agent", "Rust-GitHub-Client")
//         .send()
//         .unwrap();
//
//     if response.status().is_success() {
//         response
//             .json::<Vec<Contributor>>()
//             .unwrap_or_default()
//             .into_iter()
//             .map(|c| c.login)
//             .collect()
//     } else {
//         println!("获取贡献者失败: {}", response.status());
//         HashSet::new()
//     }
// }
//
// fn get_commits(client: &Client) -> Vec<Commit> {
//     let url = format!("{GITHUB_API}/repos/{OWNER}/{REPO}/commits?sha={BRANCH}");
//     let response = client
//         .get(&url)
//         .header("Authorization", format!("token {GITHUB_TOKEN}"))
//         .header("User-Agent", "Rust-GitHub-Client")
//         .send()
//         .unwrap();
//
//     if response.status().is_success() {
//         response.json::<Vec<Commit>>().unwrap_or_default()
//     } else {
//         println!("获取提交记录失败: {}", response.status());
//         Vec::new()
//     }
// }
//
// fn get_latest_release(client: &Client) -> Option<Release> {
//     let url = format!("{GITHUB_API}/repos/{OWNER}/{REPO}/releases");
//     let response = client
//         .get(&url)
//         .header("Authorization", format!("token {GITHUB_TOKEN}"))
//         .header("User-Agent", "Rust-GitHub-Client")
//         .send()
//         .unwrap();
//
//     if response.status().is_success() {
//         let releases: Vec<Release> = response.json().unwrap_or_default();
//         releases.into_iter().next() // 返回最新版本（按照 API 返回顺序，第一个是最新版本）
//     } else {
//         println!("获取版本信息失败: {}", response.status());
//         None
//     }
// }
//
//
// fn detect_new_contributors() {
//     let client = Client::new();
//
//     // 获取现有贡献者
//     let mut existing_contributors = get_contributors(&client);
//     println!("现有贡献者: {:?}", existing_contributors);
//
//     // 获取分支提交记录
//     let commits = get_commits(&client);
//     let mut new_contributors = Vec::new();
//
//     // 检测新贡献者
//     for commit in commits {
//         if let Some(author) = commit.author {
//             if !existing_contributors.contains(&author.login) {
//                 println!("检测到新贡献者: {}", author.login);
//                 new_contributors.push((
//                     author.login.clone(),
//                     commit.sha,
//                     commit.commit.author.date.clone(),
//                     commit.commit.message.clone(),
//                 ));
//                 existing_contributors.insert(author.login);
//             }
//         }
//     }
//
//     // 打印新贡献者信息
//     if new_contributors.is_empty() {
//         println!("未检测到新贡献者。");
//     } else {
//         for (login, sha, date, message) in new_contributors {
//             println!(
//                 "新贡献者: {}\n  提交 SHA: {}\n  提交时间: {}\n  提交信息: {}",
//                 login, sha, date, message
//             );
//         }
//     }
// }
//
//
// fn detect_new_release() {
//     let client = Client::new();
//
//     // 获取存储的版本号
//     let stored_version = read_stored_version();
//
//
//     // 获取最新版本信息
//     if let Some(latest_release) = get_latest_release(&client) {
//         let latest_version = &latest_release.tag_name;
//
//         if Some(latest_version) != stored_version.as_ref() {
//             println!("检测到新版本发布!");
//             println!(
//                 "版本号: {}\n发布日期: {}\n描述: {:?}",
//                 latest_version, latest_release.published_at, latest_release.name
//             );
//
//             // 更新存储的版本号
//             write_stored_version(latest_version);
//         } else {
//             println!("当前已是最新版本: {}", latest_version);
//         }
//     } else {
//         println!("未能获取最新版本信息。");
//     }
// }
//
// fn main() {
//     detect_new_contributors();
//     detect_new_release();
// }
