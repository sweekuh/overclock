import { getCurrentWindow } from "@tauri-apps/api/window";

interface TitleBarProps {
  step?: number;
  totalSteps?: number;
}

export function TitleBar({ step, totalSteps = 5 }: TitleBarProps) {
  const appWindow = getCurrentWindow();

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
          onClick={() => appWindow.minimize()}
          aria-label="Minimize"
        >
          &#x2500;
        </button>
        <button
          className="title-bar__btn"
          onClick={async () => {
            const maximized = await appWindow.isMaximized();
            maximized ? appWindow.unmaximize() : appWindow.maximize();
          }}
          aria-label="Maximize"
        >
          &#x25A1;
        </button>
        <button
          className="title-bar__btn title-bar__btn--close"
          onClick={() => appWindow.close()}
          aria-label="Close"
        >
          &#x2715;
        </button>
      </div>
    </div>
  );
}
