import React, { useState } from "react";
import ReactHtmlParser from 'react-html-parser'
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  const [convertMsg, setConvertMsg] = useState("");
  const [name, setName] = useState("");

  async function convert() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setConvertMsg(await invoke("convert", { name }));
  }

  return (
    <div className="container">
      <h1>Welcome to Tauri!</h1>

      <div className="row">
        <a href="https://vitejs.dev" target="_blank">
          <img src="/vite.svg" className="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://reactjs.org" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>

      <p>Click on the Tauri, Vite, and React logos to learn more.</p>

      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          convert();
        }}
      >
        <input
          id="convert-input"
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder="Enter a name..."
        />
        <button type="submit">Convert</button>
      </form>

      {convertMsg ? ReactHtmlParser(convertMsg) : ""}
    </div>
  );
}

export default App;
