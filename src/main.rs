use blogapi::schema;
use clap::Clap;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use warp::Filter;

use juniper::EmptyMutation;

#[derive(Clap)]
#[clap(version = "0.1.0", author = "Denny Britz <dennybritz@gmail.com>")]
struct Opts {
    #[clap(subcommand)]
    cmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    #[clap(name = "serve", version = "0.1")]
    Serve(ServeOpts),
}

/// Insert records into the database
#[derive(Clap, Debug)]
struct ServeOpts {
    /// Read JSON records from this path
    #[clap(short = "p", long = "path")]
    path: String,

    /// Enable CORS requests from all origins (useful for local development)
    #[clap(long = "cors")]
    cors: bool,
}

fn make_schema() -> schema::Schema {
    schema::Schema::new(schema::Query, EmptyMutation::<schema::SharedContext>::new())
}

fn serve(opts: &ServeOpts) {
    log::info!("watching {}", opts.path);

    let path = opts.path.clone();
    let shared_ctx = schema::SharedContext::new();
    shared_ctx.update(&path);

    let watched_ctx = shared_ctx.clone();
    let mut watcher: RecommendedWatcher = Watcher::new_immediate(move |res| match res {
        Ok(_event) => {
            log::info!("file changed detected, rebuilding context");
            watched_ctx.update(&path);
        }
        Err(e) => log::info!("watch error: {:?}", e),
    })
    .expect("failed to create watcher");
    watcher.watch(&opts.path, RecursiveMode::Recursive).unwrap();

    let state = warp::any().map(move || shared_ctx.clone());
    let warp_log = warp::log("warp_server");
    let graphql_filter = juniper_warp::make_graphql_filter(make_schema(), state.boxed());

    let mut cors = warp::cors();
    if opts.cors {
        cors = cors
            .allow_any_origin()
            .allow_methods(vec!["OPTIONS", "GET", "POST", "DELETE"])
            .allow_headers(vec!["Content-Type"]);
    }
    let graphql_filter = graphql_filter.with(&cors);

    log::info!("Listening on 0.0.0.0:8080");
    warp::serve(
        warp::get2()
            .and(warp::path("graphiql"))
            .and(juniper_warp::graphiql_filter("/graphql"))
            .or(warp::path("graphql").and(graphql_filter))
            .with(warp_log),
    )
    .run(([0, 0, 0, 0], 8080));
}

fn main() {
    env_logger::init();

    let opts: Opts = Opts::parse();
    match opts.cmd {
        SubCommand::Serve(opts) => serve(&opts),
    }
}
