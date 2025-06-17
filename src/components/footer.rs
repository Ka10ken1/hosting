use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct FooterProps {
    pub dark_mode: bool,
}

#[function_component(Footer)]
pub fn footer(props: &FooterProps) -> Html {
    let FooterProps { dark_mode } = props;

    let bg_color = if *dark_mode {
        "rgba(0, 0, 0, 0.7)"
    } else {
        "rgba(0, 0, 0, 0.5)"
    };

    html!(
        <footer style={format!("
            background: {};
            backdrop-filter: blur(10px);
            color: white;
            text-align: center;
            padding: 1.5rem 2rem;
            margin-top: 3rem;
            font-size: 1rem;
            font-weight: 400;
            border-top: 1px solid rgba(255, 255, 255, 0.1);
            box-shadow: 0 -4px 12px rgba(0, 0, 0, 0.2);
        ", bg_color)}>
            <p style="
                margin: 0;
                background: linear-gradient(45deg, #00ff88, #0077cc);
                -webkit-background-clip: text;
                -webkit-text-fill-color: transparent;
                background-clip: text;
                font-weight: 700;
            ">
                { "© 2025 Mate's Lair. All rights reserved." }
            </p>
        </footer>
    )
}
