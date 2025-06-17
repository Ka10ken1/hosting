use yew::{Html, Properties, function_component, html};

#[derive(Properties, PartialEq)]
pub struct BodyProps {
    pub dark_mode: bool,
}

#[function_component(Body)]
pub fn body(props: &BodyProps) -> Html {
    let BodyProps { dark_mode } = props;

    // Define theme colors based on dark mode
    let (bg_gradient, text_color, card_bg, border_color) = if *dark_mode {
        (
            "linear-gradient(135deg, #1a1a2e 0%, #16213e 100%)",
            "rgba(255, 255, 255, 0.9)",
            "rgba(255, 255, 255, 0.05)",
            "rgba(255, 255, 255, 0.1)",
        )
    } else {
        (
            "linear-gradient(135deg, #667eea 0%, #764ba2 100%)",
            "white",
            "rgba(255, 255, 255, 0.1)",
            "rgba(255, 255, 255, 0.2)",
        )
    };

    // Define the skill_item closure outside the html! macro
    let skill_item = |label: &str, svg: Html| {
        html! {
            <li style="display: flex; align-items: center; gap: 8px; font-weight: 600; color: #00ff88;">
                { svg }
                <span>{ label }</span>
            </li>
        }
    };

    html!(
        <main style={format!("
            position: relative;
            max-width: 1400px; 
            margin: auto; 
            padding: 3rem 2rem; 
            font-family: 'Segoe UI', -apple-system, BlinkMacSystemFont, sans-serif;
            background: {};
            min-height: 100vh;
            color: {};
            overflow: hidden;
            ", bg_gradient, text_color)}>

            // Decorative elements
            <div style="
                position: absolute;
                top: 10%;
                left: -10%;
                width: 300px;
                height: 300px;
                background: radial-gradient(circle, rgba(0, 255, 136, 0.1) 0%, transparent 70%);
                border-radius: 50%;
                animation: float 6s ease-in-out infinite;
                "></div>

            <div style="
                position: absolute;
                top: 60%;
                right: -15%;
                width: 400px;
                height: 400px;
                background: radial-gradient(circle, rgba(0, 119, 204, 0.1) 0%, transparent 70%);
                border-radius: 50%;
                animation: float 8s ease-in-out infinite reverse;
                "></div>

            <div style="
                position: absolute;
                top: 30%;
                left: 5%;
                width: 2px;
                height: 100px;
                background: linear-gradient(180deg, transparent, rgba(0, 255, 136, 0.3), transparent);
                animation: pulse 4s ease-in-out infinite;
                "></div>

            <div style="
                position: absolute;
                top: 70%;
                right: 8%;
                width: 2px;
                height: 80px;
                background: linear-gradient(180deg, transparent, rgba(0, 119, 204, 0.3), transparent);
                animation: pulse 3s ease-in-out infinite;
                "></div>

            // Geometric shapes
            <div style="
                position: absolute;
                top: 20%;
                left: 2%;
                width: 20px;
                height: 20px;
                border: 2px solid rgba(0, 255, 136, 0.3);
                transform: rotate(45deg);
                animation: rotate 10s linear infinite;
                "></div>

            <div style="
                position: absolute;
                top: 80%;
                right: 3%;
                width: 25px;
                height: 25px;
                border: 2px solid rgba(0, 119, 204, 0.3);
                border-radius: 50%;
                animation: pulse 5s ease-in-out infinite;
                "></div>

            <div style="
                position: absolute;
                top: 45%;
                left: 1%;
                width: 0;
                height: 0;
                border-left: 15px solid transparent;
                border-right: 15px solid transparent;
                border-bottom: 25px solid rgba(0, 255, 136, 0.2);
                animation: float 7s ease-in-out infinite;
                "></div>

            // Side decoration panels
            <div style="
                position: absolute;
                top: 0;
                left: 0;
                width: 120px;
                height: 100%;
                background: linear-gradient(90deg, 
                    rgba(0, 255, 136, 0.05) 0%, 
                    rgba(0, 255, 136, 0.02) 50%, 
                    transparent 100%);
                pointer-events: none;
                "></div>

            <div style="
                position: absolute;
                top: 0;
                right: 0;
                width: 120px;
                height: 100%;
                background: linear-gradient(270deg, 
                    rgba(0, 119, 204, 0.05) 0%, 
                    rgba(0, 119, 204, 0.02) 50%, 
                    transparent 100%);
                pointer-events: none;
                "></div>

            // Main content container
            <div style={format!("
                position: relative;
                z-index: 10;
                max-width: 900px;
                margin: 0 auto;
                background: {};
                backdrop-filter: blur(10px);
                border-radius: 20px;
                padding: 3rem;
                box-shadow: 0 8px 32px rgba(0, 0, 0, 0.1);
                border: 1px solid {};
                ", card_bg, border_color)}>

                // Profile Section - Home
                <section id="home" style="text-align: center; margin-bottom: 3rem; scroll-margin-top: 100px;">
                    <div style="position: relative; display: inline-block; margin-bottom: 2rem;">
                        <img
                            src="https://avatars.githubusercontent.com/u/583231?v=4"
                            alt="Profile picture"
                            style="
                                width: 180px; 
                                height: 180px; 
                                border-radius: 50%; 
                                border: 5px solid rgba(255, 255, 255, 0.3);
                                box-shadow: 0 10px 30px rgba(0, 0, 0, 0.2);
                                "
                        />
                        <div style="
                                position: absolute;
                                bottom: 10px;
                                right: 10px;
                                width: 25px;
                                height: 25px;
                                background: #00ff88;
                                border-radius: 50%;
                                border: 3px solid white;
                                box-shadow: 0 2px 8px rgba(0, 0, 0, 0.2);
                                "></div>
                    </div>
                    <h1 style="
                            font-size: 3.5rem; 
                            margin: 0 0 1rem 0;
                            background: linear-gradient(45deg, #fff, #f0f8ff);
                            -webkit-background-clip: text;
                            -webkit-text-fill-color: transparent;
                            background-clip: text;
                            font-weight: 700;
                            text-shadow: 0 4px 8px rgba(0, 0, 0, 0.3);
                            ">
                        { "Mate Kopaliani" }
                    </h1>
                    <p style="
                            color: rgba(255, 255, 255, 0.9);
                            font-size: 1.2rem;
                            margin-bottom: 2rem;
                            line-height: 1.6;
                            font-weight: 300;
                            ">
                        { "Rustacean 🦀 | Arch user btw 🐧 | Hardware Whisperer ⚡ | Math Enthusiast ∑" }
                    </p>
                    <div style="display: flex; justify-content: center; gap: 1rem; flex-wrap: wrap;">
                        <a href="https://github.com/Ka10ken1" target="_blank" rel="noopener noreferrer" style="
                                padding: 12px 24px;
                                background: rgba(255, 255, 255, 0.2);
                                border: 1px solid rgba(255, 255, 255, 0.3);
                                border-radius: 25px;
                                text-decoration: none;
                                color: white;
                                font-weight: 500;
                                backdrop-filter: blur(5px);
                                transition: all 0.3s ease;
                                ">
                            { "🐙 GitHub" }
                        </a>
                        <a href="mailto:matekopaliani12@gmail.com" style="
                                padding: 12px 24px;
                                background: rgba(255, 255, 255, 0.2);
                                border: 1px solid rgba(255, 255, 255, 0.3);
                                border-radius: 25px;
                                text-decoration: none;
                                color: white;
                                font-weight: 500;
                                backdrop-filter: blur(5px);
                                transition: all 0.3s ease;
                                ">
                            { "📧 Email" }
                        </a>
                        <a href="https://www.linkedin.com/in/mate-kopaliani-8838a7277/" target="_blank" rel="noopener noreferrer" style="
                                padding: 12px 24px;
                                background: rgba(255, 255, 255, 0.2);
                                border: 1px solid rgba(255, 255, 255, 0.3);
                                border-radius: 25px;
                                text-decoration: none;
                                color: white;
                                font-weight: 500;
                                backdrop-filter: blur(5px);
                                transition: all 0.3s ease;
                                ">
                            { "🔗 LinkedIn" }
                        </a>
                        // Download CV Button
                        <a href="/CV.pdf" download="Mate_Kopaliani_CV.pdf" target="_blank" rel="noopener noreferrer" style="
                                padding: 12px 24px;
                                background: linear-gradient(45deg, #00ff88, #00cc70);
                                border: 1px solid rgba(0, 255, 136, 0.5);
                                border-radius: 25px;
                                text-decoration: none;
                                color: white;
                                font-weight: 600;
                                backdrop-filter: blur(5px);
                                transition: all 0.3s ease;
                                box-shadow: 0 4px 15px rgba(0, 255, 136, 0.3);
                                ">
                            { "📄 Download CV" }
                        </a>
                    </div>
                </section>

                { section_with_id(String::from("education"), "Education", html! {
                    <ul style="list-style: none; padding-left: 0;">
                        <li style="margin-bottom: 1rem;">
                            <strong>{ "B.Sc. in Computer Science — Kutaisi International University (2022 - now)" }</strong>
                            <p style="margin: 0.3rem 0 0 0; font-weight: 300;">
                                { "Currently pursuing a degree in Computer Science with a minor in Mathematics." }
                            </p>
                        </li>
                    </ul>
                }, *dark_mode)}

                { section_with_id(String::from("experience"), "Experience", html! {
                    <ul style="list-style: none; padding-left: 0;">
                        <li>
                            <strong>{ "Bachelor Thesis — Floating Point Unit Design (2026)" }</strong>
                            <p style="margin: 0.3rem 0 0 0; font-weight: 300;">
                                { "Designed and implemented a pipelined IEEE 754-compliant Floating Point Unit (FPU) in Verilog. Developed a custom Rust-based testbench for functional verification and performance evaluation." }
                            </p>
                        </li>

                        <li>
                            <strong>{ "Software Engineer Intern — SULIKO (Spring 2025)" }</strong>
                            <p style="margin: 0.3rem 0 0 0; font-weight: 300;">
                                { "Worked as a back-end .NET developer focused on building an OCR product using microservices architecture, with additional responsibilities as a Prompt Engineer." }
                            </p>
                        </li>
                    </ul>
                }, *dark_mode)}

                { section_with_id(String::from("projects"), "Projects", html! {
                    <ul style="list-style: none; padding-left: 0;">
                        <li style="margin-bottom: 1rem;">
                            <a href="https://ka10ken1.github.io/edge-detection/" target="_blank" rel="noopener noreferrer" style="
                                    color: #00ff88;
                                    font-weight: 600;
                                    text-decoration: none;
                                    ">
                                { "Edge Detection Analysis" }
                            </a>
                            <p style="margin: 0.3rem 0 0 0; font-weight: 300;">
                                { "A Rust-based CLI tool to make your life easier. Handles tasks automatically so you don't have to." }
                            </p>
                        </li>
                        <li>
                            <a href="https://github.com/Ka10ken1/full-stack-library" target="_blank" rel="noopener noreferrer" style="
                                    color: #00ff88;
                                    font-weight: 600;
                                    text-decoration: none;
                                    ">
                                { "Full Stack Library Project" }
                            </a>
                            <p style="margin: 0.3rem 0 0 0; font-weight: 300;">
                                { "A path planning system for autonomous duckiebots with obstacle avoidance." }
                            </p>
                        </li>
                    </ul>
                }, *dark_mode)}

                { section_with_id(String::from("technologies"), "Technologies & Tools", html! {
                    <div style="display: flex; justify-content: center; align-items: center; padding: 1rem 0;">
                        <img
                            src="https://skillicons.dev/icons?i=rust,java,python,cpp,javascript,typescript,lua,mysql,postgres,linux,docker,git,latex"
                            alt="Technologies and Tools"
                            style="
                                max-width: 100%;
                                height: auto;
                                border-radius: 10px;
                                box-shadow: 0 4px 15px rgba(0, 0, 0, 0.2);
                                "
                        />
                    </div>
                }, *dark_mode)}

            </div>

            <style>
            {r#"
                @keyframes float {
                0%, 100% { transform: translateY(0px); }
                50% { transform: translateY(-20px); }
                }

                @keyframes pulse {
                0%, 100% { opacity: 0.3; }
                50% { opacity: 0.8; }
                }

                @keyframes rotate {
                0% { transform: rotate(45deg); }
                100% { transform: rotate(405deg); }
                }

                a:hover {
                transform: translateY(-2px) !important;
                background: rgba(255, 255, 255, 0.3) !important;
                }

                html {
                scroll-behavior: smooth;
                }
                "#}
            </style>
        </main>
    )
}

fn section_with_id(id: String, title: &str, content: Html, dark_mode: bool) -> Html {
    let border_color = if dark_mode {
        "rgba(255, 255, 255, 0.1)"
    } else {
        "rgba(255, 255, 255, 0.2)"
    };

    html! {
        <section id={id} style={format!("
            margin-bottom: 3rem;
            padding-bottom: 1.5rem;
            border-bottom: 1px solid {};
            scroll-margin-top: 100px;
            ", border_color)}>
            <h2 style="
                font-size: 2rem;
                font-weight: 700;
                margin-bottom: 1rem;
                background: linear-gradient(45deg, #00ff88, #0077cc);
                -webkit-background-clip: text;
                -webkit-text-fill-color: transparent;
                background-clip: text;
                ">
                { title }
            </h2>
            { content }
        </section>
    }
}
