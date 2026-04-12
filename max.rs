use std::net::{TcpListener, TcpStream};
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

fn handle_client(mut stream: TcpStream) {
    let uptime = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let html = format!(r#"
    <!DOCTYPE html>
    <html>
    <head>
        <title>Rust Pulse</title>
        <style>
            body {{
                margin: 0;
                background: radial-gradient(circle at top, #1a1a1a, #000000);
                color: #ffffff;
                font-family: Arial, sans-serif;
            }}

            .header {{
                padding: 20px;
                text-align: center;
                font-size: 24px;
                background: linear-gradient(90deg, #ff0066, #6600ff);
                color: white;
                letter-spacing: 2px;
            }}

            .grid {{
                display: grid;
                grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
                gap: 20px;
                padding: 30px;
            }}

            .card {{
                background: rgba(255,255,255,0.05);
                border-radius: 12px;
                padding: 20px;
                backdrop-filter: blur(10px);
                box-shadow: 0 0 20px rgba(255,0,150,0.2);
                transition: transform 0.2s ease;
            }}

            .card:hover {{
                transform: scale(1.05);
            }}

            .status {{
                font-weight: bold;
                color: #00ff99;
            }}

            .warning {{
                color: #ffaa00;
            }}

            .danger {{
                color: #ff4444;
            }}

            .pulse {{
                animation: pulse 1.5s infinite;
            }}

            @keyframes pulse {{
                0% {{ opacity: 0.5; }}
                50% {{ opacity: 1; }}
                100% {{ opacity: 0.5; }}
            }}
        </style>
    </head>

    <body>

        <div class="header">
            let RUST PULSE ENGINE
        </div>

        <div class="grid">

            <div class="card">
                <h2>System Status</h2>
                <p class="status pulse">RUNNING</p>
            </div>

            <div class="card">
                <h2>Uptime</h2>
                <p>{} seconds</p>
            </div>

            <div class="card">
                <h2>CPU Load</h2>
                <p class="warning">Medium</p>
            </div>

            <div class="card">
                <h2>Memory</h2>
                <p class="danger">75% Used</p>
            </div>

        </div>

    </body>
    </html>
    "#, uptime);

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n{}",
        html
    );

    stream.write_all(response.as_bytes()).unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    println!("Rust Pulse running at http://127.0.0.1:8080");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_client(stream);
    }
}