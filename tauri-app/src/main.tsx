import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import {
  onOpenUrl,
  getCurrent as getCurrentDeepLinkUrls,
} from "@tauri-apps/plugin-deep-link";

function handler(urls: string[]) {
  console.log(urls);
}

window.addEventListener("DOMContentLoaded", () => {
  onOpenUrl(handler);

  getCurrentDeepLinkUrls()
    .then((res) => {
      console.log("res", res);
    })
    .catch(console.error);
});

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);
