#!/bin/bash

# Script to copy Criterion benchmark results to docs directory
# and create a navigation index

set -e

echo "📊 Copying benchmark results to docs directory..."

# Create docs directory if it doesn't exist
mkdir -p docs

# Remove existing benchmark results if present
if [ -d "docs/benchmarks" ]; then
    echo "🗑️  Removing existing benchmark results..."
    rm -rf docs/benchmarks
fi

# Copy criterion results to docs/benchmarks
if [ -d "target/criterion" ]; then
    echo "📋 Copying Criterion results..."
    cp -r target/criterion docs/benchmarks
    echo "✅ Benchmark results copied to docs/benchmarks/"
else
    echo "❌ Error: target/criterion directory not found. Run 'cargo bench' first."
    exit 1
fi

# Create main navigation index
echo "📝 Creating navigation index..."

cat > docs/index.html << 'EOF'
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>MPMC Queue Documentation</title>
    <style>
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            line-height: 1.6;
            color: #333;
            max-width: 1200px;
            margin: 0 auto;
            padding: 20px;
            background: #f8f9fa;
        }
        .header {
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            padding: 2rem;
            border-radius: 10px;
            margin-bottom: 2rem;
            text-align: center;
            position: relative;
        }
        .header h1 {
            margin: 0 0 0.5rem 0;
            font-size: 2.5rem;
            font-weight: 300;
        }
        .header .subtitle {
            margin: 0 0 1rem 0;
            opacity: 0.9;
            font-size: 1.1rem;
        }
        .header .performance-highlights {
            margin-top: 1rem;
            font-size: 0.95rem;
            opacity: 0.95;
            line-height: 1.4;
        }
        .header .performance-highlights ul {
            list-style: none;
            padding: 0;
            margin: 0.5rem 0 0 0;
            display: flex;
            flex-wrap: wrap;
            justify-content: center;
            gap: 1.5rem;
        }
        .header .performance-highlights li {
            margin: 0;
        }
        .github-corner {
            position: absolute;
            top: 0;
            right: 0;
            width: 80px;
            height: 80px;
            overflow: hidden;
        }
        .github-corner svg {
            fill: rgba(255, 255, 255, 0.9);
            color: #667eea;
            position: absolute;
            top: 0;
            border: 0;
            right: 0;
        }
        .github-corner:hover svg {
            fill: rgba(255, 255, 255, 1);
        }
        .nav-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
            gap: 1.5rem;
            margin-bottom: 2rem;
        }
        .nav-card {
            background: white;
            border-radius: 8px;
            padding: 1.5rem;
            box-shadow: 0 2px 10px rgba(0,0,0,0.1);
            transition: transform 0.2s ease, box-shadow 0.2s ease;
        }
        .nav-card:hover {
            transform: translateY(-2px);
            box-shadow: 0 4px 20px rgba(0,0,0,0.15);
        }
        .nav-card h2 {
            margin-top: 0;
            color: #667eea;
            font-size: 1.3rem;
        }
        .nav-card p {
            color: #666;
            margin-bottom: 1rem;
        }
        .nav-card a {
            display: inline-block;
            background: #667eea;
            color: white;
            padding: 0.5rem 1rem;
            text-decoration: none;
            border-radius: 5px;
            transition: background 0.2s ease;
        }
        .nav-card a:hover {
            background: #5a67d8;
        }
        .benchmark-section {
            background: white;
            border-radius: 8px;
            padding: 2rem;
            box-shadow: 0 2px 10px rgba(0,0,0,0.1);
            margin-bottom: 2rem;
        }
        .benchmark-section h2 {
            color: #667eea;
            border-bottom: 2px solid #eee;
            padding-bottom: 0.5rem;
        }
        .benchmark-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
            gap: 1rem;
            margin-top: 1rem;
        }
        .benchmark-item {
            background: #f8f9fa;
            padding: 1rem;
            border-radius: 5px;
            border-left: 4px solid #667eea;
        }
        .benchmark-item h4 {
            margin-top: 0;
            color: #333;
        }
        .benchmark-item a {
            color: #667eea;
            text-decoration: none;
        }
        .benchmark-item a:hover {
            text-decoration: underline;
        }
        .footer {
            text-align: center;
            padding: 2rem;
            color: #666;
            border-top: 1px solid #eee;
            margin-top: 2rem;
        }
    </style>
