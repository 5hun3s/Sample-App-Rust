import React from "react";
import ReactDOM from "react-dom/client";
import "./App.css";
// import App from "./App";
// import ActiveWindowMonitor from "./components/activeWindowMonitor";
import ActivityPage  from "./components/activityPage";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <ActivityPage />
  </React.StrictMode>,
);
