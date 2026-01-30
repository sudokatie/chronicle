/**
 * Global keyboard shortcuts
 */
import { goto } from '$app/navigation';
import { createNote, saveCurrentNote } from '$lib/stores/editor';
import { get } from 'svelte/store';
import { isVaultOpen } from '$lib/stores/vault';

export interface Shortcut {
  key: string;
  ctrl?: boolean;
  shift?: boolean;
  alt?: boolean;
  action: () => void | Promise<void>;
  description: string;
}

export const shortcuts: Shortcut[] = [
  {
    key: 'n',
    ctrl: true,
    description: 'New note',
    action: async () => {
      if (!get(isVaultOpen)) return;
      const title = prompt('Note title:');
      if (title) await createNote(title);
    },
  },
  {
    key: 's',
    ctrl: true,
    description: 'Save note',
    action: () => saveCurrentNote(),
  },
  {
    key: 'g',
    ctrl: true,
    description: 'Toggle graph view',
    action: () => {
      const currentPath = window.location.pathname;
      goto(currentPath === '/graph' ? '/' : '/graph');
    },
  },
  {
    key: 'f',
    ctrl: true,
    shift: true,
    description: 'Global search',
    action: () => {
      // Focus search input - will implement with quick open modal
      const searchInput = document.querySelector('input[type="text"][placeholder*="Search"]') as HTMLInputElement;
      if (searchInput) searchInput.focus();
    },
  },
];

export function matchesShortcut(event: KeyboardEvent, shortcut: Shortcut): boolean {
  const ctrl = shortcut.ctrl ?? false;
  const shift = shortcut.shift ?? false;
  const alt = shortcut.alt ?? false;
  
  const ctrlMatch = (event.ctrlKey || event.metaKey) === ctrl;
  const shiftMatch = event.shiftKey === shift;
  const altMatch = event.altKey === alt;
  const keyMatch = event.key.toLowerCase() === shortcut.key.toLowerCase();
  
  return ctrlMatch && shiftMatch && altMatch && keyMatch;
}

export function handleGlobalShortcut(event: KeyboardEvent): boolean {
  // Don't trigger shortcuts when typing in inputs (except for Ctrl+S)
  const target = event.target as HTMLElement;
  const isInput = target.tagName === 'INPUT' || target.tagName === 'TEXTAREA' || target.isContentEditable;
  
  for (const shortcut of shortcuts) {
    if (matchesShortcut(event, shortcut)) {
      // Allow Ctrl+S even in inputs
      if (isInput && !(shortcut.key === 's' && shortcut.ctrl)) {
        continue;
      }
      
      event.preventDefault();
      shortcut.action();
      return true;
    }
  }
  
  return false;
}
