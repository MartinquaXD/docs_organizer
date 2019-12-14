#![feature(async_closure)]

use serde_json::json;
use elastic::prelude::*;

mod document;

use document::Document;
use leptess;
use hyper::{Body, Request, Response, Server, Error};
use hyper::service::{service_fn, make_service_fn};
use opencv::prelude::*;

fn test_search() {
    // A reqwest HTTP client and default parameters.
    // The builder includes the base node url (http://localhost:9200).
    let client = SyncClient::builder().build().unwrap();

    let query = "*Mietve*";

    // A search request with a freeform body.
    let res = client.search::<Document>()
        .index("docs_organizer")
        .body(json!({
                    "query": {
                        "query_string": {
                            "query": query
                        }
                    }
                }))
        .send().unwrap();

    // Iterate through the hits in the response.
    for hit in res.hits() {
        println!("{:?}", hit);
    }
}

fn index_document(doc: Document) {
    let client = SyncClient::builder().build().unwrap();
    client.document().index(doc).send().unwrap();
}

fn scan_image(path: &str) -> String {
    let mut lt = leptess::LepTess::new(None, "deu").unwrap();
    lt.set_image(path);
    lt.set_source_resolution(300);
    lt.get_utf8_text().unwrap()
}

async fn read_body(mut req: Request<Body>) -> Result<Vec<u8>, Error> {
    use futures_util::stream::TryStreamExt;
    Ok(req.into_body().map_ok(|b| b.to_vec()).try_concat().await?)
}

fn get_next_file_path() -> Result<String, std::io::Error> {
    let now = chrono::offset::Local::now();
    let file_name = format!("{}_{}:{}:{}", now.day(), now.hour(), now.minute(), now.second());
    let folder_path = format!("/home/martinbeckmann/docs_organizer/{}/{}", now.year(), now.month());
    std::fs::create_dir_all(&folder_path)?;
    Ok(format!("{}/{}", folder_path, file_name))
}

fn rotate(src: &opencv::prelude::Mat, angle: f64) -> opencv::prelude::Mat {
    let mut dst = opencv::prelude::Mat::default().unwrap();

    let c_x = (src.cols().unwrap() as f32) / 2.0;
    let c_y = (src.rows().unwrap() as f32) / 2.0;

    let pt = opencv::core::Point2f::new(c_x, c_y);


    let mut r = opencv::imgproc::get_rotation_matrix_2d(pt, angle, 1.0).unwrap();

    let w = src.cols().unwrap() as f64;
    let h = src.rows().unwrap() as f64;

    let (sin, cos) = angle.sin_cos();
    let (sin, cos) = (sin.abs(), cos.abs());

    let n_w = (h * sin) + (w * cos);
    let n_h = (h * cos) + (w * sin);

    *r.at_2d_mut::<f64>(0, 2).unwrap() += (n_w / 2.0) - c_x as f64;
    *r.at_2d_mut::<f64>(1, 2).unwrap() += (n_h / 2.0) - c_y as f64;

    let size = opencv::core::Size_::new(n_w as i32, n_h as i32);
    opencv::imgproc::warp_affine(src, &mut dst, &r, size, opencv::imgproc::INTER_LINEAR, opencv::core::BORDER_CONSTANT, opencv::core::Scalar_::new(1.0, 1.0, 1.0, 1.0));
    dst
}

fn store_img(data: &[u8]) -> String {
    let mut path = get_next_file_path().unwrap();
    std::fs::write(&path, data).unwrap();
    let img = opencv::imgcodecs::imread(&path, opencv::imgcodecs::IMREAD_COLOR).unwrap();
    std::fs::remove_file(&path);

    let mut dst = opencv::prelude::Mat::default().unwrap();

    opencv::photo::fast_nl_means_denoising_colored(&img, &mut dst, 100.0, 3.0, 21, 7);

    let mut dst_2 = opencv::prelude::Mat::default().unwrap();
    opencv::photo::detail_enhance(&dst, &mut dst_2, 100.0, 0.15);

    let mut dst_3 = opencv::prelude::Mat::default().unwrap();
    let mut any = opencv::prelude::Mat::default().unwrap();
    opencv::photo::decolor(&dst_2, &mut dst_3, &mut any);

    let mut dst_4 = opencv::prelude::Mat::default().unwrap();
    opencv::imgproc::adaptive_threshold(&dst_3, &mut dst_4, 255.0, opencv::imgproc::ADAPTIVE_THRESH_MEAN_C, opencv::imgproc::THRESH_BINARY, 41, 10.0);

    let res = rotate(&dst_4, 0.0);


    let mut params = opencv::types::VectorOfint::new();
    params.push(opencv::imgcodecs::IMWRITE_PNG_COMPRESSION);
    params.push(9);
    path.push_str(".png");
    let res = opencv::imgcodecs::imwrite(&path, &res, &params);

    path
}


async fn handle_image_upload(req: Request<Body>) -> Result<(), Error> {
    let body = read_body(req).await?;
    let (content, directory) = read_image(&body);
    println!("{}", content);
    index_document(Document {
        content,
        directory,
    });
    Ok(())
}

fn store_image(img: image::DynamicImage) -> Result<String, std::io::Error> {
    let path = get_next_file_path()?;
    img.save_with_format(&path, image::PNG)?;
    Ok(path)
}

fn read_image(data: &Vec<u8>) -> (String, String) {
    let path = store_img(&data);
    (scan_image(path.as_str()), path)
}

#[tokio::main]
async fn main() {
    let addr = ([127, 0, 0, 1], 9999).into();

    let service = make_service_fn(async move |_| {
        Ok::<_, Error>(service_fn(async move |req: Request<Body>| {
            if req.uri() == "/uploadImage" {
                handle_image_upload(req).await?;
            }

            Ok::<_, Error>(Response::new(Body::from("Hello World")))
        }))
    });


    let server = Server::bind(&addr)
        .serve(service);

    println!("Listening on http://{}", addr);

    server.await.unwrap();
}
