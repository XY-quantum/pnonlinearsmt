<script src="https://d3js.org/d3.v7.min.js"></script>
<script>
const width = 800, height = 600;

d3.json("membrane_graph_dynamic.json").then(graph => {
    const svg = d3.select("body").append("svg").attr("width", width).attr("height", height);

    const simulation = d3.forceSimulation(graph.nodes)
        .force("link", d3.forceLink(graph.links).id(d => d.id).distance(50))
        .force("charge", d3.forceManyBody().strength(-50))
        .force("center", d3.forceCenter(width/2, height/2));

    const link = svg.selectAll("line")
        .data(graph.links)
        .enter().append("line")
        .attr("stroke", "#999");

    const node = svg.selectAll("circle")
        .data(graph.nodes)
        .enter().append("circle")
        .attr("r", 10)
        .attr("fill", d => d.alive ? "green" : "red")
        .call(drag(simulation));

    const label = svg.selectAll("text")
        .data(graph.nodes)
        .enter().append("text")
        .text(d => d.id)
        .attr("font-size", "10px");

    simulation.on("tick", () => {
        link
            .attr("x1", d => d.source.x)
            .attr("y1", d => d.source.y)
            .attr("x2", d => d.target.x)
            .attr("y2", d => d.target.y);

        node
            .attr("cx", d => d.x)
            .attr("cy", d => d.y);

        label
            .attr("x", d => d.x + 12)
            .attr("y", d => d.y + 4);
    });

    function drag(simulation) {
        function dragstarted(event, d) {
            if (!event.active) simulation.alphaTarget(0.3).restart();
            d.fx = d.x;
            d.fy = d.y;
        }

        function dragged(event, d) {
            d.fx = event.x;
            d.fy = event.y;
        }

        function dragended(event, d) {
            if (!event.active) simulation.alphaTarget(0);
            d.fx = null;
            d.fy = null;
        }

        return d3.drag()
            .on("start", dragstarted)
            .on("drag", dragged)
            .on("end", dragended);
    }
});
</script>
