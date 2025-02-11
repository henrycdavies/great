mod checkout;
mod trunk;
mod result;

pub use trunk::find_trunk_branch;
pub use checkout::checkout_branch;
pub use result::{BranchError, BranchErrorKind, BranchResult};
