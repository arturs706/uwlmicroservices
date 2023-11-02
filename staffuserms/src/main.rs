use leptos::*;
use tonic_web_wasm_client::Client;
use tonic::{metadata::MetadataValue, transport::Channel, Request};
use users::user_auth_client::UserAuthClient;
use users::UserAuthRequest;
pub mod users {
    tonic::include_proto!("staffusers");
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    view! {
        <button
            // define an event listener with on:
            on:click=move |_| {
                // on stable, this is set_count.set(3);
                set_count.set(3);
            }
        >
            // text nodes are wrapped in quotation marks
            "Click me: "
            // blocks can include Rust code
            {move || count.get()}
        </button>
    }
}

fn main() {
    let token: MetadataValue<_> = "1".parse().unwrap();
    let mut client = UserAuthClient::with_interceptor(
        Client::new(String::from("http://localhost:3000")),
        move |mut req: Request<()>| {
            req.metadata_mut().insert("authorization", token.clone());
            Ok(req)
        },
    );
    let request = tonic::Request::new(
        UserAuthRequest {
            name: "John Doe".into(),
            username: "Director".into(),
            mob_phone: "1234567890".into(),
            passwd: "123456".into(),
        }
    );
    
    leptos::mount_to_body(|| view! { <App/> })
}