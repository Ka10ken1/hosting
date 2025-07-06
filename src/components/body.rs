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

        <div class="decorative-element" style="
                position: absolute;
                top: 10%;
                left: -10%;
                width: 300px;
                height: 300px;
                background: radial-gradient(circle, rgba(0, 255, 136, 0.1) 0%, transparent 70%);
                border-radius: 50%;
                animation: float 6s ease-in-out infinite;
                "></div>

        <div class="decorative-element" style="
                position: absolute;
                top: 60%;
                right: -15%;
                width: 400px;
                height: 400px;
                background: radial-gradient(circle, rgba(0, 119, 204, 0.1) 0%, transparent 70%);
                border-radius: 50%;
                animation: float 8s ease-in-out infinite reverse;
                "></div>

        <div class="decorative-element" style="
                position: absolute;
                top: 30%;
                left: 5%;
                width: 2px;
                height: 100px;
                background: linear-gradient(180deg, transparent, rgba(0, 255, 136, 0.3), transparent);
                animation: pulse 4s ease-in-out infinite;
                "></div>

        <div class="decorative-element" style="
                position: absolute;
                top: 70%;
                right: 8%;
                width: 2px;
                height: 80px;
                background: linear-gradient(180deg, transparent, rgba(0, 119, 204, 0.3), transparent);
                animation: pulse 3s ease-in-out infinite;
                "></div>

        // Geometric shapes
        <div class="decorative-element" style="
                position: absolute;
                top: 20%;
                left: 2%;
                width: 20px;
                height: 20px;
                border: 2px solid rgba(0, 255, 136, 0.3);
                transform: rotate(45deg);
                animation: rotate 10s linear infinite;
                "></div>

        <div class="decorative-element" style="
                position: absolute;
                top: 80%;
                right: 3%;
                width: 25px;
                height: 25px;
                border: 2px solid rgba(0, 119, 204, 0.3);
                border-radius: 50%;
                animation: pulse 5s ease-in-out infinite;
                "></div>

        <div class="decorative-element" style="
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
        <div class="decorative-element" style="
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

        <div class="decorative-element" style="
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
        <div class="main-container" style={format!("
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
                        class="profile-img"
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
                <h1 class="main-title" style="
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
                <p class="subtitle" style="
                        color: rgba(255, 255, 255, 0.9);
                        font-size: 1.2rem;
                        margin-bottom: 2rem;
                        line-height: 1.6;
                        font-weight: 300;
                        ">
                    { "Rustacean 🦀 | Hardware Whisperer 🔧 | Software Lover ⚡ | Math Enthusiast ∑" }
                </p>
                <div class="button-container" style="display: flex; justify-content: center; gap: 1rem; flex-wrap: wrap;">
                    <a href="https://github.com/Ka10ken1" target="_blank" rel="noopener noreferrer" class="action-button" style="
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
                    <a href="mailto:matekopaliani12@gmail.com" class="action-button" style="
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
                    <a href="https://www.linkedin.com/in/mate-kopaliani-8838a7277/" target="_blank" rel="noopener noreferrer" class="action-button" style="
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
            // i want gaps between the list items
            <ul style="list-style: none; padding-left: 0; padding-bottom: 1rem;">
                <li style="margin-bottom: 0.5rem;">
                    <strong>{ "Student Tutor of Linear Algebra " }</strong>
                    <p style="margin: 0.3rem 0 0 0; font-weight: 300;">
                        { "I was student tutor of Linear Algebra at Kutaisi International Univerisity Where i helped my peer's with the subject" }
                    </p>
                </li>

                <li style="margin-bottom: 0.5rem;">
                    <strong>
                        { "Software Engineer Intern — " }
                        <a href="https://suliko.ge" target="_blank" rel="noopener noreferrer" style="text-decoration: none; color: #1a73e8;">
                            {"SULIKO.GE"}
                        </a>
                        { " (Spring 2025)" }
                    </strong>
                    <p style="margin: 0.3rem 0 0 0; font-weight: 300;">
                        { "Worked as a back-end .NET developer focused on building an OCR product using microservices architecture,
                        with additional responsibilities as a Prompt Engineer. Along with regular back-end stuff" }
                    </p>
                </li>
            </ul>
            }, *dark_mode)}

            { section_with_id(String::from("projects"), "Projects", html! {
            <ul style="list-style: none; padding-left: 0;">


                <li style="margin-bottom: 1rem;">
                    <a href="https://github.com/Ka10ken1/FloatingPointUnit" target="_blank" rel="noopener noreferrer" style="
                            color: #00ff88;
                            font-weight: 600;
                            text-decoration: none;
                            ">
                        { "IEEE-754 Floating Point Unit" }
                    </a>
                    <p style="margin: 0.3rem 0 0 0; font-weight: 300;">
                        { "Complete IEEE-754 compliant FPU supporting both single (32-bit) and double (64-bit) precision arithmetic.
                          Implements addition, subtraction, multiplication, and division operations with proper exception handling 
                          and rounding modes. SystemVerilog implementation optimized for FPGA synthesis." }
                    </p>
                </li>

                <li style="margin-bottom: 1rem;">
                    <a href="https://github.com/Ka10ken1/Jobless-AI" target="_blank" rel="noopener noreferrer" style="
                        color: #00ff88;
                        font-weight: 600;
                        text-decoration: none;
                        ">
                        { "Jobless-AI" }
                    </a>
                    <p style="margin: 0.3rem 0 0 0; font-weight: 300;">
                        { "
                        Jobless-AI is a fine-tuned Hugging Face model with nothing but time and purpose — to help you land a job in tech.
                        Trained on a rich mix of job descriptions and resumes. it intelligently scrapes LinkedIn, hr.ge, and other job platforms to hunt down
                        the roles that actually fit you.
                        " }
                    </p>
                </li>


                <li style="margin-bottom: 1rem;">
                    <a href="https://github.com/Ka10ken1/NP-final" target="_blank" rel="noopener noreferrer" style="
                        color: #00ff88;
                        font-weight: 600;
                        text-decoration: none;
                        ">
                        {"Numerical Methods for Sturm–Liouville Problems"}
                    </a>
                    <p style="margin: 0.3rem 0 0 0; font-weight: 300;">
                        { "Implementation and comparative analysis of numerical methods for solving eigenvalue problems in Sturm-Liouville
                          differential equations. Includes finite difference, shooting method, and spectral approaches with convergence 
                          analysis and error estimation for boundary value problems in mathematical physics." }
                    </p>
                </li>


                <li style="margin-bottom: 1rem;">
                    <a href="https://ka10ken1.github.io/edge-detection/" target="_blank" rel="noopener noreferrer" style="
                            color: #00ff88;
                            font-weight: 600;
                            text-decoration: none;
                            ">
                        { "Edge Detection Algorithm Analysis" }
                    </a>
                    <p style="margin: 0.3rem 0 0 0; font-weight: 300;">
                        { "Comprehensive comparative study of edge detection algorithms including Sobel, Canny, Prewitt, and Laplacian operators.
                          Features performance benchmarking across different image types, noise conditions, and computational complexity analysis. 
                          Interactive web-based visualization with real-time parameter tuning and quality metrics evaluation." }
                    </p>
                </li>

                <li style="margin-bottom: 1rem;">
                    <a href="https://github.com/Ka10ken1/full-stack-library" target="_blank" rel="noopener noreferrer" style="
                            color: #00ff88;
                            font-weight: 600;
                            text-decoration: none;
                            ">
                        { "Full Stack Library Management System" }
                    </a>
                    <p style="margin: 0.3rem 0 0 0; font-weight: 300;">
                        { "Complete library management system with React frontend and Spring Boot backend featuring user authentication, book inventory, and loan tracking." }
                    </p>
                </li>

            </ul>
            }, *dark_mode)}


        </div>


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
        <h2 class="section-title" style="
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