</head>
<body>
    <div class="header">
        <a href="https://github.com/abbychau/mpmc-std" class="github-corner" target="_blank" rel="noopener noreferrer">
            <svg xmlns="http://www.w3.org/2000/svg" width="80" height="80" viewBox="0 0 250 250" fill="#151513" style="position: absolute; top: 0; right: 0">
            <path d="M0 0l115 115h15l12 27 108 108V0z" fill="#000"/>
            <path class="octo-arm" d="M128 109c-15-9-9-19-9-19 3-7 2-11 2-11-1-7 3-2 3-2 4 5 2 11 2 11-3 10 5 15 9 16" style="-webkit-transform-origin: 130px 106px; transform-origin: 130px 106px"/>
            <path class="octo-body" d="M115 115s4 2 5 0l14-14c3-2 6-3 8-3-8-11-15-24 2-41 5-5 10-7 16-7 1-2 3-7 12-11 0 0 5 3 7 16 4 2 8 5 12 9s7 8 9 12c14 3 17 7 17 7-4 8-9 11-11 11 0 6-2 11-7 16-16 16-30 10-41 2 0 3-1 7-5 11l-12 11c-1 1 1 5 1 5z"/>
            </svg>
        </a>
        <h1>🚀 MPMC Queue Documentation</h1>
        <p class="subtitle">High-Performance Lockless Multi-Producer Multi-Consumer Queue</p>
        <div class="performance-highlights">
            <div>⚡ Performance Highlights:</div>
            <ul>
                <li><strong>8.9ns</strong> latency</li>
                <li><strong>1.8B ops/sec</strong> throughput</li>
                <li><strong>Linear scaling</strong> to 8 threads</li>
                <li><strong>Wait-free</strong> algorithm</li>
            </ul>
        </div>
    </div>

    <div class="nav-grid">
        <div class="nav-card">
            <h2>📚 Algorithm Documentation</h2>
            <p>Detailed explanations of the sequence-based ring buffer algorithm, memory layout optimization, multi-consumer speed analysis, and implementation details.</p>
            <a href="ALGORITHM_DIAGRAMS.html">View Algorithm Diagrams</a>
        </div>

        <div class="nav-card">
            <h2>🔬 Implementation Notes</h2>
            <p>Deep technical dive into memory ordering, cache optimization, safety guarantees, performance engineering decisions, and algorithm comparisons.</p>
            <a href="IMPLEMENTATION_NOTES.html">View Implementation Details</a>
        </div>

        <div class="nav-card">
            <h2>📊 Benchmark Results</h2>
            <p>Comprehensive performance analysis with interactive Criterion.rs reports showing throughput, latency, and scaling characteristics.</p>
            <a href="benchmarks/report/index.html">View All Benchmarks</a>
        </div>
    </div>

    <div class="benchmark-section">
        <h2>📈 Benchmark Categories</h2>
        
        <div class="benchmark-grid">
            <div class="benchmark-item">
                <h4>🔄 Single-Threaded Throughput</h4>
                <p>Raw performance measurement across different queue capacities (64, 256, 1024, 4096 elements).</p>
                <a href="benchmarks/single_threaded_throughput/report/index.html">View Results</a>
            </div>

            <div class="benchmark-item">
                <h4>👥→👤 Multi-Producer Single-Consumer</h4>
                <p>Scaling characteristics with 1, 2, 4, and 8 producer threads feeding a single consumer.</p>
                <a href="benchmarks/multi_producer_single_consumer/report/index.html">View Results</a>
            </div>

            <div class="benchmark-item">
                <h4>👤→👥 Single-Producer Multi-Consumer</h4>
                <p>Performance analysis with one producer feeding 1, 2, 4, and 8 consumer threads.</p>
                <a href="benchmarks/single_producer_multi_consumer/report/index.html">View Results</a>
            </div>

            <div class="benchmark-item">
                <h4>👥→👥 Multi-Producer Multi-Consumer</h4>
                <p>Full MPMC scenario with balanced producer-consumer thread pairs (1, 2, 4, 8 pairs).</p>
                <a href="benchmarks/multi_producer_multi_consumer/report/index.html">View Results</a>
            </div>

            <div class="benchmark-item">
                <h4>⚡ Latency Measurements</h4>
                <p>Detailed latency analysis for both send and receive operations with statistical distributions.</p>
                <a href="benchmarks/latency/report/index.html">View Results</a>
            </div>

            <div class="benchmark-item">
                <h4>🔥 High Contention Testing</h4>
                <p>Stress testing with 16 threads on different queue sizes (16, 64, 256 elements) to measure contention handling.</p>
                <a href="benchmarks/contention/report/index.html">View Results</a>
            </div>
        </div>
    </div>

    <div class="footer">
        <p>Generated automatically from Criterion.rs benchmark results</p>
        <p>⚡ Built for Speed, Designed for Safety, Optimized for Modern Hardware ⚡</p>
    </div>
