interface ApplyProgressProps {
  changes: string[];
  currentIndex: number;
  results: ("applied" | "skipped" | "failed" | "pending")[];
}

export function ApplyProgress({ changes, currentIndex, results }: ApplyProgressProps) {
  const completed = results.filter(r => r !== "pending").length;
  const total = changes.length;
  const pct = total > 0 ? Math.round((completed / total) * 100) : 0;

  return (
    <div>
      <div className="section-label">Applying Optimizations</div>

      <div style={{ marginBottom: "var(--space-xl)" }}>
        <div style={{
          display: "flex",
          justifyContent: "space-between",
          fontSize: "13px",
          color: "var(--text-secondary)",
          marginBottom: "var(--space-sm)",
        }}>
          <span>{completed} of {total} complete</span>
          <span>{pct}%</span>
        </div>
        <div className="loading__bar" style={{ width: "100%" }}>
          <div
            className="loading__bar-fill"
            style={{
              width: `${pct}%`,
              transition: "width var(--duration-standard) var(--ease-out-expo)",
              animation: "none",
              background: "var(--accent)",
            }}
          />
        </div>
      </div>

      <div className="change-list">
        {changes.map((change, i) => {
          const status = results[i] || "pending";
          const isCurrent = i === currentIndex;

          return (
            <div
              key={i}
              className={`change-row ${isCurrent ? "animate-pulse" : ""}`}
            >
              <div className="change-row__info">
                <div className="change-row__title">{change}</div>
              </div>
              <span style={{ display: "flex", alignItems: "center", fontSize: "13px" }}>
                <span className={`status-dot status-dot--${status === "pending" && isCurrent ? "applying" : status}`} />
                <span style={{
                  color: status === "applied" ? "var(--success)"
                    : status === "skipped" ? "var(--warning)"
                    : status === "failed" ? "var(--danger)"
                    : "var(--text-muted)",
                  fontFamily: "var(--font-body)",
                  fontSize: "12px",
                  textTransform: "capitalize",
                }}>
                  {isCurrent && status === "pending" ? "Applying..." : status}
                </span>
              </span>
            </div>
          );
        })}
      </div>
    </div>
  );
}
