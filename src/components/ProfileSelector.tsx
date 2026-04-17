import { useState } from "react";
import type { Profile, DetectedGame } from "../types";

interface ProfileSelectorProps {
  profiles: Profile[];
  games: DetectedGame[];
  selected: Profile | null;
  onSelect: (profile: Profile) => void;
  onContinue: () => void;
  onBack: () => void;
}

function countChanges(profile: Profile): number {
  let count = 0;
  if (profile.power_ultimate) count++;
  if (profile.usb_suspend_disable) count++;
  if (profile.nagle_disable) count++;
  if (profile.interrupt_mod_disable) count++;
  if (profile.wake_on_lan_disable) count++;
  if (profile.mouse_accel_disable) count++;
  if (profile.process_priority_high) count++;
  if (profile.background_apps_disable) count++;
  count += profile.disable_services.length;
  return count;
}

export function ProfileSelector({
  profiles,
  games,
  selected,
  onSelect,
  onContinue,
  onBack,
}: ProfileSelectorProps) {
  const [showGames, setShowGames] = useState(false);

  return (
    <div>
      <div className="section-label">Select Profile</div>

      <div className="profile-list">
        {profiles.map((profile) => {
          const isSelected = selected?.id === profile.id;
          const changes = countChanges(profile);

          return (
            <div
              key={profile.id}
              className={`profile-card ${isSelected ? "profile-card--selected" : ""}`}
              onClick={() => onSelect(profile)}
              role="button"
              tabIndex={0}
              onKeyDown={(e) => {
                if (e.key === "Enter" || e.key === " ") {
                  e.preventDefault();
                  onSelect(profile);
                }
              }}
            >
              <div className="profile-card__info">
                <div className="profile-card__name">{profile.name}</div>
                <div className="profile-card__desc">{profile.description}</div>
              </div>
              <div className="profile-card__meta">
                {changes} change{changes !== 1 ? "s" : ""}
              </div>
            </div>
          );
        })}
      </div>

      {games.length > 0 && (
        <div className="game-panel">
          <button
            className="game-panel__toggle"
            onClick={() => setShowGames(!showGames)}
          >
            <span className="game-panel__icon">🎮</span>
            <span className="game-panel__label">
              <strong>{games.length}</strong> game{games.length !== 1 ? "s" : ""} detected
            </span>
            <span className={`game-panel__chevron ${showGames ? "game-panel__chevron--open" : ""}`}>
              ›
            </span>
          </button>

          {showGames && (
            <div className="game-panel__list">
              {games.map((game, i) => (
                <div className="game-panel__item" key={i}>
                  <div className="game-panel__game-name">{game.name}</div>
                  <div className="game-panel__game-meta">
                    {game.exe_name} · {game.source}
                  </div>
                </div>
              ))}
            </div>
          )}
        </div>
      )}

      <div className="actions">
        <button className="btn btn--secondary" onClick={onBack}>
          ← Back
        </button>
        <button
          className="btn btn--primary"
          onClick={onContinue}
          disabled={!selected}
        >
          Continue →
        </button>
      </div>
    </div>
  );
}
