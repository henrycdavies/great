use git2::{Index, IndexConflicts, IndexEntry, Repository};

#[derive(Debug)]
pub enum ConflictHandleErrorKind {
    GitError,
    IoError,
}

pub struct ConflictHandleError {
    kind: ConflictHandleErrorKind,
    message: String,
}

impl ConflictHandleError {
    pub fn new(kind: ConflictHandleErrorKind, message: String) -> Self {
        Self { kind, message }
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

impl From<git2::Error> for ConflictHandleError {
    fn from(err: git2::Error) -> Self {
        Self {
            kind: ConflictHandleErrorKind::GitError,
            message: format!("Git error: {}", err.message()),
        }
    }
}

pub type ConflictHandleResult<T> = Result<T, ConflictHandleError>;

///
/// ConflictHandler is a struct that handles conflicts in the index
/// and writes conflict markers to the working directory
/// 
pub struct ConflictHandler<'a> {
    repo: &'a Repository,
    index: Index,
}

impl<'a> ConflictHandler<'a> {
    ///
    /// Creates a new `ConflictHandler`
    /// 
    /// # Arguments
    /// 
    /// * `repo` - A reference to the `Repository`
    /// * `index` - The `Index` with conflicts
    /// 
    /// # Returns
    /// 
    /// A new `ConflictHandler`
    pub fn new(repo: &'a Repository, index: Index) -> Self {
        Self { repo, index }
    }

    ///
    /// Writes conflict markers for all conflicts in the index to the working directory
    /// 
    /// # Returns
    /// 
    /// A `Result` containing `()` if the conflict markers were written successfully
    /// 
    /// # Errors
    /// 
    /// An error of type `ConflictHandleError` if the conflict markers could not be written
    /// 
    pub fn write_all_markers(&self) -> ConflictHandleResult<()> {
        let mut conflict_paths: Vec<String> = Vec::new();
        for conflict in self.find_conflicts()?.into_iter().filter_map(Result::ok) {
            if let (Some(our), Some(their)) = (conflict.our, conflict.their) {
                let conflict_path = self.write_conflict_markers(our, their)?;
                conflict_paths.push(conflict_path);
                self.log_conflict_paths(&conflict_paths);
                return Ok(())
            }
            return Err(ConflictHandleError {
                kind: ConflictHandleErrorKind::GitError,
                message: "Conflict has missing sides".to_string(),
            })
        }
        Ok(())
    }

    ///
    /// Writes conflict markers for a specific conflict to the working directory
    /// 
    /// # Arguments
    /// 
    /// * `our` - The `IndexEntry` for the "our" side of the conflict
    /// * `their` - The `IndexEntry` for the "their" side of the conflict
    /// 
    /// # Returns
    /// 
    /// A `Result` containing `()` if the conflict markers were written successfully
    /// 
    fn write_conflict_markers(&self, our: IndexEntry, their: IndexEntry) -> ConflictHandleResult<String> {
        let our_blob = self.repo.find_blob(our.id)?;
        let their_blob = self.repo.find_blob(their.id)?;

        // Example of writing conflict markers
        let our_content = std::str::from_utf8(our_blob.content()).map_err(|_| {
            ConflictHandleError::new(
                ConflictHandleErrorKind::GitError,
                "Failed to read our content".to_string(),
            )
        })?;
        let their_content = std::str::from_utf8(their_blob.content()).map_err(|_| {
            ConflictHandleError::new(
                ConflictHandleErrorKind::GitError,
                "Failed to read their content".to_string(),
            )
        })?;

        let conflict_markers = format!(
            "<<<<<<< OURS\n{}\n=======\n{}\n>>>>>>> THEIRS\n",
            our_content, their_content
        );

        // Write the conflict markers to the file
        let path = std::str::from_utf8(&our.path).map_err(|_| {
            ConflictHandleError::new(
                ConflictHandleErrorKind::IoError,
                "Failed to read path".to_string(),
            )
        })?;
        std::fs::write(path, conflict_markers).map_err(|_| {
            ConflictHandleError::new(
                ConflictHandleErrorKind::IoError,
                "Failed to write conflict markers".to_string(),
            )
        })?;
        return Ok(path.to_string())
    }

    ///
    /// Finds all conflicts in the index
    /// 
    /// # Returns
    /// 
    /// A `Result` containing an `IndexConflicts` iterator
    /// 
    fn find_conflicts(&self) -> Result<IndexConflicts<'_>, git2::Error> {
        self.index.conflicts()
    }

    fn log_conflict_paths(&self, conflict_paths: &[String]) {
        let message_header = format!("🚨 {} conflict(s) found in the following files:", conflict_paths.len());
        let message_body = conflict_paths.join("\n");
        let message_footer = String::from("Resolve these conflicts and run \"great continue\" to continue the recursive sync.");
        let message = [message_header, message_body, message_footer].join("\n\n");
        println!("{}", message);
    }
}
