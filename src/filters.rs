use lazy_static::lazy_static;
use regex::bytes::Regex;
use crate::github::Notification;

/******************************************************************************
Simple function to check if the notification matches the format of a dependency
bump.
******************************************************************************/
lazy_static! {
    static ref DEP_BUMP_REGEX: Regex = Regex::new("chore\\((deps?(-dev)?)\\)")
        .expect("regex should not fail to compile");
}

pub fn is_dep_bump(n: &Notification) -> bool {
    DEP_BUMP_REGEX.is_match(n.subject.title.as_bytes())
}

/******************************************************************************
Determine is this notification is from a repo we care about.
******************************************************************************/
pub fn allowed_repo(_n: &Notification) -> bool {
    true
}