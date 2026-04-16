import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";

interface SnapshotInfo {
  exists: boolean;
  timestamp: string;
  profile: string;
  change_count: number;
}

interface RevertPanelProps {
  snapshot: SnapshotInfo;
  onReverted: () => void;
  onSkip: () => void;
}

export function RevertPanel({ snapshot, onReverted, onSkip }: RevertPanelProps) {
  const [reverting, setReverting] = useState(false);
  const [results, setResults] = useState<{ title: string; status: string; message?: string }[] | null>(null);
  const [error, setError] = useState<string | null>(null);

  const handleRevert = async () => {
    setReverting(true);
    setError(null);
    try {
      const res = await invoke<{ title: string; status: string; message?: string }[]>("revert_snapshot");
      setResults(res);
    } catch (err) {
      setError(String(err));
    } finally {
      setReverting(false);
    }
  };

  // Show results after revert
  if (results) {
    const applied = results.filter(r => r.status === "applied").length;
    const failed = results.filter(r => r.status === "failed").length;
    const allGood = failed === 0;

    return (
      <div>
        <div style={{ textAlign: "center", paddingTop: "var(--space-2xl)" }}>
          <div style={{
            fontSize: "48px",
            marginBottom: "var(--space-lg)",
            color: allGood ? "var(--success)" : "var(--warning)",
          }}>
            {allGood ? "↩" : "⚠"}
          </div>

          <div className="section-title" style={{ textAlign: "center" }}>
            {allGood ? "System Restored" : "Restore Completed with Warnings"}
          </div>

          <div style={{
            fontSize: "14px",
            color: "var(--text-secondary)",
            marginBottom: "var(--space-2xl)",
          }}>
            {applied} change{applied !== 1 ? "s" : ""} reverted successfully
            {failed > 0 && `, ${failed} failed`}
          </div>

          <div className="change-list" style={{ textAlign: "left", marginBottom: "var(--space-xl)" }}>
            {results.map((r, i) => (
              <div className="change-row" key={i}>
                <div className="change-row__info">
                  <div className="change-row__title">{r.title}</div>
                  {r.message && <div className="change-row__detail">{r.message}</div>}
                </div>
                <span className={`status-dot status-dot--${r.status}`} />
              </div>
            ))}
          </div>

          <div style={{
            fontSize: "13px",
            color: "var(--text-muted)",
            marginBottom: "var(--space-2xl)",
          }}>
            Restart your computer for all changes to take full effect.
          </div>

          <div className="actions actions--end">
            <button className="btn btn--primary" onClick={onReverted}>
              Continue to Optimize
            </button>
          </div>
        </div>
      </div>
    );
  }

  // Revert prompt
  return (
    <div>
      <div style={{ textAlign: "center", paddingTop: "var(--space-2xl)" }}>
        <div style={{
          fontSize: "48px",
          marginBottom: "var(--space-lg)",
          color: "var(--accent)",
        }}>
          ⚡
        </div>

        <div className="section-title" style={{ textAlign: "center" }}>
          Previous Optimization Found
        </div>

        <div style={{
          fontSize: "14px",
          color: "var(--text-secondary)",
          marginBottom: "var(--space-2xl)",
          maxWidth: "400px",
          margin: "0 auto var(--space-2xl)",
          lineHeight: "1.6",
        }}>
          A snapshot from a previous session was detected.
          You can revert those changes or continue to apply new optimizations.
        </div>

        <div className="snapshot-banner" style={{ textAlign: "left", marginBottom: "var(--space-xl)" }}>
          <span className="snapshot-banner__icon">&#x1F6E1;</span>
          <div>
            <div style={{ fontWeight: 500, color: "var(--text-primary)", marginBottom: "4px" }}>
              {snapshot.profile} profile
            </div>
            <div style={{ fontSize: "12px" }}>
              {snapshot.change_count} change{snapshot.change_count !== 1 ? "s" : ""} · Applied {snapshot.timestamp}
            </div>
          </div>
        </div>

        {error && (
          <div style={{
            color: "var(--danger)",
            fontSize: "13px",
            marginBottom: "var(--space-lg)",
          }}>
            {error}
          </div>
        )}

        <div className="actions" style={{ justifyContent: "center", gap: "var(--space-lg)" }}>
          <button className="btn btn--ghost" onClick={onSkip}>
            Skip — Optimize Again
          </button>
          <button
            className="btn btn--primary"
            onClick={handleRevert}
            disabled={reverting}
          >
            {reverting ? "Reverting..." : "↩ Revert All Changes"}
          </button>
        </div>
      </div>
    </div>
  );
}
