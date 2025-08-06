#!/usr/bin/env node

const fs = require('fs');
const path = require('path');

// Simple but robust markdown to HTML converter
class MarkdownConverter {
    constructor() {
        this.inCodeBlock = false;
        this.codeBlockLanguage = '';
    }

    escapeHtml(unsafe) {
        return unsafe
            .replace(/&/g, "&amp;")
            .replace(/</g, "&lt;")
            .replace(/>/g, "&gt;")
            .replace(/"/g, "&quot;")
            .replace(/'/g, "&#039;");
    }

    processInlineElements(line) {
        // Skip if we're in a code block
        if (this.inCodeBlock) {
            return this.escapeHtml(line);
        }

        // Inline code (backticks) - handle nested cases better
        line = line.replace(/`([^`\n]+)`/g, '<code>$1</code>');
        
        // Bold text (**text** or __text__) - improved regex to avoid conflicts
        line = line.replace(/\*\*([^\*\n]+)\*\*/g, '<strong>$1</strong>');
        line = line.replace(/__([^_\n]+)__/g, '<strong>$1</strong>');
        
        // Italic text (*text* or _text_) - avoid conflicts with bold
        line = line.replace(/(?<!\*)\*([^\*\n]+)\*(?!\*)/g, '<em>$1</em>');
        line = line.replace(/(?<!_)_([^_\n]+)_(?!_)/g, '<em>$1</em>');
        
        // Links [text](url) - improved to handle complex URLs
        line = line.replace(/\[([^\]]+)\]\(([^\)]+)\)/g, '<a href="$2">$1</a>');
        
        // Images ![alt](src) - convert to img tags
        line = line.replace(/!\[([^\]]*)\]\(([^\)]+)\)/g, '<img src="$2" alt="$1">');
        
        // Strikethrough ~~text~~
        line = line.replace(/~~([^~\n]+)~~/g, '<del>$1</del>');
        
        return line;
    }

    convertLine(line, index, lines) {
        const trimmed = line.trim();
        
        // Handle code blocks
        if (trimmed.startsWith('```')) {
            if (!this.inCodeBlock) {
                this.inCodeBlock = true;
                // Extract language if specified
                this.codeBlockLanguage = trimmed.substring(3).trim();
                const langClass = this.codeBlockLanguage ? ` class="language-${this.codeBlockLanguage}"` : '';
                return `<pre><code${langClass}>`;
            } else {
                this.inCodeBlock = false;
                this.codeBlockLanguage = '';
                return '</code></pre>';
            }
        }
        
        // If we're in a code block, just escape and return
        if (this.inCodeBlock) {
            return this.escapeHtml(line);
        }
        
        // Headers
        if (trimmed.startsWith('###### ')) {
            return `<h6>${this.processInlineElements(trimmed.substring(7))}</h6>`;
        } else if (trimmed.startsWith('##### ')) {
            return `<h5>${this.processInlineElements(trimmed.substring(6))}</h5>`;
        } else if (trimmed.startsWith('#### ')) {
            return `<h4>${this.processInlineElements(trimmed.substring(5))}</h4>`;
        } else if (trimmed.startsWith('### ')) {
            return `<h3>${this.processInlineElements(trimmed.substring(4))}</h3>`;
        } else if (trimmed.startsWith('## ')) {
            return `<h2>${this.processInlineElements(trimmed.substring(3))}</h2>`;
        } else if (trimmed.startsWith('# ')) {
            return `<h1>${this.processInlineElements(trimmed.substring(2))}</h1>`;
        }
        
        // Blockquotes
        if (trimmed.startsWith('> ')) {
            return `<blockquote>${this.processInlineElements(trimmed.substring(2))}</blockquote>`;
        }
        
        // Horizontal rules
        if (trimmed === '---' || trimmed === '***' || trimmed === '___') {
            return '<hr>';
        }
        
        // Unordered lists - improved detection
        const unorderedListMatch = trimmed.match(/^([-*+])\s+(.+)$/);
        if (unorderedListMatch) {
            return `<li>${this.processInlineElements(unorderedListMatch[2])}</li>`;
        }
        
        // Ordered lists - improved regex
        const orderedListMatch = trimmed.match(/^(\d+)\.\s+(.+)$/);
        if (orderedListMatch) {
            return `<li>${this.processInlineElements(orderedListMatch[2])}</li>`;
        }
        
        // Empty lines
        if (trimmed === '') {
            // Check if we're between headers (skip empty lines around headers)
            const prevLine = index > 0 ? lines[index - 1].trim() : '';
            const nextLine = index < lines.length - 1 ? lines[index + 1].trim() : '';
            
            const prevIsHeader = /^#{1,6}\s/.test(prevLine);
            const nextIsHeader = /^#{1,6}\s/.test(nextLine);
            const prevIsList = /^[-*+]\s|^\d+\.\s/.test(prevLine);
            const nextIsList = /^[-*+]\s|^\d+\.\s/.test(nextLine);
            const prevEndsWithColon = prevLine.endsWith(':');
            
            // Skip empty lines around headers, before lists after colons, or between list items
            if (prevIsHeader || nextIsHeader || (prevEndsWithColon && nextIsList) || (prevIsList && nextIsList)) {
                return ''; // Skip these empty lines
            }
            return '<br>';
        }
        
        // Regular paragraphs
        return `<p>${this.processInlineElements(line)}</p>`;
    }

