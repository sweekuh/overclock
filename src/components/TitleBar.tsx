import { useState, useEffect } from "react";
import { getCurrentWindow } from "@tauri-apps/api/window";

interface TitleBarProps {
  step?: number;
  totalSteps?: number;
}

export function TitleBar({ step, totalSteps = 5 }: TitleBarProps) {
  // Gracefully handle running in a normal browser (localhost) vs Tauri wrapper
  const isTauri = typeof window !== 'undefined' && '__TAURI_INTERNALS__' in (window as any);
  const appWindow = isTauri ? getCurrentWindow() : null;
  
  const [skin, setSkin] = useState<"precision" | "retro">(() => {
    return (localStorage.getItem("overclock_skin") as "precision" | "retro") || "precision";
  });
  const [theme, setTheme] = useState<"dark" | "light">(() => {
    return (localStorage.getItem("overclock_theme") as "dark" | "light") || "dark";
  });

  useEffect(() => {
    document.body.setAttribute("data-skin", skin);
    document.body.setAttribute("data-theme", theme);
    localStorage.setItem("overclock_skin", skin);
    localStorage.setItem("overclock_theme", theme);
  }, [skin, theme]);

  const toggleSkin = () => {
    setSkin((s) => (s === "precision" ? "retro" : "precision"));
  };

  const toggleTheme = () => {
    setTheme((t) => (t === "dark" ? "light" : "dark"));
  };

  return (
    <div className="title-bar">
      <div className="title-bar__brand">
        <span className="title-bar__bolt">&#9889;</span>
        OVERCLOCK
      </div>

      {step !== undefined && (
        <span className="title-bar__step">
          Step {step} of {totalSteps}
        </span>
      )}

      <div className="title-bar__controls">
        <button
          className="title-bar__btn"
          onClick={toggleTheme}
          title={`Switch to ${theme === "dark" ? "Light" : "Dark"} Mode`}
        >
          {theme === "dark" ? "☼" : "☾"}
        </button>
        <button
          className="title-bar__btn"
          style={{ fontSize: "12px", width: "auto", padding: "0 8px" }}
          onClick={toggleSkin}
          title="Toggle Skin"
        >
          [ {skin.toUpperCase()} ]
        </button>

        <button
          className="title-bar__btn"
          onClick={() => appWindow?.minimize()}
          aria-label="Minimize"
        >
          &#x2500;
        </button>
        <button
          className="title-bar__btn"
          onClick={async () => {
            if (!appWindow) return;
            const maximized = await appWindow.isMaximized();
            maximized ? appWindow.unmaximize() : appWindow.maximize();
          }}
          aria-label="Maximize"
        >
          &#x25A1;
        </button>
        <button
          className="title-bar__btn title-bar__btn--close"
          onClick={() => appWindow?.close()}
          aria-label="Close"
        >
          &#x2715;
        </button>
      </div>
    </div>
  );
}
