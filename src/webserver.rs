use warp::Filter;
use std::sync::mpsc::Sender;

pub async fn start_server(tx: Sender<i32>, tx_end: Sender<()>) {
    // Route for the root path serving start.html
    let start = warp::path::end()
        .map(|| warp::reply::html(include_str!("../web/start.html")));

    // Route for /quiz.html serving quiz.html
    let quiz = warp::path("quiz.html")
        .map(|| warp::reply::html(include_str!("../web/quiz.html")));

    let end = warp::path("end")
        .map(move || {
            tx_end.send(()).unwrap();  // Send a signal to indicate the /end route was hit.
            warp::reply::html("Ending...")
        });

    // Combine the routes and start the server
    let routes = start.or(quiz).or(end);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}