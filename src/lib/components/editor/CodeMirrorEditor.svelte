<script lang="ts">
  import { onMount, onDestroy, createEventDispatcher } from 'svelte';
  import { EditorView, keymap, highlightActiveLine, drawSelection, dropCursor, rectangularSelection, crosshairCursor, highlightActiveLineGutter, lineNumbers } from '@codemirror/view';
  import { EditorState, Compartment } from '@codemirror/state';
  import { defaultKeymap, history, historyKeymap, indentWithTab } from '@codemirror/commands';
  import { markdown } from '@codemirror/lang-markdown';
  import { syntaxHighlighting, defaultHighlightStyle, bracketMatching, foldGutter, indentOnInput } from '@codemirror/language';
  import { autocompletion, completionKeymap } from '@codemirror/autocomplete';
  import type { CompletionContext, CompletionResult } from '@codemirror/autocomplete';
  import { oneDark } from '@codemirror/theme-one-dark';
  import { wikiLinkPlugin, wikiLinkTheme } from './wikiLinkPlugin';
  import { notes } from '$lib/stores/vault';
  import { get } from 'svelte/store';
  
  export let content: string = '';
  export let readonly: boolean = false;
  
  const dispatch = createEventDispatcher<{
    change: { content: string };
    linkClick: { target: string };
  }>();
  
  let editorContainer: HTMLDivElement;
  let view: EditorView | null = null;
  const readonlyCompartment = new Compartment();
  
  // Wiki-link autocompletion: triggers on [[
  function wikiLinkCompletion(context: CompletionContext): CompletionResult | null {
    // Look for [[ before cursor
    const line = context.state.doc.lineAt(context.pos);
    const textBefore = line.text.slice(0, context.pos - line.from);
    
    // Find last [[ that isn't closed
    const lastOpen = textBefore.lastIndexOf('[[');
    if (lastOpen === -1) return null;
    
    // Check if there's a ]] after the [[
    const afterOpen = textBefore.slice(lastOpen + 2);
    if (afterOpen.includes(']]')) return null;
    
    // Get the partial text typed after [[
    const query = afterOpen.toLowerCase();
    const from = line.from + lastOpen + 2;
    
    // Get notes from store
    const allNotes = get(notes);
    
    // Filter and map to completions
    const options = allNotes
      .filter(n => 
        n.title.toLowerCase().includes(query) ||
        n.path.toLowerCase().includes(query)
      )
      .slice(0, 15)
      .map(n => ({
        label: n.title,
        detail: n.path !== n.title + '.md' ? n.path : undefined,
        apply: (view: EditorView, completion: any, from: number, to: number) => {
          // Insert the note title and close with ]]
          const insert = n.title + ']]';
          view.dispatch({
            changes: { from, to, insert },
            selection: { anchor: from + insert.length },
          });
        },
      }));
    
    if (options.length === 0) return null;
    
    return {
      from,
      to: context.pos,
      options,
      filter: false, // We already filtered
    };
  }
  
  // Wrap selection with markers (for bold/italic)
  function wrapSelection(view: EditorView, before: string, after: string): boolean {
    const { from, to } = view.state.selection.main;
    const selectedText = view.state.sliceDoc(from, to);
    
    view.dispatch({
      changes: { from, to, insert: before + selectedText + after },
      selection: { anchor: from + before.length, head: to + before.length },
    });
    return true;
  }
  
  // Insert wiki link
  function insertWikiLink(view: EditorView): boolean {
    const { from, to } = view.state.selection.main;
    const selectedText = view.state.sliceDoc(from, to);
    
    if (selectedText) {
      // Wrap selection in [[]]
      view.dispatch({
        changes: { from, to, insert: `[[${selectedText}]]` },
        selection: { anchor: from + 2, head: to + 2 },
      });
    } else {
      // Insert [[]] and place cursor inside
      view.dispatch({
        changes: { from, insert: '[[]]' },
        selection: { anchor: from + 2 },
      });
    }
    return true;
  }
  
  // Change heading level
  function changeHeadingLevel(view: EditorView, increase: boolean): boolean {
    const { from } = view.state.selection.main;
    const line = view.state.doc.lineAt(from);
    const lineText = line.text;
    
    // Find current heading level
    const match = lineText.match(/^(#{0,6})\s*/);
    const currentLevel = match ? match[1].length : 0;
    
    let newLevel: number;
    if (increase) {
      newLevel = Math.min(currentLevel + 1, 6);
    } else {
      newLevel = Math.max(currentLevel - 1, 0);
    }
    
    const hashCount = match ? match[0].length : 0;
    const contentStart = hashCount;
    const content = lineText.slice(contentStart);
    
    const newPrefix = newLevel > 0 ? '#'.repeat(newLevel) + ' ' : '';
    const newLine = newPrefix + content.trimStart();
    
    view.dispatch({
      changes: { from: line.from, to: line.to, insert: newLine },
    });
    return true;
  }
  
  // Custom keymap for formatting
  const formattingKeymap = keymap.of([
    {
      key: 'Mod-b',
      run: (view) => wrapSelection(view, '**', '**'),
    },
    {
      key: 'Mod-i',
      run: (view) => wrapSelection(view, '_', '_'),
    },
    {
      key: 'Mod-k',
      run: insertWikiLink,
    },
    {
      key: 'Mod-]',
      run: (view) => changeHeadingLevel(view, true),
    },
    {
      key: 'Mod-[',
      run: (view) => changeHeadingLevel(view, false),
    },
  ]);
  
  function createEditor() {
    const extensions = [
      lineNumbers(),
      highlightActiveLineGutter(),
      history(),
      foldGutter(),
      drawSelection(),
      dropCursor(),
      indentOnInput(),
      bracketMatching(),
      rectangularSelection(),
      crosshairCursor(),
      highlightActiveLine(),
      formattingKeymap,
      keymap.of([
        ...defaultKeymap,
        ...historyKeymap,
        ...completionKeymap,
        indentWithTab,
      ]),
      markdown(),
      syntaxHighlighting(defaultHighlightStyle, { fallback: true }),
      autocompletion({
        override: [wikiLinkCompletion],
        activateOnTyping: true,
      }),
      oneDark,
      wikiLinkPlugin((target) => dispatch('linkClick', { target })),
      wikiLinkTheme,
      readonlyCompartment.of(EditorState.readOnly.of(readonly)),
      EditorView.updateListener.of((update) => {
        if (update.docChanged) {
          const newContent = update.state.doc.toString();
          dispatch('change', { content: newContent });
        }
      }),
      EditorView.theme({
        '&': {
          height: '100%',
          fontSize: '14px',
        },
        '.cm-scroller': {
          overflow: 'auto',
          fontFamily: 'ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace',
        },
        '.cm-content': {
          padding: '16px',
        },
        '.cm-gutters': {
          backgroundColor: 'transparent',
          borderRight: 'none',
        },
      }),
    ];
    
    const state = EditorState.create({
      doc: content,
      extensions,
    });
    
    view = new EditorView({
      state,
      parent: editorContainer,
    });
  }
  
  // Update content when prop changes
  $: if (view && content !== view.state.doc.toString()) {
    view.dispatch({
      changes: {
        from: 0,
        to: view.state.doc.length,
        insert: content,
      },
    });
  }
  
  // Update readonly state
  $: if (view) {
    view.dispatch({
      effects: readonlyCompartment.reconfigure(EditorState.readOnly.of(readonly)),
    });
  }
  
  onMount(() => {
    createEditor();
  });
  
  onDestroy(() => {
    view?.destroy();
  });
  
  export function focus() {
    view?.focus();
  }
</script>

<div bind:this={editorContainer} class="h-full w-full overflow-hidden"></div>
