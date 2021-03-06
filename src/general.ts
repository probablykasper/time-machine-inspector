import { invoke } from '@tauri-apps/api/tauri'

export function popup(msg: string) {
  invoke('error_popup', { msg })
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export async function runCmd<T = any>(cmd: string, options: { [key: string]: any } = {}) {
  return (await invoke(cmd, options).catch((msg) => {
    popup(msg)
    throw msg
  })) as T
}
