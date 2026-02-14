/**
 * Editor store - manages current note state
 */
import { writable, derived, get } from 'svelte/store';
import * as api from '$lib/api/tauri';
import type { Note, Backlink } from '$lib/api/tauri';
import { refreshNotes } from './vault';
import { pluginEvents } from '$lib/plugins';
import type { PluginNote } from '$lib/plugins';

// Editor state
export const currentNote = writable<Note | null>(null);
export const backlinks = writable<Backlink[]>([]);
export const isDirty = writable(false);
export const isSaving = writable(false);

// Derived
export const currentPath = derived(currentNote, ($note) => $note?.path ?? null);

// Helper to convert Note to PluginNote
function toPluginNote(note: Note): PluginNote {
  return {
    path: note.path,
    title: note.title,
    content: note.content,
    wordCount: note.word_count,
    tags: note.tags
  };
}

// Actions

export async function openNote(path: string): Promise<void> {
  const note = await api.getNote(path);
  currentNote.set(note);
  isDirty.set(false);
  
  // Load backlinks
  const links = await api.getBacklinks(path);
  backlinks.set(links);
  
  // Notify plugins
  pluginEvents.emitNoteOpen(toPluginNote(note));
}

export async function createNote(title: string): Promise<string> {
  const meta = await api.createNote(title);
  await refreshNotes();
  await openNote(meta.path);
  return meta.path;
}

export async function saveCurrentNote(): Promise<void> {
  const current = get(currentNote);
  if (!current) return;
  
  isSaving.set(true);
  try {
    await api.saveNote(current.path, current.content);
    isDirty.set(false);
    await refreshNotes();
    
    // Notify plugins
    pluginEvents.emitNoteSave(toPluginNote(current));
  } finally {
    isSaving.set(false);
  }
}

export function updateContent(content: string): void {
  currentNote.update((note) => {
    if (!note) return note;
    isDirty.set(true);
    return { ...note, content };
  });
  
  // Notify plugins
  pluginEvents.emitNoteChange(content);
}

export async function deleteCurrentNote(): Promise<void> {
  const current = get(currentNote);
  if (!current) return;
  
  await api.deleteNote(current.path);
  currentNote.set(null);
  backlinks.set([]);
  isDirty.set(false);
  await refreshNotes();
}

export function closeNote(): void {
  currentNote.set(null);
  backlinks.set([]);
  isDirty.set(false);
  
  // Notify plugins
  pluginEvents.emitNoteClose();
}

export async function reloadCurrentNote(): Promise<void> {
  const current = get(currentNote);
  if (!current) return;
  
  const note = await api.getNote(current.path);
  currentNote.set(note);
  
  // Reload backlinks
  const links = await api.getBacklinks(current.path);
  backlinks.set(links);
  
  await refreshNotes();
}
