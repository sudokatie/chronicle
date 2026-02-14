/**
 * Type declarations for @tauri-apps/plugin-fs.
 * This plugin is optional - external plugins won't work without it.
 */

declare module '@tauri-apps/plugin-fs' {
  export function readDir(path: string): Promise<{ name?: string; isDirectory: boolean }[]>;
  export function readTextFile(path: string): Promise<string>;
  export function exists(path: string): Promise<boolean>;
}
