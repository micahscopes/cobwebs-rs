import { Network } from "vis-network";
import { DataSet } from "vis-data";
import graphData from "./graph-data.json"
var nodes = new DataSet(graphData.nodes);
var edges = new DataSet(graphData.edges);

// create a network
var container = document.getElementById("graph");
console.log("container", container);
var data = {
  nodes: nodes,
  edges: edges,
};
var width = 400;
var height = 400;
var options = {
  nodes: {
    shape: "dot",
  },
  edges: {
    smooth: false,
  },
  physics: false,
  interaction: {
    dragNodes: true, // do not allow dragging nodes
    zoomView: true, // do not allow zooming
    dragView: true, // do not allow dragging
  },
};
var network = new Network(container, data, options);
window.network = network;

// Set the coordinate system of Network such that it exactly
// matches the actual pixels of the HTML canvas on screen
// this must correspond with the width and height set for
// the networks container element.
network.moveTo({
  position: { x: 0, y: 0 },
  offset: { x: -width / 2, y: -height / 2 },
  scale: 1,
});

nodes.forEach(node => {
  let positions = network.getPositions(node.id);
  node.x = positions[node.id].x;
  node.y = positions[node.id].y;
})

import init, { GraphLayout } from './pkg/cobwebs_rs';

async function run() {
  await init();
  let layout = new GraphLayout(graphData.nodes, graphData.edges);
  const peak = 50
  let initialTime = null
  const step = () => {
    initialTime ||= Date.now()
    // layout.randomize_node_positions(peak*Math.sin((Date.now() - initialTime)/4000)**2);
    let i = 0
    while (i < 50) {
      // layout.inside_box(250, true);
      i=i+1
    }
    // for(let j=0; j<50; j++) {
    //   layout.count_graph_intersections(true);
      // layout.count_edges_intersections(false);
    // }

    // layout.tree_facts();
    nodes.update([
      ...layout.nodes_data(),
      {
        id: 2,
        x: 500 * Math.cos(Date.now() / 1000),
        y: 500 * Math.sin(Date.now() / 1000),
      },
      {
        id: 3,
        x: 200 * Math.cos(Date.now() / 500)+150,
        y: 200 * Math.sin(Date.now() / 500)+300,
      },
      {
        id: 4,
        x: 200 * Math.cos(Date.now() / 2000)-100,
        y: 200 * Math.sin(Date.now() / 2000)-200,
      },
    ])
    setTimeout(() => window.requestAnimationFrame(step), 1000/60)
  };
  setTimeout(() => window.requestAnimationFrame(step), 3000)
}

run()