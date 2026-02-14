//! Git operations for sync

use git2::{
    Cred, FetchOptions, MergeOptions, PushOptions,
    RemoteCallbacks, Repository, Signature, StatusOptions,
};
use std::path::Path;
use thiserror::Error;

use super::status::SyncStatus;

/// Git operation errors
#[derive(Debug, Error)]
pub enum GitError {
    #[error("Repository not initialized")]
    NotInitialized,
    #[error("No remote configured")]
    NoRemote,
    #[error("Remote URL required")]
    RemoteRequired,
    #[error("Git error: {0}")]
    Git(#[from] git2::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

/// Git repository wrapper for Chronicle sync operations
pub struct GitRepo {
    repo: Repository,
}

impl GitRepo {
    /// Open existing repository at path
    pub fn open(path: &Path) -> Result<Self, GitError> {
        let repo = Repository::open(path)?;
        Ok(Self { repo })
    }

    /// Initialize new repository at path
    pub fn init(path: &Path) -> Result<Self, GitError> {
        let repo = Repository::init(path)?;
        
        // Create initial commit so we have a HEAD
        {
            let sig = Signature::now("Chronicle", "chronicle@local")?;
            let mut index = repo.index()?;
            let tree_id = index.write_tree()?;
            let tree = repo.find_tree(tree_id)?;
            repo.commit(Some("HEAD"), &sig, &sig, "Initial commit", &tree, &[])?;
        }
        
        Ok(Self { repo })
    }

    /// Check if path is a git repository
    pub fn is_repo(path: &Path) -> bool {
        Repository::open(path).is_ok()
    }

    /// Get current branch name
    pub fn current_branch(&self) -> Result<String, GitError> {
        let head = self.repo.head()?;
        let branch = head.shorthand().unwrap_or("main");
        Ok(branch.to_string())
    }

    /// Get configured remote URL
    pub fn remote_url(&self) -> Option<String> {
        self.repo
            .find_remote("origin")
            .ok()
            .and_then(|r| r.url().map(String::from))
    }

    /// Set remote URL
    pub fn set_remote(&self, url: &str) -> Result<(), GitError> {
        // Remove existing origin if present
        if self.repo.find_remote("origin").is_ok() {
            self.repo.remote_delete("origin")?;
        }
        self.repo.remote("origin", url)?;
        Ok(())
    }

    /// Stage all changes and commit
    pub fn commit(&self, message: &str) -> Result<String, GitError> {
        let mut index = self.repo.index()?;
        index.add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, None)?;
        index.write()?;

        let tree_id = index.write_tree()?;
        let tree = self.repo.find_tree(tree_id)?;
        let sig = Signature::now("Chronicle", "chronicle@local")?;

        let parent = self.repo.head().ok().and_then(|h| h.peel_to_commit().ok());
        let parents: Vec<_> = parent.iter().collect();

        let oid = self.repo.commit(Some("HEAD"), &sig, &sig, message, &tree, &parents)?;
        Ok(oid.to_string())
    }

    /// Check if working directory has changes
    pub fn is_dirty(&self) -> Result<bool, GitError> {
        let mut opts = StatusOptions::new();
        opts.include_untracked(true);
        let statuses = self.repo.statuses(Some(&mut opts))?;
        Ok(!statuses.is_empty())
    }

    /// Get list of changed files
    pub fn changed_files(&self) -> Result<Vec<String>, GitError> {
        let mut opts = StatusOptions::new();
        opts.include_untracked(true);
        let statuses = self.repo.statuses(Some(&mut opts))?;
        
        let files: Vec<String> = statuses
            .iter()
            .filter_map(|s| s.path().map(String::from))
            .collect();
        
        Ok(files)
    }

    /// Get ahead/behind counts compared to remote
    pub fn ahead_behind(&self) -> Result<(usize, usize), GitError> {
        let head = self.repo.head()?;
        let local_oid = head.target().ok_or(GitError::NotInitialized)?;

        // Try to find remote tracking branch
        let remote_ref = format!("refs/remotes/origin/{}", head.shorthand().unwrap_or("main"));
        match self.repo.find_reference(&remote_ref) {
            Ok(remote) => {
                let remote_oid = remote.target().ok_or(GitError::NotInitialized)?;
                let (ahead, behind) = self.repo.graph_ahead_behind(local_oid, remote_oid)?;
                Ok((ahead, behind))
            }
            Err(_) => {
                // No remote tracking branch yet - we're ahead by all commits
                let mut count = 0;
                let mut revwalk = self.repo.revwalk()?;
                revwalk.push_head()?;
                for _ in revwalk {
                    count += 1;
                }
                Ok((count, 0))
            }
        }
    }

    /// Fetch from remote
    pub fn fetch(&self) -> Result<(), GitError> {
        let mut remote = self.repo.find_remote("origin").map_err(|_| GitError::NoRemote)?;
        let mut callbacks = RemoteCallbacks::new();
        callbacks.credentials(|_url, username, _allowed| {
            Cred::ssh_key_from_agent(username.unwrap_or("git"))
        });

        let mut fetch_opts = FetchOptions::new();
        fetch_opts.remote_callbacks(callbacks);

        let branch = self.current_branch()?;
        remote.fetch(&[&branch], Some(&mut fetch_opts), None)?;
        Ok(())
    }

    /// Push to remote
    pub fn push(&self) -> Result<(), GitError> {
        let mut remote = self.repo.find_remote("origin").map_err(|_| GitError::NoRemote)?;
        let mut callbacks = RemoteCallbacks::new();
        callbacks.credentials(|_url, username, _allowed| {
            Cred::ssh_key_from_agent(username.unwrap_or("git"))
        });

        let mut push_opts = PushOptions::new();
        push_opts.remote_callbacks(callbacks);

        let branch = self.current_branch()?;
        let refspec = format!("refs/heads/{}:refs/heads/{}", branch, branch);
        remote.push(&[&refspec], Some(&mut push_opts))?;
        Ok(())
    }

    /// Pull (fetch + merge) from remote
    pub fn pull(&self) -> Result<Vec<String>, GitError> {
        self.fetch()?;

        let branch = self.current_branch()?;
        let remote_ref = format!("refs/remotes/origin/{}", branch);
        
        let fetch_head = self.repo.find_reference(&remote_ref)?;
        let fetch_commit = self.repo.reference_to_annotated_commit(&fetch_head)?;

        let (analysis, _) = self.repo.merge_analysis(&[&fetch_commit])?;

        if analysis.is_up_to_date() {
            return Ok(Vec::new());
        }

        if analysis.is_fast_forward() {
            // Fast-forward merge
            let mut reference = self.repo.find_reference(&format!("refs/heads/{}", branch))?;
            reference.set_target(fetch_commit.id(), "Fast-forward")?;
            self.repo.checkout_head(Some(git2::build::CheckoutBuilder::default().force()))?;
            return Ok(Vec::new());
        }

        // Need to do a real merge
        self.repo.merge(&[&fetch_commit], Some(MergeOptions::new().fail_on_conflict(false)), None)?;

        // Check for conflicts
        let mut index = self.repo.index()?;
        if index.has_conflicts() {
            let conflicts: Vec<String> = index
                .conflicts()?
                .filter_map(|c| c.ok())
                .filter_map(|c| c.our.or(c.their))
                .filter_map(|e| String::from_utf8(e.path.clone()).ok())
                .collect();
            return Ok(conflicts);
        }

        // Commit the merge
        let sig = Signature::now("Chronicle", "chronicle@local")?;
        let tree_id = index.write_tree()?;
        let tree = self.repo.find_tree(tree_id)?;
        let head_commit = self.repo.head()?.peel_to_commit()?;
        let fetch_commit_obj = self.repo.find_commit(fetch_commit.id())?;

        self.repo.commit(
            Some("HEAD"),
            &sig,
            &sig,
            "Merge remote changes",
            &tree,
            &[&head_commit, &fetch_commit_obj],
        )?;

        self.repo.cleanup_state()?;

        Ok(Vec::new())
    }

    /// Get current sync status
    pub fn status(&self) -> Result<SyncStatus, GitError> {
        let (ahead, behind) = self.ahead_behind()?;
        
        Ok(SyncStatus {
            initialized: true,
            remote_url: self.remote_url(),
            branch: self.current_branch()?,
            ahead,
            behind,
            conflicts: self.get_conflicts()?,
            last_sync: None, // Tracked externally
            dirty: self.is_dirty()?,
        })
    }

    /// Get list of conflicted files
    pub fn get_conflicts(&self) -> Result<Vec<String>, GitError> {
        let index = self.repo.index()?;
        if !index.has_conflicts() {
            return Ok(Vec::new());
        }

        let conflicts: Vec<String> = index
            .conflicts()?
            .filter_map(|c| c.ok())
            .filter_map(|c| c.our.or(c.their))
            .filter_map(|e| String::from_utf8(e.path.clone()).ok())
            .collect();

        Ok(conflicts)
    }

    /// Mark conflict as resolved for a file
    pub fn resolve_conflict(&self, path: &str) -> Result<(), GitError> {
        let mut index = self.repo.index()?;
        index.add_path(Path::new(path))?;
        index.write()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs;

    #[test]
    fn test_init_repo() {
        let temp = TempDir::new().unwrap();
        let repo = GitRepo::init(temp.path());
        assert!(repo.is_ok());
        assert!(GitRepo::is_repo(temp.path()));
    }

    #[test]
    fn test_open_repo() {
        let temp = TempDir::new().unwrap();
        GitRepo::init(temp.path()).unwrap();
        let repo = GitRepo::open(temp.path());
        assert!(repo.is_ok());
    }

    #[test]
    fn test_current_branch() {
        let temp = TempDir::new().unwrap();
        let repo = GitRepo::init(temp.path()).unwrap();
        // After init, branch is either main or master
        let branch = repo.current_branch().unwrap();
        assert!(!branch.is_empty());
    }

    #[test]
    fn test_commit() {
        let temp = TempDir::new().unwrap();
        let repo = GitRepo::init(temp.path()).unwrap();
        
        // Create a file
        fs::write(temp.path().join("test.md"), "# Test").unwrap();
        
        let result = repo.commit("Add test note");
        assert!(result.is_ok());
    }

    #[test]
    fn test_is_dirty() {
        let temp = TempDir::new().unwrap();
        let repo = GitRepo::init(temp.path()).unwrap();
        
        assert!(!repo.is_dirty().unwrap());
        
        // Create a file
        fs::write(temp.path().join("test.md"), "# Test").unwrap();
        
        assert!(repo.is_dirty().unwrap());
    }

    #[test]
    fn test_changed_files() {
        let temp = TempDir::new().unwrap();
        let repo = GitRepo::init(temp.path()).unwrap();
        
        fs::write(temp.path().join("test.md"), "# Test").unwrap();
        
        let files = repo.changed_files().unwrap();
        assert_eq!(files.len(), 1);
        assert!(files.contains(&"test.md".to_string()));
    }

    #[test]
    fn test_set_remote() {
        let temp = TempDir::new().unwrap();
        let repo = GitRepo::init(temp.path()).unwrap();
        
        repo.set_remote("https://github.com/user/repo.git").unwrap();
        
        assert_eq!(
            repo.remote_url(),
            Some("https://github.com/user/repo.git".to_string())
        );
    }

    #[test]
    fn test_status() {
        let temp = TempDir::new().unwrap();
        let repo = GitRepo::init(temp.path()).unwrap();
        
        let status = repo.status().unwrap();
        assert!(status.initialized);
        assert!(!status.dirty);
        assert!(status.conflicts.is_empty());
    }
}
