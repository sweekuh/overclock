import { useState, useEffect, useRef } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import type { HardwareProfile, Profile, AppStep } from "./types";
import { TitleBar } from "./components/TitleBar";
import { StepProgress } from "./components/StepProgress";
import { DetectScreen } from "./components/DetectScreen";
import { ProfileSelector } from "./components/ProfileSelector";
import { ChangePreview } from "./components/ChangePreview";
import { ApplyProgress } from "./components/ApplyProgress";
import { ResultsSummary } from "./components/ResultsSummary";
import { RevertPanel } from "./components/RevertPanel";
import { StepTransition } from "./components/StepTransition";

type FullStep = AppStep | "revert_prompt";

const STEP_MAP: Record<FullStep, number> = {
  revert_prompt: 0,
  detecting: 1,
  select_profile: 2,
  preview: 3,
  applying: 4,
  results: 5,
};

interface SnapshotInfo {
  exists: boolean;
  timestamp: string;
  profile: string;
  change_count: number;
}

function App() {
  const [step, setStep] = useState<FullStep>("detecting");
  const [hardware, setHardware] = useState<HardwareProfile | null>(null);
  const [profiles, setProfiles] = useState<Profile[]>([]);
  const [selectedProfile, setSelectedProfile] = useState<Profile | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [isAdmin, setIsAdmin] = useState(true);
  const [snapshotInfo, setSnapshotInfo] = useState<SnapshotInfo | null>(null);

  // Apply state
  const [applyChanges, setApplyChanges] = useState<string[]>([]);
  const [applyIndex, setApplyIndex] = useState(0);
  const [applyResults, setApplyResults] = useState<("applied" | "skipped" | "failed" | "pending")[]>([]);

  // Transition direction tracking
  const [direction, setDirection] = useState<"forward" | "backward" | "commit">("forward");
  const prevStepRef = useRef(step);

  const navigateTo = (target: FullStep) => {
    const from = STEP_MAP[prevStepRef.current] ?? 0;
    const to = STEP_MAP[target] ?? 0;
    if (target === "applying") {
      setDirection("commit");
    } else if (to > from) {
      setDirection("forward");
    } else {
      setDirection("backward");
    }
    prevStepRef.current = target;
    setStep(target);
  };

  // ─── Init: check admin + detect hardware + check snapshot ─────────────────
  useEffect(() => {
    async function init() {
      try {
        const admin = await invoke<boolean>("check_admin");
        setIsAdmin(admin);
        if (!admin) {
          setLoading(false);
          return;
        }

        // Check for existing snapshot BEFORE detection
        const snap = await invoke<SnapshotInfo | null>("check_snapshot");
        if (snap) {
          setSnapshotInfo(snap);
          setStep("revert_prompt");
          setLoading(false);
          return;
        }

        // No snapshot — proceed to detect
        const [hw, profs] = await Promise.all([
          invoke<HardwareProfile>("detect_hardware"),
          invoke<Profile[]>("get_profiles"),
        ]);

        setHardware(hw);
        setProfiles(profs);
        setLoading(false);
      } catch (err) {
        setError(String(err));
        setLoading(false);
      }
    }

    init();
  }, []);

  // ─── Continue to detect after revert or skip ────────────────────────────────
  const proceedToDetect = async () => {
    setSnapshotInfo(null);
    setLoading(true);

    try {
      const [hw, profs] = await Promise.all([
        invoke<HardwareProfile>("detect_hardware"),
        invoke<Profile[]>("get_profiles"),
      ]);

      setHardware(hw);
      setProfiles(profs);
      navigateTo("detecting");
      setLoading(false);
    } catch (err) {
      setError(String(err));
      setLoading(false);
    }
  };

  // ─── Real apply process via Tauri IPC ──────────────────────────────────────
  const handleApply = async (excludedKeys: string[]) => {
    if (!selectedProfile || !hardware) return;

    navigateTo("applying");
    setApplyChanges([]);
    setApplyResults([]);
    setApplyIndex(0);

    try {
      // Call the real optimizer backend with excluded changes
      const results = await invoke<{ title: string; status: string; message?: string }[]>(
        "apply_profile",
        { profileId: selectedProfile.id, hardware, excludedKeys }
      );

      // Populate the progress UI with real results
      const titles = results.map(r => r.title);
      const statuses = results.map(r => r.status as "applied" | "skipped" | "failed");

      setApplyChanges(titles);
      setApplyResults(statuses);
      setApplyIndex(titles.length);
      navigateTo("results");
    } catch (err) {
      setError(String(err));
    }
  };

  // ─── Admin guard ─────────────────────────────────────────────────────────
  if (!isAdmin && !loading) {
    return (
      <>
        <TitleBar />
        <div className="app-content">
          <div className="error-screen">
            <div className="error-screen__icon">&#x26A0;</div>
            <div className="error-screen__title">Administrator Required</div>
            <div className="error-screen__message">
              OVERCLOCK requires Administrator privileges to modify system settings.
              Right-click the application and select "Run as Administrator."
            </div>
          </div>
        </div>
      </>
    );
  }

  // ─── Loading state ───────────────────────────────────────────────────────
  if (loading) {
    return (
      <>
        <TitleBar />
        <div className="app-content">
          <div className="loading">
            <div className="loading__text">Detecting hardware...</div>
            <div className="loading__bar">
              <div className="loading__bar-fill" />
            </div>
          </div>
        </div>
      </>
    );
  }

  // ─── Error state ─────────────────────────────────────────────────────────
  if (error) {
    return (
      <>
        <TitleBar />
        <div className="app-content">
          <div className="error-screen">
            <div className="error-screen__icon">&#x2715;</div>
            <div className="error-screen__title">Something went wrong</div>
            <div className="error-screen__message">{error}</div>
            <div className="actions" style={{ marginTop: "var(--space-xl)", justifyContent: "center" }}>
              <button
                className="btn btn--primary"
                onClick={() => {
                  setError(null);
                  setLoading(true);
                  setStep("detecting");
                  setSelectedProfile(null);
                  setApplyChanges([]);
                  setApplyResults([]);
                  setApplyIndex(0);
                  // Re-trigger init
                  (async () => {
                    try {
                      const [hw, profs] = await Promise.all([
                        invoke<HardwareProfile>("detect_hardware"),
                        invoke<Profile[]>("get_profiles"),
                      ]);
                      setHardware(hw);
                      setProfiles(profs);
                      setLoading(false);
                    } catch (retryErr) {
                      setError(String(retryErr));
                      setLoading(false);
                    }
                  })();
                }}
              >
                Try Again
              </button>
            </div>
          </div>
        </div>
      </>
    );
  }

  // ─── Wizard steps ────────────────────────────────────────────────────────
  const stepNum = STEP_MAP[step] || 0;

  const renderStep = () => {
    if (step === "revert_prompt" && snapshotInfo) {
      return (
        <RevertPanel
          snapshot={snapshotInfo}
          onReverted={proceedToDetect}
          onSkip={proceedToDetect}
        />
      );
    }

    if (step === "detecting" && hardware) {
      return (
        <DetectScreen
          hardware={hardware}
          onContinue={() => navigateTo("select_profile")}
        />
      );
    }

    if (step === "select_profile" && hardware) {
      return (
        <ProfileSelector
          profiles={profiles}
          games={hardware.games}
          selected={selectedProfile}
          onSelect={setSelectedProfile}
          onContinue={() => navigateTo("preview")}
          onBack={() => navigateTo("detecting")}
        />
      );
    }

    if (step === "preview" && selectedProfile && hardware) {
      return (
        <ChangePreview
          profile={selectedProfile}
          hardware={hardware}
          onApply={handleApply}
          onBack={() => navigateTo("select_profile")}
        />
      );
    }

    if (step === "applying") {
      return (
        <ApplyProgress
          changes={applyChanges}
          currentIndex={applyIndex}
          results={applyResults}
        />
      );
    }

    if (step === "results" && selectedProfile) {
      return (
        <ResultsSummary
          profileName={selectedProfile.name}
          applied={applyResults.filter(r => r === "applied").length}
          skipped={applyResults.filter(r => r === "skipped").length}
          failed={applyResults.filter(r => r === "failed").length}
          onClose={() => {
            setSelectedProfile(null);
            navigateTo("detecting");
          }}
        />
      );
    }

    return null;
  };

  return (
    <>
      <TitleBar step={stepNum > 0 ? stepNum : undefined} />
      <div className="app-content">
        {stepNum > 0 && <StepProgress current={stepNum - 1} total={5} />}

        <StepTransition stepKey={step} direction={direction}>
          {renderStep()}
        </StepTransition>
      </div>
    </>
  );
}

export default App;
