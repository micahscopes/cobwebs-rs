use wasm_bindgen::prelude::*;
use serde::{Serialize,Deserialize};
use std::collections::HashMap;
use js_sys::Math::random;
use crate::tabu::{Possible};
use crate::pos;
use geo::{Coordinate};
use petgraph::stable_graph::{StableGraph};
use petgraph::graph::{NodeIndex, GraphIndex};

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct NodeData {
    pub id: usize,
    pub x: Option<f64>,
    pub y: Option<f64>,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct EdgeData {
  pub from: usize,
  pub to: usize,
}

#[derive(Serialize, Deserialize)]
pub struct GraphData {
  pub nodes: Vec<NodeData>,
  pub edges: Vec<EdgeData>,
}

#[wasm_bindgen]
pub struct GraphLayout {
  graph: StableGraph::<NodeData, ()>,
}

#[wasm_bindgen]
impl GraphLayout {
  #[wasm_bindgen(constructor)]
  pub fn new(data: &JsValue) -> GraphLayout {
    let data: GraphData = JsValue::into_serde(&data).unwrap();
    let mut idx_map = HashMap::new();
    let mut layout = GraphLayout{
      graph: StableGraph::<NodeData, ()>::new()
    };

    for node in data.nodes {
      let idx = layout.graph.add_node(node);
      idx_map.insert(node.id, idx);
    }

    for edge in data.edges {
      layout.graph.add_edge(
        idx_map[&edge.from],
        idx_map[&edge.to],
      ());
    }

    return layout;
  }
  pub fn node_positions(&self) -> JsValue {
    let result: Vec<NodeData> = self.graph.node_indices()
      .map(|node_idx| *&self.graph[node_idx])
      .collect();
    return JsValue::from_serde(&result).unwrap()
  }
  pub fn randomize_node_positions(&mut self) {
    fn random_drift() -> f64 {
      return 5.0*(random() - 0.5);
    }
    for node in self.graph.node_weights_mut() {
      match node.x.zip(node.y) {
        Some((x,y)) => {
          node.x = Some(x + random_drift());
          node.y = Some(y + random_drift());
        }
        None => {
          node.x = Some(random_drift());
          node.y = Some(random_drift());
        }
      }
    }
  }
}
