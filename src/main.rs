use pandoc_graphql::schema;
use clap::Clap;
use juniper::EmptyMutation;
use warp::Filter;

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
    /// Read document index from these files (can be globs)
    #[clap(short = "f", long = "file", multiple = true)]
    files: Vec<String>,

    /// Base directory to run pandoc from
    #[clap(short = "b", long = "base-dir", default_value=".")]
    base_dir: String,

    /// Enable CORS requests from all origins (useful for local development)
    #[clap(long = "cors")]
    cors: bool,
}

fn make_schema() -> schema::Schema {
    schema::Schema::new(schema::Query, EmptyMutation::<schema::Context>::new())
}

fn serve(opts: &ServeOpts) {
    log::info!("reading files from {}", opts.files.join(", "));

    let base_dir = std::fs::canonicalize(&opts.base_dir).unwrap();
    let base_dir = base_dir.to_string_lossy();
    log::info!("using base dir {}", &base_dir);

    let context = schema::Context::new(opts.files.clone(), String::from(base_dir));
    let state = warp::any().map(move || context.clone());
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
