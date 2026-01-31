/**
 * Tauri API wrapper for Chronicle commands
 */
import { invoke } from '@tauri-apps/api/core';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';

// Types matching Rust structs

export interface VaultInfo {
  path: string;
  note_count: number;
  is_open: boolean;
}

export interface NoteMeta {
  id: number;
  path: string;
  title: string;
  word_count: number;
  created_at: string | null;
  modified_at: string | null;
}

export interface Note extends NoteMeta {
  content: string;
  tags: string[];
}

export interface SearchResult {
  id: number;
  path: string;
  title: string;
  snippet: string;
  rank: number;
  match_count: number;
}

export interface Backlink {
  source_path: string;
  source_title: string;
  line_number: number | null;
  display_text: string | null;
  context: string | null;
}

export interface GraphNode {
  id: string;
  title: string;
  word_count: number;
}

export interface GraphEdge {
  source: string;
  target: string;
}

export interface GraphData {
  nodes: GraphNode[];
  edges: GraphEdge[];
}

export interface TagInfo {
  name: string;
  count: number;
}

export interface AppConfig {
  vault: VaultConfig;
  editor: EditorConfig;
  graph: GraphConfig;
  ui: UiConfig;
}

export interface VaultConfig {
  path: string | null;
}

export interface EditorConfig {
  font_family: string;
  font_size: number;
  line_height: number;
  word_wrap: boolean;
  vim_mode: boolean;
}

export interface GraphConfig {
  physics_enabled: boolean;
  link_distance: number;
  charge_strength: number;
  node_size: number;
}

export interface UiConfig {
  sidebar_width: number;
  panel_width: number;
  show_backlinks: boolean;
  show_tags: boolean;
}

export type VaultEvent =
  | { type: 'note_created'; path: string }
  | { type: 'note_modified'; path: string }
  | { type: 'note_deleted'; path: string }
  | { type: 'note_renamed'; old_path: string; new_path: string }
  | { type: 'index_complete'; note_count: number };

// Vault commands

export async function openVault(path: string): Promise<VaultInfo> {
  return invoke('open_vault', { path });
}

export async function getVaultInfo(): Promise<VaultInfo> {
  return invoke('get_vault_info');
}

export async function closeVault(): Promise<void> {
  return invoke('close_vault');
}

export async function pollVaultEvents(): Promise<void> {
  return invoke('poll_vault_events');
}

// Note commands

export async function listNotes(): Promise<NoteMeta[]> {
  return invoke('list_notes');
}

export async function getNote(path: string): Promise<Note> {
  return invoke('get_note', { path });
}

export async function createNote(title: string, content?: string): Promise<NoteMeta> {
  return invoke('create_note', { title, content });
}

export async function saveNote(path: string, content: string): Promise<NoteMeta> {
  return invoke('save_note', { path, content });
}

export async function deleteNote(path: string): Promise<void> {
  return invoke('delete_note', { path });
}

export async function renameNote(oldPath: string, newPath: string): Promise<NoteMeta> {
  return invoke('rename_note', { oldPath, newPath });
}

export async function updateNoteTags(path: string, tags: string[]): Promise<NoteMeta> {
  return invoke('update_note_tags', { path, tags });
}

// Search commands

export async function searchNotes(query: string, limit?: number): Promise<SearchResult[]> {
  return invoke('search_notes', { query, limit });
}

export async function getBacklinks(path: string): Promise<Backlink[]> {
  return invoke('get_backlinks_cmd', { path });
}

// Graph commands

export async function getGraphData(): Promise<GraphData> {
  return invoke('get_graph_data');
}

// Tag commands

export async function listTags(): Promise<TagInfo[]> {
  return invoke('list_tags');
}

export async function getNotesByTag(tag: string): Promise<NoteMeta[]> {
  return invoke('get_notes_by_tag', { tag });
}

// Config commands

export async function getConfig(): Promise<AppConfig> {
  return invoke('get_config');
}

export async function saveConfig(config: AppConfig): Promise<void> {
  return invoke('save_config', { config });
}

// Event listener

export function onVaultEvent(callback: (event: VaultEvent) => void): Promise<UnlistenFn> {
  return listen<VaultEvent>('vault-event', (event) => {
    callback(event.payload);
  });
}
