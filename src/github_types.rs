#![allow(dead_code)]
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
  pub login: String,
  pub id: u64,
  pub node_id: String,
  pub avatar_url: String,
  pub gravatar_id: Option<String>,
  pub url: String,
  pub html_url: String,
  pub followers_url: String,
  pub following_url: String,
  pub gists_url: String,
  pub starred_url: String,
  pub subscriptions_url: String,
  pub organizations_url: String,
  pub repos_url: String,
  pub events_url: String,
  pub received_events_url: String,
  #[serde(rename = "type")]
  pub user_type: String,
  pub site_admin: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Repository {
  pub id: u64,
  pub node_id: String,
  pub name: String,
  pub full_name: String,
  pub private: bool,
  pub owner: User,
  pub html_url: String,
  pub description: Option<String>,
  pub fork: bool,
  pub url: String,
  pub created_at: String,
  pub updated_at: String,
  pub pushed_at: Option<String>,
  pub git_url: String,
  pub ssh_url: String,
  pub clone_url: String,
  pub svn_url: String,
  pub homepage: Option<String>,
  pub size: u64,
  pub stargazers_count: u64,
  pub watchers_count: u64,
  pub language: Option<String>,
  pub has_issues: bool,
  pub has_projects: bool,
  pub has_downloads: bool,
  pub has_wiki: bool,
  pub has_pages: bool,
  pub has_discussions: Option<bool>,
  pub forks_count: u64,
  pub archived: bool,
  pub disabled: bool,
  pub open_issues_count: u64,
  pub license: Option<serde_json::Value>,
  pub allow_forking: Option<bool>,
  pub is_template: Option<bool>,
  pub web_commit_signoff_required: Option<bool>,
  pub topics: Option<Vec<String>>,
  pub visibility: Option<String>,
  pub forks: u64,
  pub open_issues: u64,
  pub watchers: u64,
  pub default_branch: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Organization {
  pub login: String,
  pub id: u64,
  pub node_id: String,
  pub url: String,
  pub repos_url: String,
  pub events_url: String,
  pub hooks_url: String,
  pub issues_url: String,
  pub members_url: String,
  pub public_members_url: String,
  pub avatar_url: String,
  pub description: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Installation {
  pub id: u64,
  pub node_id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Enterprise {
  pub id: u64,
  pub slug: String,
  pub name: String,
  pub node_id: String,
  pub avatar_url: String,
  pub description: Option<String>,
  pub website_url: Option<String>,
  pub html_url: String,
  pub created_at: String,
  pub updated_at: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Commit {
  pub id: String,
  pub tree_id: String,
  pub distinct: bool,
  pub message: String,
  pub timestamp: String,
  pub url: String,
  pub author: CommitAuthor,
  pub committer: CommitAuthor,
  pub added: Vec<String>,
  pub removed: Vec<String>,
  pub modified: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CommitAuthor {
  pub name: String,
  pub email: String,
  pub username: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Issue {
  pub id: u64,
  pub node_id: String,
  pub url: String,
  pub repository_url: String,
  pub labels_url: String,
  pub comments_url: String,
  pub events_url: String,
  pub html_url: String,
  pub number: u64,
  pub state: String,
  pub title: String,
  pub body: Option<String>,
  pub user: User,
  pub labels: Vec<Label>,
  pub assignee: Option<User>,
  pub assignees: Vec<User>,
  pub milestone: Option<serde_json::Value>,
  pub locked: bool,
  pub active_lock_reason: Option<String>,
  pub comments: u64,
  pub pull_request: Option<serde_json::Value>,
  pub closed_at: Option<String>,
  pub created_at: String,
  pub updated_at: String,
  pub author_association: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Label {
  pub id: u64,
  pub node_id: String,
  pub url: String,
  pub name: String,
  pub description: Option<String>,
  pub color: String,
  pub default: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PullRequest {
  pub id: u64,
  pub node_id: String,
  pub url: String,
  pub html_url: String,
  pub diff_url: String,
  pub patch_url: String,
  pub issue_url: String,
  pub commits_url: String,
  pub review_comments_url: String,
  pub review_comment_url: String,
  pub comments_url: String,
  pub statuses_url: String,
  pub number: u64,
  pub state: String,
  pub locked: bool,
  pub title: String,
  pub user: User,
  pub body: Option<String>,
  pub labels: Vec<Label>,
  pub milestone: Option<serde_json::Value>,
  pub active_lock_reason: Option<String>,
  pub created_at: String,
  pub updated_at: String,
  pub closed_at: Option<String>,
  pub merged_at: Option<String>,
  pub merge_commit_sha: Option<String>,
  pub assignee: Option<User>,
  pub assignees: Vec<User>,
  pub requested_reviewers: Vec<User>,
  pub requested_teams: Vec<serde_json::Value>,
  pub head: PullRequestRef,
  pub base: PullRequestRef,
  pub author_association: String,
  pub draft: bool,
  pub merged: bool,
  pub mergeable: Option<bool>,
  pub rebaseable: Option<bool>,
  pub mergeable_state: String,
  pub merged_by: Option<User>,
  pub comments: u64,
  pub review_comments: u64,
  pub maintainer_can_modify: bool,
  pub commits: u64,
  pub additions: u64,
  pub deletions: u64,
  pub changed_files: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PullRequestRef {
  pub label: String,
  pub ref_field: String,
  pub sha: String,
  pub user: User,
  pub repo: Option<Repository>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Release {
  pub id: u64,
  pub node_id: String,
  pub url: String,
  pub html_url: String,
  pub assets_url: String,
  pub upload_url: String,
  pub tarball_url: Option<String>,
  pub zipball_url: Option<String>,
  pub tag_name: String,
  pub target_commitish: String,
  pub name: Option<String>,
  pub body: Option<String>,
  pub draft: bool,
  pub prerelease: bool,
  pub created_at: String,
  pub published_at: Option<String>,
  pub author: User,
  pub assets: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Comment {
  pub id: u64,
  pub node_id: String,
  pub url: String,
  pub html_url: String,
  pub body: String,
  pub user: User,
  pub created_at: String,
  pub updated_at: String,
  pub issue_url: Option<String>,
  pub author_association: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "event", content = "payload")]
#[serde(rename_all = "snake_case")]
pub enum WebhookEvent {
  Push(PushEvent),
  PullRequest(PullRequestEvent),
  Issues(IssuesEvent),
  IssueComment(IssueCommentEvent),
  Create(CreateEvent),
  Delete(DeleteEvent),
  Fork(ForkEvent),
  Release(ReleaseEvent),
  #[serde(other)]
  Unknown,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PushEvent {
  #[serde(rename = "ref")]
  pub ref_field: String,
  pub before: String,
  pub after: String,
  pub repository: Repository,
  pub pusher: Pusher,
  pub organization: Option<Organization>,
  pub sender: User,
  pub created: bool,
  pub deleted: bool,
  pub forced: bool,
  pub base_ref: Option<String>,
  pub compare: String,
  pub commits: Vec<Commit>,
  pub head_commit: Option<Commit>,
  pub installation: Option<Installation>,
  pub enterprise: Option<Enterprise>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Pusher {
  pub name: String,
  pub email: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PullRequestEvent {
  pub action: String,
  pub number: u64,
  pub pull_request: PullRequest,
  pub repository: Repository,
  pub organization: Option<Organization>,
  pub installation: Option<Installation>,
  pub sender: User,
  pub enterprise: Option<Enterprise>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IssuesEvent {
  pub action: String,
  pub issue: Issue,
  pub repository: Repository,
  pub sender: User,
  pub organization: Option<Organization>,
  pub installation: Option<Installation>,
  pub enterprise: Option<Enterprise>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IssueCommentEvent {
  pub action: String,
  pub issue: Issue,
  pub comment: Comment,
  pub repository: Repository,
  pub sender: User,
  pub organization: Option<Organization>,
  pub installation: Option<Installation>,
  pub enterprise: Option<Enterprise>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateEvent {
  #[serde(rename = "ref")]
  pub ref_field: String,
  pub ref_type: String,
  pub master_branch: String,
  pub description: Option<String>,
  pub pusher_type: String,
  pub repository: Repository,
  pub sender: User,
  pub organization: Option<Organization>,
  pub installation: Option<Installation>,
  pub enterprise: Option<Enterprise>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DeleteEvent {
  #[serde(rename = "ref")]
  pub ref_field: String,
  pub ref_type: String,
  pub pusher_type: String,
  pub repository: Repository,
  pub sender: User,
  pub organization: Option<Organization>,
  pub installation: Option<Installation>,
  pub enterprise: Option<Enterprise>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ForkEvent {
  pub forkee: Repository,
  pub repository: Repository,
  pub sender: User,
  pub organization: Option<Organization>,
  pub installation: Option<Installation>,
  pub enterprise: Option<Enterprise>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ReleaseEvent {
  pub action: String,
  pub release: Release,
  pub repository: Repository,
  pub sender: User,
  pub organization: Option<Organization>,
  pub installation: Option<Installation>,
  pub enterprise: Option<Enterprise>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum GitHubWebhookPayload {
  Push(PushEvent),
  PullRequest(PullRequestEvent),
  Issues(IssuesEvent),
  IssueComment(IssueCommentEvent),
  Create(CreateEvent),
  Delete(DeleteEvent),
  Fork(ForkEvent),
  Release(ReleaseEvent),
  Generic(GenericPayload),
}

#[derive(Debug, Clone, Deserialize)]
pub struct GenericPayload {
  pub repository: Option<Repository>,
  pub sender: Option<User>,
  pub organization: Option<Organization>,
  pub installation: Option<Installation>,
  pub enterprise: Option<Enterprise>,
  #[serde(flatten)]
  pub other: HashMap<String, serde_json::Value>,
}

impl GitHubWebhookPayload {
  pub fn validate_required_fields(&self) -> bool {
    match self {
      GitHubWebhookPayload::Push(e) => !e.ref_field.is_empty(),
      GitHubWebhookPayload::PullRequest(e) => e.number > 0,
      GitHubWebhookPayload::Issues(e) => !e.action.is_empty(),
      GitHubWebhookPayload::IssueComment(e) => !e.action.is_empty(),
      GitHubWebhookPayload::Create(e) => !e.ref_field.is_empty() && !e.ref_type.is_empty(),
      GitHubWebhookPayload::Delete(e) => !e.ref_field.is_empty() && !e.ref_type.is_empty(),
      GitHubWebhookPayload::Fork(_) => true,
      GitHubWebhookPayload::Release(e) => !e.action.is_empty(),
      GitHubWebhookPayload::Generic(e) => {
        e.repository.is_some() || e.sender.is_some()
      }
    }
  }
}
