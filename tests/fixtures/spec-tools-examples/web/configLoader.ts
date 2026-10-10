// SPDX-License-Identifier: MIT
import { get } from "./api";

export async function loadConfig(): Promise<unknown> {
  return get("/config.json");
}
