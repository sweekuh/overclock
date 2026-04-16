import type { HardwareProfile } from "../types";
import { HardwareCard } from "./HardwareCard";

interface DetectScreenProps {
  hardware: HardwareProfile;
  onContinue: () => void;
}

export function DetectScreen({ hardware, onContinue }: DetectScreenProps) {
  const formatClock = (mhz: number) => {
    return mhz >= 1000 ? `${(mhz / 1000).toFixed(1)} GHz` : `${mhz} MHz`;
  };

  const formatMB = (mb: number) => {
    return mb >= 1024 ? `${Math.round(mb / 1024)} GB` : `${mb} MB`;
  };

  const gameCount = hardware.games.length;

  return (
    <div>
      <div className="section-label">Hardware Detected</div>

      <div className="hw-grid">
        <HardwareCard
          label="CPU"
          value={hardware.cpu.name}
          detail={`${hardware.cpu.cores}C/${hardware.cpu.threads}T · ${formatClock(hardware.cpu.max_clock_mhz)}`}
        />
        <HardwareCard
          label="GPU"
          value={hardware.gpu.name}
          detail={`${formatMB(hardware.gpu.vram_mb)} VRAM`}
        />
        <HardwareCard
          label="Memory"
          value={`${formatMB(hardware.memory.total_mb)} ${hardware.memory.memory_type}`}
          detail={hardware.memory.speed_mhz > 0 ? `${hardware.memory.speed_mhz} MHz` : undefined}
        />
        <HardwareCard
          label="Storage"
          value={`${hardware.storage.media_type} ${hardware.storage.total_gb > 0 ? hardware.storage.total_gb + ' GB' : ''}`}
          detail={hardware.storage.model}
        />
        <HardwareCard
          label="Network"
          value={hardware.network.description}
          detail={`${hardware.network.link_speed} · ${hardware.network.connection_type}`}
        />
        <HardwareCard
          label="OS"
          value={hardware.os.caption}
          detail={`Build ${hardware.os.build_number} · ${hardware.os.edition}`}
        />
      </div>

      {gameCount > 0 && (
        <div className="game-count">
          <span className="game-count__text">
            <strong>{gameCount}</strong> game{gameCount !== 1 ? 's' : ''} detected
            {hardware.games.some(g => g.source === "Steam") && " (Steam)"}
          </span>
        </div>
      )}

      <div className="actions actions--end">
        <button className="btn btn--primary" onClick={onContinue}>
          Continue →
        </button>
      </div>
    </div>
  );
}
