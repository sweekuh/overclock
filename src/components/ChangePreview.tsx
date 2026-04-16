import { useState, useMemo } from "react";
import type { Profile, HardwareProfile } from "../types";

interface ChangePreviewProps {
  profile: Profile;
  hardware: HardwareProfile;
  onApply: (excludedKeys: string[]) => void;
  onBack: () => void;
}

interface ChangeItem {
  key: string;
  category: string;
  title: string;
  detail: string;
  risk: "Safe" | "Caution" | "Info";
}

function buildChangeList(profile: Profile, hardware: HardwareProfile): ChangeItem[] {
  const changes: ChangeItem[] = [];

  // Power changes
  if (profile.power_ultimate) {
    changes.push({
      key: "power_ultimate",
      category: "Power",
      title: "Activate Ultimate Performance power plan",
      detail: "Balanced → Ultimate Performance",
      risk: "Safe",
    });
  }
  if (profile.usb_suspend_disable) {
    changes.push({
      key: "usb_suspend_disable",
      category: "Power",
      title: "Disable USB Selective Suspend",
      detail: "Enabled → Disabled",
      risk: hardware.os.is_desktop ? "Safe" : "Caution",
    });
  }

  // Network changes
  if (profile.nagle_disable) {
    changes.push({
      key: "nagle_disable",
      category: "Network",
      title: "Disable Nagle's Algorithm",
      detail: "TcpAckFrequency: 0→1, TCPNoDelay: 0→1",
      risk: "Safe",
    });
  }
  if (profile.interrupt_mod_disable && hardware.network.supports_interrupt_mod) {
    changes.push({
      key: "interrupt_mod_disable",
      category: "Network",
      title: `Disable Interrupt Moderation on ${hardware.network.adapter_name}`,
      detail: "Enabled → Disabled",
      risk: "Caution",
    });
  }
  if (profile.wake_on_lan_disable) {
    changes.push({
      key: "wake_on_lan_disable",
      category: "Network",
      title: "Disable Wake-on-LAN",
      detail: "Enabled → Disabled",
      risk: "Safe",
    });
  }

  // Input changes
  if (profile.mouse_accel_disable) {
    changes.push({
      key: "mouse_accel_disable",
      category: "Input",
      title: "Disable Mouse Acceleration",
      detail: "MouseSpeed: 1→0, flat SPI curve",
      risk: "Safe",
    });
  }

  // Service changes
  for (const svc of profile.disable_services) {
    const descriptions: Record<string, string> = {
      SysMain: "Superfetch/SysMain — disk prefetching",
      DiagTrack: "Connected User Experiences — telemetry",
      XblAuthManager: "Xbox Live Auth — background Xbox service",
      WSearch: "Windows Search — indexer",
    };
    changes.push({
      key: `service_${svc}`,
      category: "Services",
      title: `Disable ${svc}`,
      detail: descriptions[svc] || `${svc} service`,
      risk: svc === "WSearch" ? "Caution" : "Safe",
    });
  }

  // Process changes
  if (profile.process_priority_high) {
    changes.push({
      key: "process_priority_high",
      category: "Process",
      title: "Set game process priority to High",
      detail: "IFEO CpuPriorityClass → 3 (High)",
      risk: "Safe",
    });
  }

  // Background changes
  if (profile.background_apps_disable) {
    changes.push({
      key: "background_apps_disable",
      category: "Background",
      title: "Disable background apps",
      detail: "GlobalUserDisabled → 1",
      risk: "Safe",
    });
  }

  return changes;
}

export function ChangePreview({ profile, hardware, onApply, onBack }: ChangePreviewProps) {
  const changes = useMemo(() => buildChangeList(profile, hardware), [profile, hardware]);
  const [excluded, setExcluded] = useState<Set<string>>(new Set());

  const toggleChange = (key: string) => {
    setExcluded(prev => {
      const next = new Set(prev);
      if (next.has(key)) {
        next.delete(key);
      } else {
        next.add(key);
      }
      return next;
    });
  };

  const toggleAll = () => {
    if (excluded.size === 0) {
      // Exclude all
      setExcluded(new Set(changes.map(c => c.key)));
    } else {
      // Include all
      setExcluded(new Set());
    }
  };

  const enabledCount = changes.length - excluded.size;

  // Group by category
  const groups: Record<string, ChangeItem[]> = {};
  for (const change of changes) {
    if (!groups[change.category]) groups[change.category] = [];
    groups[change.category].push(change);
  }

  return (
    <div>
      <div style={{ display: "flex", alignItems: "center", justifyContent: "space-between", marginBottom: "var(--space-sm)" }}>
        <div className="section-label" style={{ marginBottom: 0 }}>Review Changes</div>
        <div className="section-label" style={{ marginBottom: 0 }}>{profile.name}</div>
      </div>

      <div style={{
        display: "flex",
        justifyContent: "space-between",
        alignItems: "center",
        fontSize: "13px",
        color: "var(--text-secondary)",
        marginBottom: "var(--space-lg)",
      }}>
        <span>
          {enabledCount} of {changes.length} optimization{changes.length !== 1 ? "s" : ""} selected
        </span>
        <button
          className="btn-link"
          onClick={toggleAll}
        >
          {excluded.size === 0 ? "Deselect All" : "Select All"}
        </button>
      </div>

      <div className="change-list">
        {Object.entries(groups).map(([category, items]) => (
          <div key={category}>
            <div className="change-group-label">{category}</div>
            {items.map((item) => {
              const isExcluded = excluded.has(item.key);

              return (
                <div
                  className={`change-row ${isExcluded ? "change-row--disabled" : ""}`}
                  key={item.key}
                  onClick={() => toggleChange(item.key)}
                  style={{ cursor: "pointer" }}
                >
                  <div style={{ display: "flex", alignItems: "center", gap: "var(--space-md)" }}>
                    <div className={`change-toggle ${isExcluded ? "" : "change-toggle--checked"}`}>
                      {!isExcluded && "✓"}
                    </div>
                    <div className="change-row__info">
                      <div className="change-row__title">{item.title}</div>
                      <div className="change-row__detail">{item.detail}</div>
                    </div>
                  </div>
                  <span className={`badge badge--${item.risk.toLowerCase()}`}>
                    {item.risk}
                  </span>
                </div>
              );
            })}
          </div>
        ))}
      </div>

      <div className="snapshot-banner">
        <span className="snapshot-banner__icon">&#x1F6E1;</span>
        A snapshot will be saved before any changes are made.
      </div>

      <div className="actions">
        <button className="btn btn--ghost" onClick={onBack}>
          ← Back
        </button>
        <button
          className="btn btn--primary"
          onClick={() => onApply([...excluded])}
          disabled={enabledCount === 0}
        >
          Apply {enabledCount} Change{enabledCount !== 1 ? "s" : ""}
        </button>
      </div>
    </div>
  );
}
