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
    <link rel="stylesheet" href="shared-styles.css">
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
        <p style="text-align: center; margin-bottom: 2rem; color: #475569; font-size: 1.1rem;">
            Comprehensive benchmark suite showing throughput, latency, and scaling characteristics across all scenarios.
            <br><strong><a href="benchmarks/report/index.html" style="color: #1e40af; text-decoration: none; padding: 0.5rem 1rem; background: linear-gradient(135deg, rgba(30, 64, 175, 0.06) 0%, rgba(30, 64, 175, 0.08) 100%); border-radius: 6px; margin-top: 0.5rem; display: inline-block; font-weight: 600; font-size: 0.9rem; border: 1px solid rgba(30, 64, 175, 0.12); transition: all 0.25s ease;">→ View Complete Benchmark Report</a></strong>
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

# Check if Node.js is available
if ! command -v node &> /dev/null; then
    echo "❌ Error: Node.js is required for markdown conversion. Please install Node.js."
    exit 1
fi

# Ensure the markdown converter script exists
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
MARKDOWN_CONVERTER="$SCRIPT_DIR/scripts/markdown-converter.js"

if [ ! -f "$MARKDOWN_CONVERTER" ]; then
    echo "❌ Error: Markdown converter script not found at $MARKDOWN_CONVERTER"
    exit 1
fi

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
    <link rel="stylesheet" href="shared-styles.css">

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
            <div style="text-align: center; margin-bottom: 2rem;">
                <a href="index.html" class="nav-back">← Back to Documentation Index</a>
            </div>
EOF

    # Convert and append markdown content
    node "$MARKDOWN_CONVERTER" "$md_file" >> "$html_file"
    
    # Close content div and add navigation section
    cat >> "$html_file" << 'EOF'
        </div>
        
        <div class="nav-grid">
            <div class="nav-card">
                <h2>📚 Related Documentation</h2>
                <p>Explore the complete technical documentation suite for the MPMC Queue implementation.</p>
                <div style="display: flex; gap: 1rem; margin-top: 1.5rem;">
                    <a href="index.html" style="flex: 1; text-align: center;">Documentation Index</a>
                    <a href="../benchmarks/report/index.html" style="flex: 1; text-align: center;">Benchmark Results</a>
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

# No cleanup needed - using standalone JS script

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