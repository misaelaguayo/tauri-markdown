import { useEffect, useState } from "react";
import ReactHtmlParser from 'react-html-parser'
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";
import "./App.css";

function App() {
    const [html, setHtml] = useState("");

    type Payload = {
        message: string;
    }

    useEffect(() => {
        const testAppHandle = async () => await invoke("test_app_handle");
        const listener = async () => await listen<Payload>("test", (event) => {
            setHtml(event.payload.message);
        })

        testAppHandle();
        listener();
    }, []);

    return (
        <div className="container">
            {html ? ReactHtmlParser(html) : ""}
        </div>
    );
}

export default App;
