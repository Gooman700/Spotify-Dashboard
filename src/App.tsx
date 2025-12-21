import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import {open} from "@tauri-apps/plugin-dialog";
import "./App.css";

function App() {
  const [status, setStatus] = useState("No file selected");

  async function handleImport() {
    try {
      const selected = await open({
        multiple: false,
        filters: [{ name: 'JSON', extensions: ['json'] }]
      });

      if (selected) {
        setStatus("Reading file...");
        const message = await invoke<string>("import_extended_history", { path: selected });
        setStatus(message);
      }
    } catch (err) {
      setStatus(`Error: ${err}`);
    }
  }

  return (
    <main className="container">
      <h1>Spotify History Importer</h1>
      
      <div className="card">
        <button onClick={handleImport}>
          Select Spotify JSON File
        </button>
        <p>{status}</p>
      </div>
    </main>
  );
}

export default App;
