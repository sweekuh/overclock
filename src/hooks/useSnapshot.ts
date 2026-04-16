import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import type { SnapshotInfo } from "../types";

export function useSnapshot() {
  const [snapshot, setSnapshot] = useState<SnapshotInfo | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    invoke<SnapshotInfo | null>("check_snapshot")
      .then((result) => {
        setSnapshot(result);
        setLoading(false);
      })
      .catch(() => {
        setLoading(false);
      });
  }, []);

  return { snapshot, loading };
}
