use actix_web::http::header::CONTENT_TYPE;
use actix_web::web::Bytes;
use actix_web::{get, HttpRequest, HttpResponse};
use futures::stream::unfold;
use tokio::time::sleep;

#[get("/healthz")]
async fn index(req: HttpRequest) -> HttpResponse {
    let mine = req
        .headers()
        .get(CONTENT_TYPE)
        .and_then(|x| x.to_str().ok());

    if mine.map(|x| x == "text/event-stream").unwrap_or_default() {
        let no_stream = unfold((), forever_await);
        return HttpResponse::Ok().streaming(no_stream);
    }

    HttpResponse::NoContent().finish()
}

pub async fn forever_await(_: ()) -> Option<(std::result::Result<Bytes, actix_web::Error>, ())> {
    sleep(std::time::Duration::from_secs(1)).await;
    let bytes = Bytes::copy_from_slice(&[]);
    Some((Ok::<_, actix_web::Error>(bytes), ()))
}
