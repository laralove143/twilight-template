// #![warn(clippy::cargo, clippy::nursery, clippy::pedantic,
// clippy::restriction)] #![allow(
//     clippy::blanket_clippy_restriction_lints,
// )]

// mod interaction;

use std::{env, sync::Arc};

use anyhow::Result;
use futures_util::StreamExt;
use twilight_gateway::{Cluster, EventTypeFlags};
use twilight_http::Client;
use twilight_model::gateway::{event::Event, Intents};

type Context = Arc<ContextInner>;

struct ContextInner {
    http: Client,
    // cache: InMemoryCache,
    // cluster: Cluster
    // application_id: Id<ApplicationMarker>,
    // user_id: Id<UserMarker>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let intents = Intents::empty();
    let event_types = EventTypeFlags::empty();
    // let resource_types = ResourceType::empty();

    let token = env::var("TOKEN")?;

    let (cluster, mut events) = Cluster::builder(token.clone(), intents)
        .event_types(event_types)
        .build()
        .await?;
    let cluster_spawn = Arc::new(cluster);
    tokio::spawn(async move { cluster_spawn.up().await });

    let http = Client::new(token);

    // let application_id = http
    //     .current_user_application()
    //     .exec()
    //     .await?
    //     .model()
    //     .await?
    //     .id;

    // let user_id = http.current_user().exec().await?.model().await?.id;

    // http.interaction(application_id)
    //     .set_global_commands(&[])
    //     .exec()
    //     .await?;

    // let cache = InMemoryCache::builder()
    //     .resource_types(resource_types)
    //     .build();

    let ctx = Arc::new(ContextInner { http });

    while let Some((_, event)) = events.next().await {
        // ctx.cache.update(&event);
        tokio::spawn(handle_event(Arc::clone(&ctx), event));
    }

    Ok(())
}

async fn handle_event(ctx: Context, event: Event) {
    match event {
        _ => (),
    }
}
