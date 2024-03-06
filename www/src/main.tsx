// import * as buffer from "buffer";
// // eslint-disable-next-line @typescript-eslint/no-explicit-any
// (window as any).Buffer = buffer.Buffer;

import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App.tsx";
import "./index.css";
import { nearConnectionConfig, viewMethodOnContract } from "./lib/utils.ts";

viewMethodOnContract(nearConnectionConfig, "get_solution").then(
  (hashed: string) => {
    console.log(hashed);

    return ReactDOM.createRoot(document.getElementById("root")!).render(
      <React.StrictMode>
        <App />
      </React.StrictMode>
    );
  }
);
// ReactDOM.createRoot(document.getElementById("root")!).render(
//   <React.StrictMode>
//     <App />
//   </React.StrictMode>
// );
