use tokio::{
    select,
    signal::unix::{signal, SignalKind},
    time::{Duration, interval}
};

mod conf;
mod filters;
mod github;
mod notify;

#[tokio::main]
async fn main() {
    let mut sigint = signal(SignalKind::interrupt()).unwrap();
    let mut interval = interval(Duration::from_secs(120));
    loop {
        select! {
            _ =  sigint.recv() => {
                println!("sigint recv'd");
                break;
            }
            _ = interval.tick() => {
                println!("poll start");
                let notifications = github::list_notifications()
                    .await
                    .unwrap()
                    .into_iter()
                    .filter(|n| !filters::is_dep_bump(&n) && filters::allowed_repo(&n))
                    .collect::<Vec<_>>();

                for n in notifications {
                    notify::display_notification(&n).await;
                    github::mark_read(n.id).await.unwrap();
                }
                println!("poll finished");
            }
        }
    }
}
