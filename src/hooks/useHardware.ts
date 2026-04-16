import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import type { HardwareProfile } from "../types";

export function useHardware() {
  const [hardware, setHardware] = useState<HardwareProfile | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    let cancelled = false;

    async function detect() {
      try {
        setLoading(true);
        setError(null);
        const result = await invoke<HardwareProfile>("detect_hardware");
        if (!cancelled) {
          setHardware(result);
        }
      } catch (err) {
        if (!cancelled) {
          setError(String(err));
        }
      } finally {
        if (!cancelled) {
          setLoading(false);
        }
      }
    }

    detect();
    return () => { cancelled = true; };
  }, []);

  return { hardware, loading, error };
}