</body>
</html>
EOF

# Convert markdown files to HTML with proper handling
echo "🔄 Converting markdown documentation to HTML..."

# Create a Python script for markdown conversion
cat > /tmp/md_to_html.py << 'EOF'
#!/usr/bin/env python3
import re
import html
import sys

def convert_markdown_to_html(filename):
    with open(filename, 'r', encoding='utf-8') as f:
        content = f.read()
    
    lines = content.split('\n')
    result = []
    in_code_block = False
    
    for line in lines:
        # Handle code blocks
        if line.strip().startswith('```'):
            if not in_code_block:
                in_code_block = True
                result.append('<pre><code>')
            else:
                in_code_block = False
                result.append('</code></pre>')
            continue
        
        if in_code_block:
            # Escape HTML in code blocks
            escaped_line = html.escape(line)
            result.append(escaped_line)
            continue
        
        # Process markdown outside code blocks
        
        # Headers
        if line.startswith('##### '):
            line = f'<h5>{line[6:]}</h5>'
        elif line.startswith('#### '):
            line = f'<h4>{line[5:]}</h4>'
        elif line.startswith('### '):
            line = f'<h3>{line[4:]}</h3>'
        elif line.startswith('## '):
            line = f'<h2>{line[3:]}</h2>'
        elif line.startswith('# '):
            line = f'<h1>{line[2:]}</h1>'
        else:
            # Inline code
            line = re.sub(r'`([^`]+)`', r'<code>\1</code>', line)
            
            # Bold text
            line = re.sub(r'\*\*([^*]+)\*\*', r'<strong>\1</strong>', line)
            
            # Convert empty lines to line breaks
            if line.strip() == '':
                line = '<br>'
        
        result.append(line)
    
    return '\n'.join(result)

if __name__ == '__main__':
    if len(sys.argv) != 2:
        print("Usage: python3 md_to_html.py <filename>")
        sys.exit(1)
    
    print(convert_markdown_to_html(sys.argv[1]))
EOF

