/**
 * CodeMirror plugin for wiki-style [[links]]
 */
import { Decoration, EditorView, ViewPlugin } from '@codemirror/view';
import type { DecorationSet, ViewUpdate } from '@codemirror/view';
import { RangeSetBuilder } from '@codemirror/state';

// Regex for wiki links: [[target]] or [[target|display]]
const WIKI_LINK_RE = /\[\[([^\]|]+)(?:\|([^\]]+))?\]\]/g;

// Decoration for the wiki link
const wikiLinkMark = Decoration.mark({ class: 'cm-wiki-link' });

// Create decorations for wiki links
function getWikiLinkDecorations(view: EditorView): DecorationSet {
  const builder = new RangeSetBuilder<Decoration>();
  
  for (const { from, to } of view.visibleRanges) {
    const text = view.state.sliceDoc(from, to);
    let match;
    
    while ((match = WIKI_LINK_RE.exec(text)) !== null) {
      const start = from + match.index;
      const end = start + match[0].length;
      builder.add(start, end, wikiLinkMark);
    }
  }
  
  return builder.finish();
}

// View plugin that creates and updates decorations
export function wikiLinkPlugin(onLinkClick: (target: string) => void) {
  return ViewPlugin.fromClass(
    class {
      decorations: DecorationSet;
      
      constructor(view: EditorView) {
        this.decorations = getWikiLinkDecorations(view);
      }
      
      update(update: ViewUpdate) {
        if (update.docChanged || update.viewportChanged) {
          this.decorations = getWikiLinkDecorations(update.view);
        }
      }
    },
    {
      decorations: (v) => v.decorations,
      eventHandlers: {
        click: (event: MouseEvent, view: EditorView) => {
          // Check if Cmd/Ctrl is held
          if (!event.metaKey && !event.ctrlKey) return false;
          
          const pos = view.posAtCoords({ x: event.clientX, y: event.clientY });
          if (pos === null) return false;
          
          // Get text around click position
          const line = view.state.doc.lineAt(pos);
          const lineText = line.text;
          
          // Find wiki link at position
          let match;
          while ((match = WIKI_LINK_RE.exec(lineText)) !== null) {
            const start = line.from + match.index;
            const end = start + match[0].length;
            
            if (pos >= start && pos <= end) {
              const target = match[1].trim();
              onLinkClick(target);
              event.preventDefault();
              return true;
            }
          }
          
          // Reset regex lastIndex
          WIKI_LINK_RE.lastIndex = 0;
          return false;
        },
      },
    }
  );
}

// Theme for wiki links
export const wikiLinkTheme = EditorView.baseTheme({
  '.cm-wiki-link': {
    color: '#7dd3fc', // sky-300
    textDecoration: 'none',
    cursor: 'pointer',
    '&:hover': {
      textDecoration: 'underline',
    },
  },
});
