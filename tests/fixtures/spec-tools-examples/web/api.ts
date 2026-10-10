// SPDX-License-Identifier: MIT
export async function get(url: string): Promise<unknown> {
  const response = await fetch(url);
  return response.json();
}

export function parse(raw: string): any {
  return JSON.parse(raw);
}
