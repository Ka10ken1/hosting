use super::body::Body;
use super::footer::Footer;
use super::header::Header;
use yew::{Callback, Html, function_component, html, use_state};

#[function_component]
pub fn Page() -> Html {
    let dark_mode = use_state(|| true);

    let toggle_dark_mode = {
        let dark_mode = dark_mode.clone();
        Callback::from(move |_| {
            dark_mode.set(!*dark_mode);
        })
    };

    let theme_colors = if *dark_mode {
        (
            "linear-gradient(135deg, #1a1a2e 0%, #16213e 100%)", // Dark background
            "rgba(255, 255, 255, 0.9)",                          // Light text
            "rgba(255, 255, 255, 0.1)",                          // Light card background
        )
    } else {
        (
            "linear-gradient(135deg, #667eea 0%, #764ba2 100%)", // Light background
            "white",                                             // White text
            "rgba(255, 255, 255, 0.1)",                          // White card background
        )
    };

    html!(
        <>
            <style>
                {format!(r#"
                    * {{
                        box-sizing: border-box;
                    }}
                    body, html {{
                        margin: 0;
                        padding: 0;
                        height: 100%;
                        font-family: 'Segoe UI', -apple-system, BlinkMacSystemFont, sans-serif;
                        background: {};
                        color: {};
                        scroll-behavior: smooth;
                        overflow-x: hidden;
                        transition: all 0.3s ease;
                    }}
                    #root {{
                        display: flex;
                        flex-direction: column;
                        min-height: 100vh;
                        position: relative;
                    }}
                    header, footer {{
                        flex-shrink: 0;
                    }}
                    a {{
                        transition: color 0.3s ease;
                    }}
                    a:hover {{
                        color: #00ff88;
                    }}

                    .floating-shape {{
                        position: absolute;
                        width: 150px;
                        height: 150px;
                        border-radius: 50%;
                        opacity: 0.15;
                        animation: float 6s ease-in-out infinite alternate;
                        pointer-events: none;
                        z-index: 0;
                    }}

                    .left-shape {{
                        background: radial-gradient(circle, #00ff88, transparent);
                        top: 20%;
                        left: -75px;
                    }}

                    .right-shape {{
                        background: radial-gradient(circle, #00d4ff, transparent);
                        bottom: 15%;
                        right: -75px;
                        animation-delay: 2s;
                    }}

                    .moving-object {{
                        position: absolute;
                        width: 60px;
                        height: 60px;
                        border-radius: 50%;
                        z-index: 0;
                        opacity: 0.2;
                        pointer-events: none;
                    }}

                    .object-left {{
                        background: linear-gradient(135deg, #00ff88, transparent);
                        top: 30%;
                        left: -100px;
                        animation: move-right 12s linear infinite;
                    }}

                    .object-right {{
                        background: linear-gradient(135deg, #0077cc, transparent);
                        bottom: 20%;
                        right: -100px;
                        animation: move-left 15s linear infinite;
                    }}

                    @keyframes float {{
                        0% {{
                            transform: translateY(0px) scale(1);
                        }}
                        50% {{
                            transform: translateY(-20px) scale(1.1);
                        }}
                        100% {{
                            transform: translateY(0px) scale(1);
                        }}
                    }}

                    @keyframes move-right {{
                        0% {{
                            transform: translateX(0) scale(1);
                        }}
                        50% {{
                            transform: translateX(120vw) scale(1.2) rotate(180deg);
                        }}
                        100% {{
                            transform: translateX(0) scale(1);
                        }}
                    }}

                    @keyframes move-left {{
                        0% {{
                            transform: translateX(0) scale(1);
                        }}
                        50% {{
                            transform: translateX(-120vw) scale(1.1) rotate(-180deg);
                        }}
                        100% {{
                            transform: translateX(0) scale(1);
                        }}
                    }}
                "#, theme_colors.0, theme_colors.1)}
            </style>

            <div id="root">
                <div class="floating-shape left-shape"></div>
                <div class="floating-shape right-shape"></div>

                <div class="moving-object object-left"></div>
                <div class="moving-object object-right"></div>

                <Header dark_mode={*dark_mode} toggle_dark_mode={toggle_dark_mode.clone()} />
                <Body dark_mode={*dark_mode} />
                <Footer dark_mode={*dark_mode} />
            </div>
        </>
    )
}