    convert(markdown) {
        const lines = markdown.split('\n');
        const result = [];
        let inList = false;
        let listType = null; // 'ul' or 'ol'
        
        this.inCodeBlock = false;
        this.codeBlockLanguage = '';
        
        for (let i = 0; i < lines.length; i++) {
            const line = lines[i];
            const trimmed = line.trim();
            const convertedLine = this.convertLine(line, i, lines);
            
            // Handle list wrapping
            const isListItem = convertedLine.startsWith('<li>');
            const isOrderedListItem = /^\d+\.\s/.test(trimmed);
            const isUnorderedListItem = /^[-*+]\s/.test(trimmed);
            
            if (isListItem && !inList) {
                // Starting a new list
                inList = true;
                listType = isOrderedListItem ? 'ol' : 'ul';
                result.push(`<${listType}>`);
                result.push(convertedLine);
            } else if (isListItem && inList) {
                // Continuing current list, but check if type changed
                const currentListType = isOrderedListItem ? 'ol' : 'ul';
                if (currentListType !== listType) {
                    // Close current list and start new one
                    result.push(`</${listType}>`);
                    listType = currentListType;
                    result.push(`<${listType}>`);
                }
                result.push(convertedLine);
            } else if (!isListItem && inList) {
                // End of list
                result.push(`</${listType}>`);
                inList = false;
                listType = null;
                if (convertedLine) {
                    result.push(convertedLine);
                }
            } else {
                // Regular line
                if (convertedLine) {
                    result.push(convertedLine);
                }
            }
        }
        
        // Close any remaining open list
        if (inList) {
            result.push(`</${listType}>`);
        }
        
        return result.filter(line => line !== '').join('\n');
    }
}

function convertMarkdownFile(inputFile, outputFile) {
    try {
        const markdown = fs.readFileSync(inputFile, 'utf8');
        const converter = new MarkdownConverter();
        const html = converter.convert(markdown);
        
        if (outputFile) {
            fs.writeFileSync(outputFile, html, 'utf8');
            console.log(`✅ Converted ${inputFile} → ${outputFile}`);
        } else {
            console.log(html);
        }
    } catch (error) {
        console.error(`❌ Error converting ${inputFile}:`, error.message);
        process.exit(1);
    }
}

// CLI usage
if (require.main === module) {
    const args = process.argv.slice(2);
    
    if (args.length === 0) {
        console.error('Usage: node markdown-converter.js <input.md> [output.html]');
        console.error('       node markdown-converter.js <input.md>  (outputs to stdout)');
        process.exit(1);
    }
    
    const inputFile = args[0];
    const outputFile = args[1];
    
    if (!fs.existsSync(inputFile)) {
        console.error(`❌ Input file not found: ${inputFile}`);
        process.exit(1);
    }
    
    convertMarkdownFile(inputFile, outputFile);
}

module.exports = { MarkdownConverter, convertMarkdownFile };
