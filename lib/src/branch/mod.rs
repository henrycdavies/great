mod checkout;
mod result;
mod trunk;

pub use checkout::checkout_branch;
pub use result::{BranchError, BranchErrorKind, BranchResult};
pub use trunk::find_trunk_branch;
