interface ResultsSummaryProps {
  profileName: string;
  applied: number;
  skipped: number;
  failed: number;
  onClose: () => void;
}

export function ResultsSummary({ profileName, applied, skipped, failed, onClose }: ResultsSummaryProps) {
  const allGood = failed === 0;

  return (
    <div>
      <div style={{ textAlign: "center", paddingTop: "var(--space-2xl)" }}>
        <div style={{
          fontSize: "48px",
          marginBottom: "var(--space-lg)",
          color: allGood ? "var(--success)" : "var(--warning)",
        }}>
          {allGood ? "✓" : "⚠"}
        </div>

        <div className="section-title" style={{ textAlign: "center" }}>
          {allGood ? "Optimization Complete" : "Optimization Completed with Warnings"}
        </div>

        <div style={{
          fontSize: "14px",
          color: "var(--text-secondary)",
          marginBottom: "var(--space-2xl)",
        }}>
          {profileName} profile applied successfully
        </div>

        <div style={{
          display: "flex",
          justifyContent: "center",
          gap: "var(--space-2xl)",
          marginBottom: "var(--space-2xl)",
        }}>
          <div style={{ textAlign: "center" }}>
            <div style={{
              fontFamily: "var(--font-heading)",
              fontSize: "24px",
              fontWeight: 600,
              color: "var(--success)",
            }}>
              {applied}
            </div>
            <div style={{ fontSize: "12px", color: "var(--text-muted)", marginTop: "4px" }}>
              Applied
            </div>
          </div>

          {skipped > 0 && (
            <div style={{ textAlign: "center" }}>
              <div style={{
                fontFamily: "var(--font-heading)",
                fontSize: "24px",
                fontWeight: 600,
                color: "var(--warning)",
              }}>
                {skipped}
              </div>
              <div style={{ fontSize: "12px", color: "var(--text-muted)", marginTop: "4px" }}>
                Skipped
              </div>
            </div>
          )}

          {failed > 0 && (
            <div style={{ textAlign: "center" }}>
              <div style={{
                fontFamily: "var(--font-heading)",
                fontSize: "24px",
                fontWeight: 600,
                color: "var(--danger)",
              }}>
                {failed}
              </div>
              <div style={{ fontSize: "12px", color: "var(--text-muted)", marginTop: "4px" }}>
                Failed
              </div>
            </div>
          )}
        </div>

        <div className="snapshot-banner" style={{ textAlign: "left" }}>
          <span className="snapshot-banner__icon">&#x1F6E1;</span>
          <span>
            A snapshot was saved. Run OVERCLOCK again and select "Revert" to undo all changes.
          </span>
        </div>

        <div style={{
          fontSize: "13px",
          color: "var(--text-muted)",
          marginBottom: "var(--space-2xl)",
        }}>
          Some optimizations require a restart to take full effect.
        </div>

        <div className="actions actions--end">
          <button className="btn btn--primary" onClick={onClose}>
            Done
          </button>
        </div>
      </div>
    </div>
  );
}
