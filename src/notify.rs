use std::collections::BTreeMap;
use std::sync::Once;
use notify_rust::{get_bundle_identifier_or_default, set_application, Notification};
use crate::github::Notification as GithubNotification;

static NOTIFICATION_INIT: Once = Once::new();

fn maybe_init_notifications() {
    NOTIFICATION_INIT.call_once(|| {
        let safari_id = get_bundle_identifier_or_default("Github Desktop");
        set_application(&safari_id).expect("setting application id for notifications");
    });
}

pub async fn display_notification(n: &GithubNotification) {
    maybe_init_notifications();
    let body = format!("{}: {}", n.subject.category, n.subject.title);
    Notification::new()
        .summary(&n.repository.full_name)
        .body(&body)
        .timeout(0)
        .show()
        .unwrap();
}