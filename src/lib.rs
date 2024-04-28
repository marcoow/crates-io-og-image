use worker::*;

#[event(fetch)]
async fn main(req: Request, env: Env, ctx: Context) -> Result<Response> {
    let svg_data = include_bytes!("../img.svg");
    let mut opt = usvg::Options::default();
    let mut fontdb = fontdb::Database::new();
    let svg = usvg::Tree::from_data(svg_data, &opt, &fontdb).unwrap();

    let pixmap_size = svg.size().to_int_size();
    let mut pixmap = tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height()).unwrap();
    resvg::render(&svg, tiny_skia::Transform::default(), &mut pixmap.as_mut());

    let mut headers = Headers::new();
    headers.set("content-type", "image/png")?;
    Ok(Response::from_bytes(pixmap.data().to_vec())?.with_headers(headers))
}
