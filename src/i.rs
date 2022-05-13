use colour::dark_grey_ln;
use colour::red_ln;
#[tokio::main]

pub async fn i() {
    if let Some(ip) = public_ip::addr().await {
        dark_grey_ln!("{:?}", ip);
    } else {
        red_ln!("error: unknown");
    }
}
