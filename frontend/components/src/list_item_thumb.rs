use hex_color::HexColor;

use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    #[prop_or_default]
    pub text: String,

    #[prop_or_default]
    pub color: String,
}

// from https://wunnle.com/dynamic-text-color-based-on-background
fn rgb(c: u8) -> f64 {
    if c as f64 / 255.0 <= 0.03928 {
        c as f64 / 255.0 / 12.92
    } else {
        f64::powf((c as f64 / 255.0 + 0.055) / 1.055, 2.4)
    }
}

fn luminance(color: &str) -> f64 {
    let color = HexColor::parse_rgb(color).unwrap();
    (0.2126 * rgb(color.r)) + (0.7152 * rgb(color.g)) + (0.0722 * rgb(color.b))
}

fn contrast(foreground: &str, background: &str) -> f64 {
    let l1 = luminance(foreground);
    let l2 = luminance(background);
    (f64::max(l1, l2) + 0.05) / (f64::min(l1, l2) + 0.05)
}

#[function_component(ListItemThumb)]
pub fn list_item_thumb(Props { text, color }: &Props) -> Html {
    let white_contrast = contrast(color, "#ffffff");
    let black_contrast = contrast(color, "#555555");
    let text_color = if white_contrast > black_contrast {
        "text-white"
    } else {
        "text-black"
    };
    html! {
        <div style={format!("background-color: {};", color)} class={classes!(
            "md:p-6", "mr-3", "md:mr-4", "w-6", "md:w-20",
            "text-2xl", "rounded-lg", "text-center",
            "flex", "flex-col", "justify-center", "items-center",
            "min-h-full", "md:min-h-fit", text_color,
            "drop-shadow-sm"
        )}>
            <span class={classes!("hidden", "md:inline")}>{text}</span>
        </div>
    }
}
