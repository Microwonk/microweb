use leptos::prelude::*;
use qrcode::QrCode;
use qrcode::render::svg;

#[component]
pub fn QrCode(
    #[prop(into)] data: Signal<String>,
    #[prop(into)] dark_c: Signal<String>,
    #[prop(into)] light_c: Signal<String>,
    #[prop(into, optional)] width: MaybeProp<Signal<f64>>,
    #[prop(into, optional)] height: MaybeProp<Signal<f64>>,
    #[prop(into, optional)] aspect_divisor: MaybeProp<Signal<f64>>,
) -> impl IntoView {
    view! {
        <img
            src=move || {
                let qr = QrCode::new(data.get().as_bytes()).unwrap();
                let svg_data = qr
                    .render::<svg::Color>()
                    .dark_color(svg::Color(dark_c.get().as_str()))
                    .light_color(svg::Color(light_c.get().as_str()))
                    .build();
                format!("data:image/svg+xml;utf8,{}", urlencoding::encode(&svg_data))
            }
            width=move || {
                width.get().map(|w| w.get()).unwrap_or(256.)
                    / aspect_divisor.get().map(|a| a.get()).unwrap_or(1.)
            }
            height=move || {
                height.get().map(|h| h.get()).unwrap_or(256.)
                    / aspect_divisor.get().map(|a| a.get()).unwrap_or(1.)
            }
        />
    }
}
