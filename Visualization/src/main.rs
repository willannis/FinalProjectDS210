use plotly::{Bar, Plot, Scatter}; // Import chart types and plotting functionality
use plotly::layout::{Layout, Axis}; // Import layout and axis customization tools

fn main() {
    //  Graph analysis

    // Define analysis metrics: labels and their corresponding values
    let graph_analysis_metrics = vec![
        ("Average Connections", 257.69),
        ("Person with Highest Degree", 1357.0),
        ("Average Shortest Path Length", 1.86),
    ];

    // Extract labels and values from the metrics tuple vector
    let metric_labels: Vec<&str> = graph_analysis_metrics.iter().map(|x| x.0).collect();
    let metric_values: Vec<f64> = graph_analysis_metrics.iter().map(|x| x.1).collect();

    // Create a bar chart from the metric data
    let bar = Bar::new(metric_labels.clone(), metric_values.clone());

    // Initialize the plot and add the bar chart as a trace
    let mut graph_analysis_fig = Plot::new();
    graph_analysis_fig.add_trace(bar);

    // Set layout: title and axis labels
    let layout = Layout::new()
        .title("Graph Analysis")                  // Main plot title
        .x_axis(Axis::new().title("Metric"))      // X-axis label
        .y_axis(Axis::new().title("Value"));      // Y-axis label

    graph_analysis_fig.set_layout(layout);        // Apply the layout to the figure
    graph_analysis_fig.show();                    // Display the plot in a web browser

    // --- HEALTH DATA SCATTER PLOT ---

    // Sample data: income levels and corresponding food security values
    let income = vec![3, 5, 1, 2, 2, 4, 5, 3, 4, 1, 5, 2, 2, 3, 4, 3, 5, 4, 1, 3, 4, 1];
    let food_security = vec![0, 0, 6, 1, 0, 1, 2, 1, 2, 1, 1, 6, 2, 2, 6, 0, 2, 1, 3, 3, 3, 2];

    // Create a scatter plot of income vs food security
    let scatter = Scatter::new(income, food_security)
        .mode(plotly::common::Mode::Markers)     // Use points (not lines)
        .name("Income vs Food Security");        // Legend label

    // Initialize the plot and add the scatter trace
    let mut health_fig = Plot::new();
    health_fig.add_trace(scatter);

    // Set layout for the scatter plot
    let layout2 = Layout::new()
        .title("Health Conditions by Income and Food Security") // Title
        .x_axis(Axis::new().title("Income"))                    // X-axis label
        .y_axis(Axis::new().title("Food Security"));            // Y-axis label

    health_fig.set_layout(layout2); // Apply layout
    health_fig.show();              // Show the plot
}