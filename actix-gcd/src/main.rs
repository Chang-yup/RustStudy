use actix_web::{web, App, HttpResponse, HttpServer};

#[actix_web::main]
async fn main() {
    let server = HttpServer::new(|| {App::new()
        .route("/", web::get().to(get_index))
        .route("/gcd", web::post().to(post_gcd))
    });
    // server 는 HttpServer::new(...)로 생성한 구조체다.
    // ||{App::new().route("/",web::get().to(get_index)) } 는 클로저인데, new() 메서드로 전달되어서 서버 실행시 어떤 동작을 해야하는지 전달한다.
    // 여기서는 App::new()를 호출해서 내부가 비어있는 새 App를 만든 다음, 여기에 route 메서드를 호출해서 "/"(http://domain/)주소에 라우팅 경로와 핸들러를 추가로 등록한다.
    // 따라서 HttpServer스레드도 이 값을 사용하게 된다.

    println!("Serving on http://localhost:3000...");
    server
        .bind("127.0.0.1:3000")
        .expect("error binding server to address")
        .run()
        .await
        .expect("error running server");
}

// get_index 함수는 GET요청에 대한 응답을 표현하는 HttpResponse 값을 만든다.
async fn get_index() -> HttpResponse {
    HttpResponse::Ok() // HTTP STATUS 200(OK) 반환
        .content_type("text/html")
        .body(
            r#"
                <title>GCD Calculator</title>
                <form action="/gcd" method="post">
                <input type="text" name="n"/>
                <input type="text" name="m"/>
                <button type="submit">Compute GCD</button>
                </form>
            "#, // r#""# (#기호는 몇개든 상관없음) 를 사용하면 쓴 그대로 문자열이 표현횐다. (이스케이프 시퀀스 적용 안됨)
        )
}

//App에 POST요청을 처리하는 라우팅 경로를 추가하자.
use serde::Deserialize;

#[derive(Deserialize)] // 프로그램이 컴파일 될 때 serde 크레이트가 해당 타입을 살펴보고 HTML 폼이 POST 요청에 사용하는 형식으로 된 데이터를 해당 값으로 파싱하는 코드를 생성해준다.
                       // 이 어트리뷰트만 있으면 실제로 JSON, YAML, TOML 등 텍스트와 바이너리 형식으로 된 거의 모든 종류의 구조화된 데이터를 GcdParameters 값으로 파싱할 수 있다.
struct GcdParameters {
    n: u64,
    m: u64,
}

async fn post_gcd(form: web::Form<GcdParameters>) -> HttpResponse {
    if form.n == 0 || form.m == 0 {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("Computing the GCD with zero is boring.");
    }

    let response =
        format!("The greatest common divisor of the numbers {} and {} \
                 is <b>{}</b>\n",
                form.n, form.m, gcd(form.n, form.m));

    HttpResponse::Ok()
        .content_type("text/html")
        .body(response)
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}