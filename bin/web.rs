use warp::Filter;

fn main() {
    println!("this is the webserver, you can close it if you want because it only opened an html page which is already cached in your browser");
    web();
}

#[tokio::main]
async fn web() {
    if webbrowser::open("http://127.0.0.1:3030").is_err() {
        eprintln!("Failed to open the c2 browser");
    }

    // Start the c2 server.
    start_server().await;
}

async fn start_server() {
    // Route for the root path serving start.html
    let html = warp::path::end()
        .map(|| warp::reply::html(include_str!("../c2/start.html")));

    // Route for /quiz.html serving quiz.html
    let html2 = warp::path("quiz.html")
        .map(|| warp::reply::html(include_str!("../c2/quiz.html")));

    // Combine the routes and start the server
    let routes = html.or(html2);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}