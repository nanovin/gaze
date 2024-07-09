import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import { createBrowserRouter, RouterProvider } from "react-router-dom";
import { Timeline } from "./layouts/Timeline";

export default function App() {
  const router = createBrowserRouter([
    {
      path: "/",
      element: <Timeline />,
    },
  ]);

  return <RouterProvider router={router} />;
}
