// RE_TRACKER Chart.js Configuration and Data Management

let housingChart = null;
let cachedData = null;

// Initialize the application when DOM is ready
document.addEventListener('DOMContentLoaded', function() {
    initializeChart();
    loadData();
    setupEventListeners();
});

// Setup event listeners for controls
function setupEventListeners() {
    document.getElementById('timeframeSelect').addEventListener('change', updateChart);
    document.getElementById('smaSelect').addEventListener('change', updateChart);
    document.getElementById('refreshBtn').addEventListener('click', refreshData);
}

// Initialize Chart.js with dual-axis configuration
function initializeChart() {
    const ctx = document.getElementById('housingChart').getContext('2d');
    
    housingChart = new Chart(ctx, {
        type: 'line',
        data: {
            labels: [],
            datasets: [
                {
                    label: 'Active Listings',
                    data: [],
                    borderColor: '#8ab4f8',
                    backgroundColor: 'rgba(138, 180, 248, 0.1)',
                    yAxisID: 'y-listings',
                    tension: 0.3,
                    borderWidth: 2,
                    pointRadius: 3,
                    pointHoverRadius: 5
                },
                {
                    label: 'Avg Price/SqFt',
                    data: [],
                    borderColor: '#81c995',
                    backgroundColor: 'rgba(129, 201, 149, 0.1)',
                    yAxisID: 'y-price',
                    tension: 0.3,
                    borderWidth: 2,
                    pointRadius: 3,
                    pointHoverRadius: 5
                }
            ]
        },
        options: {
            responsive: true,
            maintainAspectRatio: true,
            interaction: {
                mode: 'index',
                intersect: false,
            },
            plugins: {
                legend: {
                    display: true,
                    position: 'top',
                    labels: {
                        color: '#e8eaed',
                        font: {
                            size: 14
                        },
                        padding: 15
                    }
                },
                tooltip: {
                    backgroundColor: 'rgba(45, 49, 57, 0.95)',
                    titleColor: '#e8eaed',
                    bodyColor: '#e8eaed',
                    borderColor: '#3c4043',
                    borderWidth: 1,
                    padding: 12,
                    displayColors: true,
                    callbacks: {
                        label: function(context) {
                            let label = context.dataset.label || '';
                            if (label) {
                                label += ': ';
                            }
                            if (context.parsed.y !== null) {
                                if (context.dataset.yAxisID === 'y-price') {
                                    label += '$' + context.parsed.y.toFixed(2);
                                } else {
                                    label += context.parsed.y;
                                }
                            }
                            return label;
                        }
                    }
                }
            },
            scales: {
                x: {
                    grid: {
                        color: 'rgba(60, 64, 67, 0.5)'
                    },
                    ticks: {
                        color: '#9aa0a6',
                        font: {
                            size: 11
                        }
                    }
                },
                'y-listings': {
                    type: 'linear',
                    position: 'left',
                    title: {
                        display: true,
                        text: 'Active Listings',
                        color: '#8ab4f8',
                        font: {
                            size: 14,
                            weight: 'bold'
                        }
                    },
                    grid: {
                        color: 'rgba(60, 64, 67, 0.3)'
                    },
                    ticks: {
                        color: '#8ab4f8',
                        font: {
                            size: 11
                        }
                    }
                },
                'y-price': {
                    type: 'linear',
                    position: 'right',
                    title: {
                        display: true,
                        text: 'Price per SqFt ($)',
                        color: '#81c995',
                        font: {
                            size: 14,
                            weight: 'bold'
                        }
                    },
                    grid: {
                        drawOnChartArea: false
                    },
                    ticks: {
                        color: '#81c995',
                        font: {
                            size: 11
                        },
                        callback: function(value) {
                            return '$' + value.toFixed(0);
                        }
                    }
                }
            }
        }
    });
}

// Load data from the API endpoint
async function loadData() {
    try {
        // Fetch data from the backend (served as JSON)
        const response = await fetch('/api/data');
        
        if (!response.ok) {
            throw new Error('Failed to fetch data');
        }
        
        cachedData = await response.json();
        updateChart();
        updateStats();
        
    } catch (error) {
        console.error('Error loading data:', error);
        // Use demo data if fetch fails
        useDemoData();
    }
}

