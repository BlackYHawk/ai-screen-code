// Runtime environment detection
export const isTauri = (): boolean => {
  return typeof window !== 'undefined' && '__TAURI__' in window
}

export const isDev = (): boolean => {
  return import.meta.env.DEV
}
