import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import type { Profile } from "../types";

export function useProfiles() {
  const [profiles, setProfiles] = useState<Profile[]>([]);
  const [selected, setSelected] = useState<Profile | null>(null);

  useEffect(() => {
    invoke<Profile[]>("get_profiles").then(setProfiles).catch(console.error);
  }, []);

  return { profiles, selected, setSelected };
}
