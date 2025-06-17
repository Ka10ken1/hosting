use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct HeaderProps {
    pub dark_mode: bool,
    pub toggle_dark_mode: Callback<()>,
}

#[function_component(Header)]
pub fn header(props: &HeaderProps) -> Html {
    let HeaderProps {
        dark_mode,
        toggle_dark_mode,
    } = props;

    let on_toggle = {
        let toggle_dark_mode = toggle_dark_mode.clone();
        Callback::from(move |_| {
            toggle_dark_mode.emit(());
        })
    };

    let header_bg = if *dark_mode {
        "linear-gradient(135deg, rgba(26, 26, 46, 0.9) 0%, rgba(22, 33, 62, 0.9) 100%)"
    } else {
        "linear-gradient(135deg, rgba(102, 126, 234, 0.9) 0%, rgba(118, 75, 162, 0.9) 100%)"
    };

    let border_color = if *dark_mode {
        "rgba(255, 255, 255, 0.1)"
    } else {
        "rgba(255, 255, 255, 0.2)"
    };

    html! {
        <header style={format!("
            position: relative;
            background: {};
            backdrop-filter: blur(15px);
            padding: 2rem 2rem;
            color: white;
            box-shadow: 0 8px 32px rgba(0, 0, 0, 0.1);
            border-bottom: 1px solid {};
            overflow: hidden;
            transition: all 0.3s ease;
        ", header_bg, border_color)}>
            // Code-themed decorative elements
            <div style="
                position: absolute;
                top: 20%;
                left: 5%;
                font-family: 'Courier New', monospace;
                font-size: 0.8rem;
                opacity: 0.3;
                animation: fadeInOut 4s ease-in-out infinite;
            ">
                { "fn main() {" }
            </div>

            <div style="
                position: absolute;
                top: 60%;
                right: 8%;
                font-family: 'Courier New', monospace;
                font-size: 0.8rem;
                opacity: 0.3;
                animation: fadeInOut 3s ease-in-out infinite;
            ">
                { "println!(\"Hello, Viwer!\");" }
            </div>

            <div style="
                position: absolute;
                top: 40%;
                left: 10%;
                font-family: 'Courier New', monospace;
                font-size: 0.8rem;
                opacity: 0.3;
                animation: fadeInOut 5s ease-in-out infinite;
            ">
                { "}" }
            </div>

            <div style="
                display: flex;
                justify-content: space-between;
                align-items: center;
                max-width: 1400px;
                margin: 0 auto;
                position: relative;
                z-index: 1;
            ">
                <h1 style="
                    font-size: 1.8rem;
                    font-weight: 700;
                    margin: 0;
                    background: linear-gradient(45deg, #00ff88, #ffffff);
                    -webkit-background-clip: text;
                    -webkit-text-fill-color: transparent;
                    background-clip: text;
                ">
                    { "Welcome to My Lair" }
                </h1>

                <div style="
                    display: flex;
                    align-items: center;
                    gap: 20px;
                ">
                    <button
                        onclick={on_toggle}
                        style={format!("
                            background: {};
                            border: 2px solid rgba(255, 255, 255, 0.3);
                            border-radius: 25px;
                            padding: 8px 16px;
                            color: white;
                            font-size: 1rem;
                            cursor: pointer;
                            transition: all 0.3s ease;
                            backdrop-filter: blur(5px);
                            display: flex;
                            align-items: center;
                            gap: 8px;
                        ", if *dark_mode { "rgba(0, 0, 0, 0.3)" } else { "rgba(255, 255, 255, 0.2)" })}
                    >
                        <span>{ if *dark_mode { "☀️" } else { "🌙" } }</span>
                        <span>{ if *dark_mode { "Light" } else { "Dark" } }</span>
                    </button>

                    // Status indicator
                    <div style="
                        display: flex;
                        align-items: center;
                        gap: 8px;
                        font-size: 0.9rem;
                        opacity: 0.8;
                    ">
                        <div style="
                            width: 8px;
                            height: 8px;
                            background: #00ff88;
                            border-radius: 50%;
                            animation: pulse 2s ease-in-out infinite;
                        "></div>
                        { "Available for work" }
                    </div>
                </div>
            </div>

            <style>
                {r#"
                    @keyframes fadeInOut {
                        0%, 100% { opacity: 0.2; }
                        50% { opacity: 0.4; }
                    }
                    
                    button:hover {
                        transform: translateY(-2px);
                        box-shadow: 0 4px 15px rgba(0, 0, 0, 0.2);
                    }
                "#}
            </style>
        </header>
    }
}
