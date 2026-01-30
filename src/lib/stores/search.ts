/**
 * Search store - manages search state
 */
import { writable } from 'svelte/store';
import * as api from '$lib/api/tauri';
import type { SearchResult } from '$lib/api/tauri';

// Search state
export const searchQuery = writable('');
export const searchResults = writable<SearchResult[]>([]);
export const isSearching = writable(false);

let searchTimeout: ReturnType<typeof setTimeout> | null = null;

// Debounced search
export function setSearchQuery(query: string): void {
  searchQuery.set(query);
  
  if (searchTimeout) {
    clearTimeout(searchTimeout);
  }
  
  if (!query.trim()) {
    searchResults.set([]);
    return;
  }
  
  searchTimeout = setTimeout(() => {
    performSearch(query);
  }, 150);
}

async function performSearch(query: string): Promise<void> {
  if (!query.trim()) {
    searchResults.set([]);
    return;
  }
  
  isSearching.set(true);
  try {
    const results = await api.searchNotes(query, 20);
    searchResults.set(results);
  } catch (e) {
    console.error('Search error:', e);
    searchResults.set([]);
  } finally {
    isSearching.set(false);
  }
}

export function clearSearch(): void {
  searchQuery.set('');
  searchResults.set([]);
}
