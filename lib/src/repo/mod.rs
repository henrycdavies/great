mod commit;
mod open;
mod result;
mod stage;

pub use commit::new_commit;
pub use open::open_repo;
pub use result::{RepoError, RepoErrorKind, RepoResult};
pub use stage::add_all;
