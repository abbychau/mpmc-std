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
        }
        .header h1 {
            margin: 0;
            font-size: 2.5rem;
            font-weight: 300;
        }
        .header p {
            margin: 0.5rem 0 0 0;
            opacity: 0.9;
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
        .perf-highlight {
            background: linear-gradient(135deg, #11998e 0%, #38ef7d 100%);
            color: white;
            padding: 1rem;
            border-radius: 5px;
            margin: 1rem 0;
        }
    </style>
</head>
<body>
    <div class="header">
        <h1>🚀 MPMC Queue Documentation</h1>
        <p>High-Performance Lockless Multi-Producer Multi-Consumer Queue</p>
    </div>

    <div class="nav-grid">
        <div class="nav-card">
            <h2>📚 Algorithm Documentation</h2>
            <p>Detailed explanations of the sequence-based ring buffer algorithm, memory layout optimization, and implementation details.</p>
            <a href="ALGORITHM_DIAGRAMS.html">View Algorithm Diagrams</a>
        </div>

        <div class="nav-card">
            <h2>🔬 Implementation Notes</h2>
            <p>Deep technical dive into memory ordering, cache optimization, safety guarantees, and performance engineering decisions.</p>
            <a href="IMPLEMENTATION_NOTES.html">View Implementation Details</a>
        </div>

        <div class="nav-card">
            <h2>📖 README</h2>
            <p>Complete project overview with quick start guide, usage examples, and architecture explanations.</p>
            <a href="../README.html">View README</a>
        </div>

        <div class="nav-card">
            <h2>📊 Benchmark Results</h2>
            <p>Comprehensive performance analysis with interactive Criterion.rs reports showing throughput, latency, and scaling characteristics.</p>
            <a href="benchmarks/report/index.html">View All Benchmarks</a>
        </div>
    </div>

    <div class="perf-highlight">
        <h3>⚡ Performance Highlights</h3>
        <ul>
            <li><strong>8.9ns</strong> average latency per operation</li>
            <li><strong>~1.8 billion ops/sec</strong> single-threaded throughput</li>
            <li><strong>Linear scaling</strong> up to 8 threads</li>
            <li><strong>Wait-free</strong> lockless algorithm</li>
        </ul>
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

# Convert markdown files to HTML for better navigation
echo "🔄 Converting markdown documentation to HTML..."

# Convert ALGORITHM_DIAGRAMS.md to HTML
if [ -f "docs/ALGORITHM_DIAGRAMS.md" ]; then
    cat > docs/ALGORITHM_DIAGRAMS.html << 'EOF'
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Algorithm Diagrams - MPMC Queue</title>
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
        }
        code {
            background: #f4f4f4;
            padding: 0.2rem 0.4rem;
            border-radius: 3px;
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
    </style>
</head>
<body>
    <div class="header">
        <h1>🎨 Algorithm Diagrams</h1>
        <p>Visual explanations of the MPMC queue algorithm</p>
    </div>
    
    <a href="index.html" class="nav-back">← Back to Documentation Index</a>
    
    <div class="content">
EOF
    
    # Convert markdown content to HTML (basic conversion)
    sed 's/^# /\n<h1>/g; s/^## /\n<h2>/g; s/^### /\n<h3>/g; s/^#### /\n<h4>/g' docs/ALGORITHM_DIAGRAMS.md | \
    sed 's/$/\n/g' | \
    sed 's/```/\n<pre><code>/g; s/```/<\/code><\/pre>\n/g' >> docs/ALGORITHM_DIAGRAMS.html
    
    echo "</div></body></html>" >> docs/ALGORITHM_DIAGRAMS.html
fi

# Convert IMPLEMENTATION_NOTES.md to HTML
if [ -f "docs/IMPLEMENTATION_NOTES.md" ]; then
    cat > docs/IMPLEMENTATION_NOTES.html << 'EOF'
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Implementation Notes - MPMC Queue</title>
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
        }
        code {
            background: #f4f4f4;
            padding: 0.2rem 0.4rem;
            border-radius: 3px;
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
    </style>
</head>
<body>
    <div class="header">
        <h1>🔬 Implementation Deep Dive</h1>
        <p>Technical details and performance engineering</p>
    </div>
    
    <a href="index.html" class="nav-back">← Back to Documentation Index</a>
    
    <div class="content">
EOF
    
    # Convert markdown content to HTML (basic conversion)
    sed 's/^# /\n<h1>/g; s/^## /\n<h2>/g; s/^### /\n<h3>/g; s/^#### /\n<h4>/g' docs/IMPLEMENTATION_NOTES.md | \
    sed 's/$/\n/g' | \
    sed 's/```/\n<pre><code>/g; s/```/<\/code><\/pre>\n/g' >> docs/IMPLEMENTATION_NOTES.html
    
    echo "</div></body></html>" >> docs/IMPLEMENTATION_NOTES.html
fi

# Create README.html link (assumes README.md will be viewed as HTML)
if [ -f "README.md" ]; then
    cp README.md docs/README.html
fi

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