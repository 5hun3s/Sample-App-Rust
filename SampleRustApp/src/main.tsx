import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import ActiveWindowMonitor from "./components/activeWindowMonitor";


ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <ActiveWindowMonitor/>
  </React.StrictMode>,
);
