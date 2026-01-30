<script lang="ts">
  import { onMount, onDestroy, createEventDispatcher } from 'svelte';
  import { EditorView, keymap, highlightActiveLine, drawSelection, dropCursor, rectangularSelection, crosshairCursor, highlightActiveLineGutter, lineNumbers } from '@codemirror/view';
  import { EditorState, Compartment } from '@codemirror/state';
  import { defaultKeymap, history, historyKeymap, indentWithTab } from '@codemirror/commands';
  import { markdown } from '@codemirror/lang-markdown';
  import { syntaxHighlighting, defaultHighlightStyle, bracketMatching, foldGutter, indentOnInput } from '@codemirror/language';
  import { oneDark } from '@codemirror/theme-one-dark';
  import { wikiLinkPlugin, wikiLinkTheme } from './wikiLinkPlugin';
  
  export let content: string = '';
  export let readonly: boolean = false;
  
  const dispatch = createEventDispatcher<{
    change: { content: string };
    linkClick: { target: string };
  }>();
  
  let editorContainer: HTMLDivElement;
  let view: EditorView | null = null;
  const readonlyCompartment = new Compartment();
  
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
      keymap.of([
        ...defaultKeymap,
        ...historyKeymap,
        indentWithTab,
      ]),
      markdown(),
      syntaxHighlighting(defaultHighlightStyle, { fallback: true }),
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