# Function to create HTML file with proper structure
create_html_doc() {
    local md_file="$1"
    local html_file="$2"
    local title="$3"
    local subtitle="$4"
    
    # Create HTML header
    cat > "$html_file" << EOF
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>$title - MPMC Queue</title>
    <style>
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            line-height: 1.6;
            color: #333;
            max-width: 1000px;
            margin: 0 auto;
            padding: 20px;
            background: #f8f9fa;
        }
        .header {
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            padding: 2rem;
            border-radius: 10px;
            margin-bottom: 2rem;
            text-align: center;
            position: relative;
        }
        .content {
            background: white;
            padding: 2rem;
            border-radius: 8px;
            box-shadow: 0 2px 10px rgba(0,0,0,0.1);
        }
        pre {
            background: #f4f4f4;
            padding: 1rem;
            border-radius: 5px;
            overflow-x: auto;
            border-left: 4px solid #667eea;
            white-space: pre;
            font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
        }
        code {
            background: #f4f4f4;
            padding: 0.2rem 0.4rem;
            border-radius: 3px;
            font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
            display: block;
            line-height: 1em;
        }
        pre code {
            background: none;
            padding: 0;
            display: block;
            line-height: 1em;
        }
        .nav-back {
            display: inline-block;
            background: #667eea;
            color: white;
            padding: 0.5rem 1rem;
            text-decoration: none;
            border-radius: 5px;
            margin-bottom: 1rem;
        }
        .nav-back:hover {
            background: #5a67d8;
        }
        .github-corner {
            position: absolute;
            top: 0;
            right: 0;
            width: 80px;
            height: 80px;
            overflow: hidden;
        }
        .github-corner svg {
            fill: rgba(255, 255, 255, 0.9);
            color: #667eea;
            position: absolute;
            top: 0;
            border: 0;
            right: 0;
        }
        .github-corner:hover svg {
            fill: rgba(255, 255, 255, 1);
        }
        h1, h2, h3, h4, h5, h6 {
            color: #667eea;
        }
        h1 { font-size: 2rem; margin-top: 2rem; }
        h2 { font-size: 1.5rem; margin-top: 1.5rem; border-bottom: 2px solid #eee; padding-bottom: 0.5rem; }
        h3 { font-size: 1.3rem; margin-top: 1.3rem; }
        h4 { font-size: 1.1rem; margin-top: 1.1rem; }
        ul, ol { padding-left: 2rem; }
        blockquote {
            border-left: 4px solid #667eea;
            margin: 1rem 0;
            padding-left: 1rem;
            color: #666;
        }
    </style>
</head>
<body>
    <div class="header">
        <a href="https://github.com/abbychau/mpmc-std" class="github-corner" target="_blank" rel="noopener noreferrer">
            <svg xmlns="http://www.w3.org/2000/svg" width="80" height="80" viewBox="0 0 250 250" fill="#151513" style="position: absolute; top: 0; right: 0">
            <path d="M0 0l115 115h15l12 27 108 108V0z" fill="#000"/>
            <path class="octo-arm" d="M128 109c-15-9-9-19-9-19 3-7 2-11 2-11-1-7 3-2 3-2 4 5 2 11 2 11-3 10 5 15 9 16" style="-webkit-transform-origin: 130px 106px; transform-origin: 130px 106px"/>
            <path class="octo-body" d="M115 115s4 2 5 0l14-14c3-2 6-3 8-3-8-11-15-24 2-41 5-5 10-7 16-7 1-2 3-7 12-11 0 0 5 3 7 16 4 2 8 5 12 9s7 8 9 12c14 3 17 7 17 7-4 8-9 11-11 11 0 6-2 11-7 16-16 16-30 10-41 2 0 3-1 7-5 11l-12 11c-1 1 1 5 1 5z"/>
            </svg>
        </a>
        <h1>$title</h1>
        <p>$subtitle</p>
    </div>
    
    <a href="index.html" class="nav-back">← Back to Documentation Index</a>
    
    <div class="content">
EOF

    # Convert and append markdown content
    python3 /tmp/md_to_html.py "$md_file" >> "$html_file"
    
    # Close HTML
    cat >> "$html_file" << 'EOF'
    </div>
</body>
</html>
EOF
}

# Convert documentation files
if [ -f "docs/ALGORITHM_DIAGRAMS.md" ]; then
    create_html_doc "docs/ALGORITHM_DIAGRAMS.md" "docs/ALGORITHM_DIAGRAMS.html" "🎨 Algorithm Diagrams" "Visual explanations of the MPMC queue algorithm"
fi

if [ -f "docs/IMPLEMENTATION_NOTES.md" ]; then
    create_html_doc "docs/IMPLEMENTATION_NOTES.md" "docs/IMPLEMENTATION_NOTES.html" "🔬 Implementation Deep Dive" "Technical details and performance engineering"
fi

# Create README.html link (assumes README.md will be viewed as HTML)
if [ -f "README.md" ]; then
    cp README.md docs/README.html
fi

# Cleanup
rm -f /tmp/md_to_html.py

echo "✅ Documentation site created!"
echo ""
echo "📂 Generated files:"
echo "   - docs/index.html (main navigation)"
echo "   - docs/benchmarks/ (copied from target/criterion)"
echo "   - docs/ALGORITHM_DIAGRAMS.html"
echo "   - docs/IMPLEMENTATION_NOTES.html"
echo "   - docs/README.html"
echo ""
echo "🌐 Open docs/index.html in your browser to navigate the documentation"
echo "📊 All benchmark results are accessible through the navigation"