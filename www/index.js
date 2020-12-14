import { Network } from "vis-network";
import { DataSet } from "vis-data";
// create an array with nodes
var nodes = new DataSet([
  { id: 1, label: "A", x: 200, y: 200 },
  { id: 2, label: "B", x: 0, y: 0 },
  { id: 3, label: "C", x: 0, y: 400 },
  { id: 4, label: "D", x: 400, y: 400 },
  { id: 5, label: "E", x: 400, y: 0 },
]);

// create an array with edges
var edges = new DataSet([
  { from: 1, to: 2, label: "A to B" },
  { from: 1, to: 3, label: "A to C" },
  { from: 1, to: 4, label: "A to D" },
  { from: 1, to: 5, label: "A to E" },
]);

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
  // width: width + 'px',
  // height: height + 'px',
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

// Set the coordinate system of Network such that it exactly
// matches the actual pixels of the HTML canvas on screen
// this must correspond with the width and height set for
// the networks container element.
network.moveTo({
  position: { x: 0, y: 0 },
  offset: { x: -width / 2, y: -height / 2 },
  scale: 1,
});
import init, { position_update } from './pkg/cobwebs_rs';

async function run() {
  await init();


  const step = () => {
    nodes.update([
      position_update(Date.now()/100),
      {
        id: 2,
        x: 100 * Math.tan(Date.now() / 531 / 2 - width / 3),
        y: 100 * Math.sin(Date.now() / 1000),
      }
    ]);
    window.requestAnimationFrame(step);
  };
  window.requestAnimationFrame(step);
}

run();
