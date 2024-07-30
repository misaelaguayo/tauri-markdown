import React, { useEffect, useState } from "react";
import ReactHtmlParser from 'react-html-parser'
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";
import "./App.css";

function App() {
    const [convertMsg, setConvertMsg] = useState("");
    const [name, setName] = useState("");

    type Payload = {
        message: string;
    }

    useEffect(() => {
        const listener = async () => await listen<Payload>("test", (event) => {
            console.log("Event triggered from rust: " + event.payload.message);
        })

        listener();
    }, []);

    async function convert() {
        // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
        setConvertMsg(await invoke("convert", { name }));
    }

    async function testAppHandle() {
        await invoke("test_app_handle");
    }

    return (
        <div className="container">
            <button onClick={() => testAppHandle()}>Start Event Listener</button>

            {convertMsg ? ReactHtmlParser(convertMsg) : ""}
        </div>
    );
}

export default App;