// Use demo data for development/demonstration
function useDemoData() {
    console.log('Using demo data');
    
    cachedData = [];
    const now = new Date();
    
    // Generate 90 days of demo data
    for (let i = 90; i >= 0; i--) {
        const date = new Date(now);
        date.setDate(date.getDate() - i);
        
        // Simulate market trends with some randomness
        const baseListings = 45 + Math.sin(i / 15) * 5 + (Math.random() - 0.5) * 3;
        const basePrice = 450 + (90 - i) * 0.5 + Math.sin(i / 10) * 10 + (Math.random() - 0.5) * 5;
        
        cachedData.push({
            date: date.toISOString().split('T')[0],
            active_listings: Math.round(baseListings),
            avg_price_per_sqft: parseFloat(basePrice.toFixed(2)),
            data_source: i > 30 ? 'historical' : 'scraped'
        });
    }
    
    updateChart();
    updateStats();
}

// Update the chart with filtered data
function updateChart() {
    if (!cachedData || cachedData.length === 0) return;
    
    const timeframe = document.getElementById('timeframeSelect').value;
    const smaPeriod = parseInt(document.getElementById('smaSelect').value);
    
    // Filter data by timeframe
    let filteredData = cachedData;
    if (timeframe !== 'all') {
        const days = parseInt(timeframe);
        filteredData = cachedData.slice(-days);
    }
    
    // Extract labels and data
    const labels = filteredData.map(d => formatDate(d.date));
    const listingsData = filteredData.map(d => d.active_listings);
    const priceData = filteredData.map(d => d.avg_price_per_sqft);
    
    // Update chart datasets
    housingChart.data.labels = labels;
    housingChart.data.datasets[0].data = listingsData;
    housingChart.data.datasets[1].data = priceData;
    
    // Apply SMA if selected
    if (smaPeriod > 0 && filteredData.length >= smaPeriod) {
        const priceSMA = calculateSMA(priceData, smaPeriod);
        
        // Add or update SMA dataset
        if (housingChart.data.datasets.length === 2) {
            housingChart.data.datasets.push({
                label: `${smaPeriod}-Day SMA`,
                data: priceSMA,
                borderColor: '#fdd663',
                backgroundColor: 'transparent',
                yAxisID: 'y-price',
                tension: 0.3,
                borderWidth: 2,
                borderDash: [5, 5],
                pointRadius: 0
            });
        } else {
            housingChart.data.datasets[2].data = priceSMA;
            housingChart.data.datasets[2].label = `${smaPeriod}-Day SMA`;
        }
    } else {
        // Remove SMA dataset if disabled
        if (housingChart.data.datasets.length > 2) {
            housingChart.data.datasets.pop();
        }
    }
    
    housingChart.update();
}

// Calculate Simple Moving Average
function calculateSMA(data, period) {
    const sma = [];
    for (let i = 0; i < data.length; i++) {
        if (i < period - 1) {
            sma.push(null); // Not enough data points
        } else {
            const sum = data.slice(i - period + 1, i + 1).reduce((a, b) => a + b, 0);
            sma.push(sum / period);
        }
    }
    return sma;
}

// Update statistics cards
function updateStats() {
    if (!cachedData || cachedData.length === 0) return;
    
    const latest = cachedData[cachedData.length - 1];
    const thirtyDaysAgo = cachedData[Math.max(0, cachedData.length - 31)];
    
    // Current values
    document.getElementById('currentListings').textContent = latest.active_listings;
    document.getElementById('currentPrice').textContent = '$' + latest.avg_price_per_sqft.toFixed(2);
    
    // Calculate 30-day change
    const priceChange = ((latest.avg_price_per_sqft - thirtyDaysAgo.avg_price_per_sqft) / thirtyDaysAgo.avg_price_per_sqft) * 100;
    const changeElement = document.getElementById('priceChange');
    changeElement.textContent = (priceChange >= 0 ? '+' : '') + priceChange.toFixed(1) + '%';
    changeElement.style.color = priceChange >= 0 ? '#81c995' : '#f28b82';
    
    // Last updated
    document.getElementById('lastUpdate').textContent = formatDate(latest.date);
}

// Format date for display
function formatDate(dateString) {
    const date = new Date(dateString);
    return date.toLocaleDateString('en-US', { month: 'short', day: 'numeric' });
}

// Refresh data from server
async function refreshData() {
    const btn = document.getElementById('refreshBtn');
    btn.disabled = true;
    btn.textContent = '⏳ Loading...';
    
    await loadData();
    
    btn.disabled = false;
    btn.textContent = '🔄 Refresh Data';
}

// Initialize with demo data
useDemoData();
