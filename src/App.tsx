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
