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
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Inter', sans-serif;
            line-height: 1.6;
            color: #1a202c;
            margin: 0;
            padding: 0;
            background: 
                linear-gradient(135deg, rgba(99, 102, 241, 0.03) 0%, rgba(139, 92, 246, 0.03) 100%),
                repeating-linear-gradient(
                    0deg,
                    transparent,
                    transparent 50px,
                    rgba(99, 102, 241, 0.02) 50px,
                    rgba(99, 102, 241, 0.02) 51px
                ),
                repeating-linear-gradient(
                    90deg,
                    transparent,
                    transparent 50px,
                    rgba(139, 92, 246, 0.02) 50px,
                    rgba(139, 92, 246, 0.02) 51px
                ),
                radial-gradient(circle at 20% 80%, rgba(99, 102, 241, 0.05) 0%, transparent 50%),
                radial-gradient(circle at 80% 20%, rgba(139, 92, 246, 0.05) 0%, transparent 50%),
                #fafbfc;
            min-height: 100vh;
            overflow-x: hidden;
        }
        .container {
            max-width: 1200px;
            margin: 0 auto;
            padding: 20px;
        }
        .header {
            background: linear-gradient(135deg, #6366f1 0%, #8b5cf6 50%, #a855f7 100%);
            color: white;
            padding: 3rem 2rem;
            border-radius: 16px;
            margin-bottom: 3rem;
            text-align: center;
            position: relative;
            box-shadow: 
                0 20px 25px -5px rgba(99, 102, 241, 0.1),
                0 10px 10px -5px rgba(99, 102, 241, 0.04);
            border: 1px solid rgba(255, 255, 255, 0.1);
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
            background: linear-gradient(145deg, #ffffff 0%, #f8fafc 100%);
            border-radius: 16px;
            padding: 2rem;
            box-shadow: 
                0 4px 6px -1px rgba(0, 0, 0, 0.1),
                0 2px 4px -1px rgba(0, 0, 0, 0.06);
            border: 1px solid rgba(99, 102, 241, 0.08);
            transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
            position: relative;
            overflow: hidden;
        }
        .nav-card::before {
            content: '';
            position: absolute;
            top: 0;
            left: 0;
            right: 0;
            height: 3px;
            background: linear-gradient(90deg, #6366f1, #8b5cf6, #a855f7);
            opacity: 0;
            transition: opacity 0.3s ease;
        }
        .nav-card:hover {
            transform: translateY(-4px);
            box-shadow: 
                0 20px 25px -5px rgba(99, 102, 241, 0.1),
                0 10px 10px -5px rgba(99, 102, 241, 0.04);
            border-color: rgba(99, 102, 241, 0.15);
        }
        .nav-card:hover::before {
            opacity: 1;
        }
        .nav-card h2 {
            margin-top: 0;
            color: #374151;
            font-size: 1.4rem;
            font-weight: 600;
            margin-bottom: 1rem;
        }
        .nav-card p {
            color: #6b7280;
            margin-bottom: 1.5rem;
            line-height: 1.6;
        }
        .nav-card a {
            display: inline-block;
            background: linear-gradient(135deg, #6366f1 0%, #8b5cf6 100%);
            color: white;
            padding: 0.75rem 1.5rem;
            text-decoration: none;
            border-radius: 12px;
            transition: all 0.2s ease;
            font-weight: 500;
            box-shadow: 0 2px 4px rgba(99, 102, 241, 0.2);
        }
        .nav-card a:hover {
            background: linear-gradient(135deg, #5855eb 0%, #7c3aed 100%);
            transform: translateY(-1px);
            box-shadow: 0 4px 8px rgba(99, 102, 241, 0.3);
        }
        .benchmark-section {
            background: linear-gradient(145deg, #ffffff 0%, #f8fafc 100%);
            border-radius: 20px;
            padding: 3rem;
            box-shadow: 
                0 10px 15px -3px rgba(0, 0, 0, 0.1),
                0 4px 6px -2px rgba(0, 0, 0, 0.05);
            border: 1px solid rgba(99, 102, 241, 0.06);
            margin-bottom: 3rem;
            position: relative;
            overflow: hidden;
        }
        .benchmark-section::before {
            content: '';
            position: absolute;
            top: 0;
            left: 0;
            right: 0;
            height: 4px;
            background: linear-gradient(90deg, #6366f1, #8b5cf6, #a855f7, #ec4899);
        }
        .benchmark-section h2 {
            color: #374151;
            border-bottom: 2px solid #e5e7eb;
            padding-bottom: 1rem;
            margin-bottom: 2rem;
            font-weight: 600;
            font-size: 1.75rem;
        }
        .benchmark-grid {
            display: grid;
            grid-template-columns: repeat(3, 1fr);
            gap: 1.5rem;
            margin-top: 2rem;
        }
        @media (max-width: 768px) {
            .benchmark-grid {
                grid-template-columns: 1fr;
            }
        }
        @media (max-width: 1024px) and (min-width: 769px) {
            .benchmark-grid {
                grid-template-columns: repeat(2, 1fr);
            }
        }
        .benchmark-item {
            background: linear-gradient(145deg, #ffffff 0%, #f9fafb 100%);
            padding: 1.75rem;
            border-radius: 16px;
            border: 1px solid rgba(99, 102, 241, 0.15);
            transition: all 0.3s ease;
            position: relative;
        }
        .benchmark-item:hover {
            transform: translateY(-2px);
            box-shadow: 
                0 10px 15px -3px rgba(99, 102, 241, 0.1),
                0 4px 6px -2px rgba(99, 102, 241, 0.05);
        }
        .benchmark-item h4 {
            margin-top: 0;
            margin-bottom: 1rem;
            color: #374151;
            font-weight: 600;
            font-size: 1.1rem;
        }
        .benchmark-item p {
            color: #6b7280;
            margin-bottom: 1.25rem;
            line-height: 1.5;
        }
        .benchmark-item a {
            color: #6366f1;
            text-decoration: none;
            font-weight: 500;
            padding: 0.5rem 1rem;
            border-radius: 8px;
            background: rgba(99, 102, 241, 0.05);
            transition: all 0.2s ease;
            display: inline-block;
        }
        .benchmark-item a:hover {
            background: rgba(99, 102, 241, 0.1);
            transform: translateX(2px);
        }
        .footer {
            background: linear-gradient(135deg, #1f2937 0%, #374151 100%);
            color: rgba(255, 255, 255, 0.9);
            text-align: center;
            padding: 4rem 2rem 3rem 2rem;
            margin-top: 4rem;
            border-top: 4px solid transparent;
            border-image: linear-gradient(90deg, #6366f1, #8b5cf6, #a855f7, #ec4899) 1;
            position: relative;
            width: 100%;
            box-sizing: border-box;
        }
        .footer::before {
            content: '';
            position: absolute;
            top: 0;
            left: 0;
            right: 0;
            height: 2px;
            background: linear-gradient(90deg, transparent, #6366f1, #8b5cf6, #a855f7, #ec4899, transparent);
        }
        .footer p {
            margin: 0.5rem 0;
            opacity: 0.8;
        }
        .footer .footer-brand {
            font-size: 1.1rem;
            font-weight: 500;
            opacity: 1;
            margin-bottom: 1rem;
        }
        
        /* Code styling */
        code {
            background: rgba(99, 102, 241, 0.08);
            color: #6366f1;
            padding: 0.2em 0.4em;
            border-radius: 6px;
            font-family: 'SF Mono', Monaco, 'Cascadia Code', 'Roboto Mono', Consolas, 'Courier New', monospace;
            font-size: 0.9em;
            font-weight: 500;
            border: 1px solid rgba(99, 102, 241, 0.12);
        }
        
        pre {
            background: linear-gradient(145deg, #f8fafc 0%, #f1f5f9 100%);
            border: 1px solid rgba(99, 102, 241, 0.1);
            border-radius: 12px;
            padding: 1.5rem;
            margin: 1.5rem 0;
            overflow-x: auto;
            box-shadow: 
                0 4px 6px -1px rgba(0, 0, 0, 0.05),
                0 2px 4px -1px rgba(0, 0, 0, 0.03);
        }
        
        pre code {
            background: none;
            color: #374151;
            padding: 0;
            border-radius: 0;
            border: none;
            font-size: 0.875rem;
            line-height: 1.6;
            font-weight: 400;
        }
        
        pre:hover {
            box-shadow: 
                0 10px 15px -3px rgba(99, 102, 241, 0.08),
                0 4px 6px -2px rgba(99, 102, 241, 0.04);
        }
    </style>
</head>
<body>
    <div class="container">
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
            <ul>
                <li><strong>8.9ns</strong> latency</li>
                <li><strong>1.8B ops/sec</strong> throughput</li>
                <li><strong>Linear scaling</strong> to 8 threads</li>
                <li><strong>Wait-free</strong> algorithm</li>
            </ul>
        </div>
    </div>

    <div class="benchmark-section">
        <h2>📈 Performance Analysis</h2>
        <p style="text-align: center; margin-bottom: 2rem; color: #666; font-size: 1.1rem;">
            Comprehensive benchmark suite showing throughput, latency, and scaling characteristics across all scenarios.
            <br><strong><a href="benchmarks/report/index.html" style="color: #667eea; text-decoration: none;">→ View Complete Benchmark Report</a></strong>
        </p>
        
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

    <div class="nav-grid">
        <div class="nav-card">
            <h2>📚 Technical Documentation</h2>
            <p>Complete technical reference covering algorithm design, implementation details, memory optimization, performance engineering, and comparative analysis with established research.</p>
            <div style="display: flex; gap: 1rem; margin-top: 1.5rem;">
                <a href="ALGORITHM_DIAGRAMS.html" style="flex: 1; text-align: center;">Algorithm Diagrams</a>
                <a href="IMPLEMENTATION_NOTES.html" style="flex: 1; text-align: center;">Implementation Deep Dive</a>
            </div>
        </div>
    </div>
    </div>

    <div class="footer">
        <div class="footer-brand">MPMC Queue - High-Performance Lockless Data Structure</div>
        <p>Built with Rust • Benchmarked with Criterion.rs • Optimized for Modern Hardware</p>
        <p style="font-size: 0.9rem; margin-top: 1rem;">
            Research-grade implementation combining Michael & Scott, LMAX Disruptor, and modern optimization techniques
        </p>
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
    
    for i, line in enumerate(lines):
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
        original_line = line
        
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
            
            # Handle empty lines
            if line.strip() == '':
                # Check surrounding lines for headings
                prev_is_heading = i > 0 and any(lines[i-1].startswith('#' * j + ' ') for j in range(1, 6))
                next_is_heading = i < len(lines) - 1 and any(lines[i+1].startswith('#' * j + ' ') for j in range(1, 6))
                
                if prev_is_heading or next_is_heading:
                    continue  # Skip empty lines around headings
                else:
                    line = '<br>'
        
        result.append(line)
    
    return '\n'.join(result)

if __name__ == '__main__':
    if len(sys.argv) != 2:
        print("Usage: python3 md_to_html.py <filename>")
        sys.exit(1)
    
    print(convert_markdown_to_html(sys.argv[1]))
EOF

# Function to create HTML file with proper structure and professional styling
create_html_doc() {
    local md_file="$1"
    local html_file="$2"
    local title="$3"
    local subtitle="$4"
    
    # Create HTML header with professional styling
    cat > "$html_file" << EOF
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>$title - MPMC Queue</title>
    <style>
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Inter', sans-serif;
            line-height: 1.6;
            color: #1a202c;
            margin: 0;
            padding: 0;
            background: 
                linear-gradient(135deg, rgba(99, 102, 241, 0.03) 0%, rgba(139, 92, 246, 0.03) 100%),
                repeating-linear-gradient(
                    0deg,
                    transparent,
                    transparent 50px,
                    rgba(99, 102, 241, 0.02) 50px,
                    rgba(99, 102, 241, 0.02) 51px
                ),
                repeating-linear-gradient(
                    90deg,
                    transparent,
                    transparent 50px,
                    rgba(139, 92, 246, 0.02) 50px,
                    rgba(139, 92, 246, 0.02) 51px
                ),
                radial-gradient(circle at 20% 80%, rgba(99, 102, 241, 0.05) 0%, transparent 50%),
                radial-gradient(circle at 80% 20%, rgba(139, 92, 246, 0.05) 0%, transparent 50%),
                #fafbfc;
            min-height: 100vh;
            overflow-x: hidden;
        }
        .container {
            max-width: 1000px;
            margin: 0 auto;
            padding: 20px;
        }
        .header {
            background: linear-gradient(135deg, #6366f1 0%, #8b5cf6 50%, #a855f7 100%);
            color: white;
            padding: 3rem 2rem;
            border-radius: 16px;
            margin-bottom: 3rem;
            text-align: center;
            position: relative;
            box-shadow: 
                0 20px 25px -5px rgba(99, 102, 241, 0.1),
                0 10px 10px -5px rgba(99, 102, 241, 0.04);
            border: 1px solid rgba(255, 255, 255, 0.1);
        }
        .content {
            background: linear-gradient(145deg, #ffffff 0%, #f8fafc 100%);
            padding: 3rem;
            border-radius: 20px;
            box-shadow: 
                0 10px 15px -3px rgba(0, 0, 0, 0.1),
                0 4px 6px -2px rgba(0, 0, 0, 0.05);
            border: 1px solid rgba(99, 102, 241, 0.06);
            margin-bottom: 3rem;
            position: relative;
            overflow: hidden;
        }
        .content::before {
            content: '';
            position: absolute;
            top: 0;
            left: 0;
            right: 0;
            height: 4px;
            background: linear-gradient(90deg, #6366f1, #8b5cf6, #a855f7, #ec4899);
        }
        .nav-back {
            display: inline-block;
            background: linear-gradient(135deg, #6366f1 0%, #8b5cf6 100%);
            color: white;
            padding: 0.75rem 1.5rem;
            text-decoration: none;
            border-radius: 12px;
            margin: 2rem 0;
            font-weight: 500;
            transition: all 0.2s ease;
            box-shadow: 0 2px 4px rgba(99, 102, 241, 0.2);
        }
        .nav-back:hover {
            background: linear-gradient(135deg, #5855eb 0%, #7c3aed 100%);
            transform: translateY(-1px);
            box-shadow: 0 4px 8px rgba(99, 102, 241, 0.3);
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
            color: #374151;
        }
        .header h1 {
            color: white;
        }
        h1 { font-size: 2rem; margin-top: 2rem; }
        h2 { font-size: 1.5rem; margin-top: 1.5rem; border-bottom: 2px solid #e5e7eb; padding-bottom: 0.5rem; }
        h3 { font-size: 1.3rem; margin-top: 1.3rem; }
        h4 { font-size: 1.1rem; margin-top: 1.1rem; }
        ul, ol { padding-left: 2rem; }
        blockquote {
            border-left: 4px solid #6366f1;
            margin: 1rem 0;
            padding-left: 1rem;
            color: #666;
        }
        .footer {
            background: linear-gradient(135deg, #1f2937 0%, #374151 100%);
            color: rgba(255, 255, 255, 0.9);
            text-align: center;
            padding: 4rem 2rem 3rem 2rem;
            margin-top: 4rem;
            border-top: 4px solid transparent;
            border-image: linear-gradient(90deg, #6366f1, #8b5cf6, #a855f7, #ec4899) 1;
            position: relative;
            width: 100%;
            box-sizing: border-box;
        }
        .footer::before {
            content: '';
            position: absolute;
            top: 0;
            left: 0;
            right: 0;
            height: 2px;
            background: linear-gradient(90deg, transparent, #6366f1, #8b5cf6, #a855f7, #ec4899, transparent);
        }
        .footer p {
            margin: 0.5rem 0;
            opacity: 0.8;
        }
        .footer .footer-brand {
            font-size: 1.1rem;
            font-weight: 500;
            opacity: 1;
            margin-bottom: 1rem;
        }
        
        /* Code styling */
        code {
            background: rgba(99, 102, 241, 0.08);
            color: #6366f1;
            padding: 0.2em 0.4em;
            border-radius: 6px;
            font-family: 'SF Mono', Monaco, 'Cascadia Code', 'Roboto Mono', Consolas, 'Courier New', monospace;
            font-size: 0.9em;
            font-weight: 500;
            border: 1px solid rgba(99, 102, 241, 0.12);
        }
        
        pre {
            background: linear-gradient(145deg, #f8fafc 0%, #f1f5f9 100%);
            border: 1px solid rgba(99, 102, 241, 0.1);
            border-radius: 12px;
            padding: 1.5rem;
            margin: 1.5rem 0;
            overflow-x: auto;
            box-shadow: 
                0 4px 6px -1px rgba(0, 0, 0, 0.05),
                0 2px 4px -1px rgba(0, 0, 0, 0.03);
        }
        
        pre code {
            background: none;
            color: #374151;
            padding: 0;
            border-radius: 0;
            border: none;
            font-size: 0.875rem;
            line-height: 1.6;
            font-weight: 400;
        }
        
        pre:hover {
            box-shadow: 
                0 10px 15px -3px rgba(99, 102, 241, 0.08),
                0 4px 6px -2px rgba(99, 102, 241, 0.04);
        }
    </style>
</head>
<body>
    <div class="container">
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
        
        <div class="content">
EOF

    # Convert and append markdown content
    python3 /tmp/md_to_html.py "$md_file" >> "$html_file"
    
    # Close content div and add navigation button
    cat >> "$html_file" << 'EOF'
        </div>
        
        <div style="text-align: center; margin-bottom: 2rem;">
            <a href="index.html" class="nav-back">← Back to Documentation Index</a>
        </div>
    </div>

    <div class="footer">
        <div class="footer-brand">MPMC Queue - High-Performance Lockless Data Structure</div>
        <p>Built with Rust • Benchmarked with Criterion.rs • Optimized for Modern Hardware</p>
        <p style="font-size: 0.9rem; margin-top: 1rem;">
            Research-grade implementation combining Michael & Scott, LMAX Disruptor, and modern optimization techniques
        </p>
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