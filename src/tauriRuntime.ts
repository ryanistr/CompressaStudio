import { invoke } from '@tauri-apps/api/core'

type TauriWindow = Window & {
  __TAURI_INTERNALS__?: unknown
}

export function isTauriRuntime(): boolean {
  return typeof window !== 'undefined' && Boolean((window as TauriWindow).__TAURI_INTERNALS__)
}

export async function invokeTauri<TResponse>(
  command: string,
  args?: Record<string, unknown>,
): Promise<TResponse> {
  if (!isTauriRuntime()) {
    throw new Error('This action requires the desktop app runtime.')
  }

  return invoke<TResponse>(command, args)
}
