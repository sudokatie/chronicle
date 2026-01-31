<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  
  export let content: string = '';
  
  const dispatch = createEventDispatcher<{ linkClick: { target: string } }>();
  
  // Simple markdown to HTML conversion
  function renderMarkdown(md: string): string {
    if (!md) return '';
    
    let html = md
      // Escape HTML first
      .replace(/&/g, '&amp;')
      .replace(/</g, '&lt;')
      .replace(/>/g, '&gt;')
      // Headers
      .replace(/^######\s+(.*)$/gm, '<h6>$1</h6>')
      .replace(/^#####\s+(.*)$/gm, '<h5>$1</h5>')
      .replace(/^####\s+(.*)$/gm, '<h4>$1</h4>')
      .replace(/^###\s+(.*)$/gm, '<h3>$1</h3>')
      .replace(/^##\s+(.*)$/gm, '<h2>$1</h2>')
      .replace(/^#\s+(.*)$/gm, '<h1>$1</h1>')
      // Bold and italic
      .replace(/\*\*\*(.+?)\*\*\*/g, '<strong><em>$1</em></strong>')
      .replace(/\*\*(.+?)\*\*/g, '<strong>$1</strong>')
      .replace(/\*(.+?)\*/g, '<em>$1</em>')
      .replace(/___(.+?)___/g, '<strong><em>$1</em></strong>')
      .replace(/__(.+?)__/g, '<strong>$1</strong>')
      .replace(/_(.+?)_/g, '<em>$1</em>')
      // Strikethrough
      .replace(/~~(.+?)~~/g, '<del>$1</del>')
      // Code blocks
      .replace(/```(\w*)\n([\s\S]*?)```/g, '<pre><code class="language-$1">$2</code></pre>')
      // Inline code
      .replace(/`([^`]+)`/g, '<code>$1</code>')
      // Wiki links - make clickable
      .replace(/\[\[([^\]|]+)\|([^\]]+)\]\]/g, '<a href="#" class="wiki-link" data-target="$1">$2</a>')
      .replace(/\[\[([^\]]+)\]\]/g, '<a href="#" class="wiki-link" data-target="$1">$1</a>')
      // Regular links
      .replace(/\[([^\]]+)\]\(([^)]+)\)/g, '<a href="$2" target="_blank" rel="noopener">$1</a>')
      // Images
      .replace(/!\[([^\]]*)\]\(([^)]+)\)/g, '<img src="$2" alt="$1" />')
      // Blockquotes
      .replace(/^>\s+(.*)$/gm, '<blockquote>$1</blockquote>')
      // Horizontal rules
      .replace(/^---$/gm, '<hr />')
      .replace(/^\*\*\*$/gm, '<hr />')
      // Unordered lists
      .replace(/^[\*\-]\s+(.*)$/gm, '<li>$1</li>')
      // Ordered lists
      .replace(/^\d+\.\s+(.*)$/gm, '<li>$1</li>')
      // Paragraphs (lines that aren't already wrapped)
      .replace(/^(?!<[h|p|u|o|l|b|c|d|i|a|s|hr]|$)(.+)$/gm, '<p>$1</p>');
    
    // Wrap consecutive <li> in <ul>
    html = html.replace(/(<li>.*<\/li>\n?)+/g, '<ul>$&</ul>');
    
    // Clean up blockquotes
    html = html.replace(/(<blockquote>.*<\/blockquote>\n?)+/g, (match) => {
      const content = match.replace(/<\/?blockquote>/g, '').trim();
      return `<blockquote>${content}</blockquote>`;
    });
    
    return html;
  }
  
  function handleClick(event: MouseEvent) {
    const target = event.target as HTMLElement;
    if (target.classList.contains('wiki-link')) {
      event.preventDefault();
      const linkTarget = target.dataset.target;
      if (linkTarget) {
        dispatch('linkClick', { target: linkTarget });
      }
    }
  }
  
  $: renderedHtml = renderMarkdown(content);
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<!-- svelte-ignore a11y-no-static-element-interactions -->
<div 
  class="preview-content h-full overflow-auto p-6 bg-neutral-950"
  on:click={handleClick}
>
  {@html renderedHtml}
</div>

<style>
  .preview-content {
    color: #e5e5e5;
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
    line-height: 1.6;
  }
  
  .preview-content :global(h1) {
    font-size: 2em;
    font-weight: 700;
    margin: 1em 0 0.5em;
    padding-bottom: 0.3em;
    border-bottom: 1px solid #404040;
  }
  
  .preview-content :global(h2) {
    font-size: 1.5em;
    font-weight: 600;
    margin: 1em 0 0.5em;
    padding-bottom: 0.2em;
    border-bottom: 1px solid #404040;
  }
  
  .preview-content :global(h3) {
    font-size: 1.25em;
    font-weight: 600;
    margin: 1em 0 0.5em;
  }
  
  .preview-content :global(h4),
  .preview-content :global(h5),
  .preview-content :global(h6) {
    font-size: 1em;
    font-weight: 600;
    margin: 1em 0 0.5em;
  }
  
  .preview-content :global(p) {
    margin: 0.5em 0;
  }
  
  .preview-content :global(a) {
    color: #60a5fa;
    text-decoration: none;
  }
  
  .preview-content :global(a:hover) {
    text-decoration: underline;
  }
  
  .preview-content :global(.wiki-link) {
    color: #a78bfa;
    cursor: pointer;
  }
  
  .preview-content :global(.wiki-link:hover) {
    color: #c4b5fd;
  }
  
  .preview-content :global(code) {
    background: #262626;
    padding: 0.2em 0.4em;
    border-radius: 4px;
    font-family: 'JetBrains Mono', ui-monospace, monospace;
    font-size: 0.9em;
  }
  
  .preview-content :global(pre) {
    background: #262626;
    padding: 1em;
    border-radius: 6px;
    overflow-x: auto;
    margin: 1em 0;
  }
  
  .preview-content :global(pre code) {
    background: none;
    padding: 0;
  }
  
  .preview-content :global(blockquote) {
    border-left: 4px solid #525252;
    padding-left: 1em;
    margin: 1em 0;
    color: #a3a3a3;
  }
  
  .preview-content :global(ul),
  .preview-content :global(ol) {
    margin: 0.5em 0;
    padding-left: 2em;
  }
  
  .preview-content :global(li) {
    margin: 0.25em 0;
  }
  
  .preview-content :global(hr) {
    border: none;
    border-top: 1px solid #404040;
    margin: 2em 0;
  }
  
  .preview-content :global(img) {
    max-width: 100%;
    border-radius: 4px;
  }
  
  .preview-content :global(strong) {
    font-weight: 600;
  }
  
  .preview-content :global(del) {
    text-decoration: line-through;
    color: #737373;
  }
</style>
