#![feature(async_closure)]

use serde_json::{json};
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
    Ok(req.into_body().map_ok(|b|b.to_vec()).try_concat().await?)
}

fn get_next_file_path() -> Result<String, std::io::Error> {
    let now = chrono::offset::Local::now();
    let file_name = format!("{}_{}:{}:{}", now.day(), now.hour(), now.minute(), now.second());
    let folder_path = format!("/home/martinbeckmann/docs_organizer/{}/{}", now.year(), now.month());
    std::fs::create_dir_all(&folder_path)?;
    Ok(format!("{}/{}", folder_path, file_name))
}

//fn rotate(src: &opencv::prelude::Mat, angle: f64)-> opencv::prelude::Mat {
//    println!("asdasd");
//    let mut dst = opencv::prelude::Mat::default().unwrap();
//    let pt = opencv::core::Point2f::new((src.cols().unwrap() as f32) / 2.0, (src.rows().unwrap() as f32) / 2.0);
//    let r = opencv::imgproc::get_rotation_matrix_2d(pt, angle, 1.0).unwrap();
//    let size = opencv::core::Size_::new(src.cols().unwrap(), src.rows().unwrap());
//    opencv::imgproc::warp_affine(src, &mut dst, &r, size, opencv::imgproc::INTER_LINEAR, opencv::core::BORDER_CONSTANT, opencv::core::Scalar_::new(1.0, 1.0, 1.0,1.0));
//    println!("asdasd");
//    return dst;
//}

async fn store_img_2(){
    let data = std::fs::read("/home/martinbeckmann/Pictures/test3").unwrap();
    let mut path = get_next_file_path().unwrap();
    std::fs::write(&path, data.as_slice()).unwrap();
    let img = opencv::imgcodecs::imread(&path, opencv::imgcodecs::IMREAD_UNCHANGED).unwrap();

    let mut dst = opencv::prelude::Mat::default().unwrap();
//    opencv::imgproc::sobel(&img, &mut dst, -1, 1, 1, 3, 1.0, 0.0, opencv::core::BORDER_DEFAULT);
//    opencv::imgproc::canny(&img, &mut dst, 40.0, 70.0, 3, false);
    opencv::photo::fast_nl_means_denoising_colored(&img, &mut dst, 100.0, 3.0, 21, 7);
    let mut dst_2 = opencv::prelude::Mat::default().unwrap();
    println!("{:#?}", opencv::photo::detail_enhance(&dst, &mut dst_2, 100.0, 0.15));

    let mut dst_3 = opencv::prelude::Mat::default().unwrap();
    let mut any = opencv::prelude::Mat::default().unwrap();
    println!("{:#?}", opencv::photo::decolor(&dst_2, &mut dst_3, &mut any));

    let mut dst_4 = opencv::prelude::Mat::default().unwrap();
    println!("{:#?}", opencv::imgproc::adaptive_threshold(&dst_3, &mut dst_4, 255.0, opencv::imgproc::ADAPTIVE_THRESH_MEAN_C, opencv::imgproc::THRESH_BINARY, 41, 10.0));

//    let res = rotate(&dst_4, -90.0);


    let mut params = opencv::types::VectorOfint::new();
    params.push(opencv::imgcodecs::IMWRITE_PNG_COMPRESSION);
    params.push(9);
    path.push_str(".png");
    let res = opencv::imgcodecs::imwrite(&path, &dst_4, &params);
    println!("{:#?}", res);
}


fn preprocess_img(data: &Vec<u8>) -> image::DynamicImage {
//    let res = opencv::imgcodecs::imdecode(data, opencv::imgcodecs::IMREAD_GRAYSCALE).expect("opencv error");
    image::load_from_memory(data.as_slice()).unwrap()
        .grayscale()
        .adjust_contrast(20.0)
}

async fn handle_image_upload(req: Request<Body>) -> Result<(), Error> {
    let body = read_body(req).await?;
    let (content, directory) = read_image(&body).unwrap();
    index_document(Document {
        content,
        directory
    });
    Ok(())
}

fn store_image(img: image::DynamicImage) -> Result<String, std::io::Error>{
    let path = get_next_file_path()?;
    img.save_with_format(&path, image::PNG)?;
    Ok(path)
}

fn read_image(data: &Vec<u8>) -> Result<(String, String), std::io::Error> {
    let img = preprocess_img(data);
    let path = store_image(img)?;
    Ok((scan_image(path.as_str()), path))
}

#[tokio::main]
async fn main() {
    store_img_2().await;

//    let addr = ([127, 0, 0, 1], 9999).into();
//
//    let service = make_service_fn(async move |_| {
//        Ok::<_, Error>(service_fn(async move |req: Request<Body>| {
//            if req.uri() == "/uploadImage" {
//                handle_image_upload(req).await?;
//            }
//
//            Ok::<_, Error>(Response::new(Body::from("Hello World")))
//        }))
//    });
//
//
//    let server = Server::bind(&addr)
//        .serve(service);
//
//    println!("Listening on http://{}", addr);
//
//    server.await.unwrap();
}
