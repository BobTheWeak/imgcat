
// NOTE: We just moved Actix Handlers into routes. But these return HttpRequest, sets the
// service path via macros, which mangles things & requires a LOT more error handling.
//mod old_vote_category;
//pub use old_vote_category::vote_category as old_vote_category_svc;
mod old_vote_mature;
pub use old_vote_mature::vote_mature;
mod old_vote_tag;
pub use old_vote_tag::vote_tag;

mod old_vote_review;
pub use old_vote_review::vote_review;
mod old_anon_review;
pub use old_anon_review::anon_review;

// NOTE: The new, standards-compliant versions return ICResult,
// and the service path is set in main(), not each individual file
mod vote_category;
pub use vote_category::vote_category as new_vote_category;
//mod vote_mature;
//pub use vote_mature::vote_mature;
//mod vote_tag;
//pub use vote_tag::vote_tag;

//mod vote_review;
//pub use vote_review::vote_review;
//mod anon_review;
//pub use anon_review::anon_review;

mod health_check;
pub use health_check::{livez_status, readyz_status};