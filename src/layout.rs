use crate::geometry::tree::GraphGeo;
use crate::geometry::tree::GraphGeoElement;
use crate::geometry::NodeGeo;
use crate::geometry::{Edge, Node};
use arraystring::{typenum::U64, ArrayString};
use bimap::BiMap;
use geo::algorithm::euclidean_distance::EuclideanDistance;
use geo::prelude::Intersects;
use geo::{Coordinate, Point};
use im::HashMap;
use itertools::Itertools;
use js_sys::Math::random;
use log::info;
use log::Level;
use petgraph::graph::NodeIndex;
use petgraph::stable_graph::StableGraph;
use rstar::AABB;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

pub type NodeDataId = ArrayString<U64>;

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct NodeData {
    pub id: ArrayString<U64>,
    pub x: Option<f64>,
    pub y: Option<f64>,
}

impl NodeData {
    pub fn coordinate(&self) -> Option<NodeGeo> {
        return Some(self.into());
    }
    pub fn point(&self) -> Option<Point<f64>> {
        let geo: NodeGeo = self.into();
        return Some(geo.into());
    }
}

impl Into<NodeGeo> for &NodeData {
    fn into(self) -> NodeGeo {
        return NodeGeo {
            x: self.x.unwrap_or_default(),
            y: self.y.unwrap_or_default(),
        };
    }
}

impl Into<NodeGeo> for NodeData {
    fn into(self) -> NodeGeo {
        return NodeGeo {
            x: self.x.unwrap_or_default(),
            y: self.y.unwrap_or_default(),
        };
    }
}

impl Into<Point<f64>> for &NodeData {
    fn into(self) -> Point<f64> {
        let geo: NodeGeo = self.into();
        return geo.into();
    }
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct EdgeData {
    pub from: ArrayString<U64>,
    pub to: ArrayString<U64>,
}

#[derive(Serialize, Deserialize)]
pub struct GraphData {
    pub nodes: Vec<NodeData>,
    pub edges: Vec<EdgeData>,
}

#[wasm_bindgen]
pub struct GraphLayout {
    #[wasm_bindgen(skip)]
    pub graph: StableGraph<NodeData, ()>,
    #[wasm_bindgen(skip)]
    pub graph_geo: GraphGeo,
    node_id_to_graph_index: BiMap<NodeDataId, NodeIndex>,
}

impl GraphLayout {
    pub fn node_data(&self, idx: NodeIndex) -> Option<NodeData> {
        let node_geo = self.node_geo(idx);
        return match node_geo {
            Some(geo) => Some(NodeData {
                x: Some(geo.x),
                y: Some(geo.y),
                id: *self.node_id_to_graph_index.get_by_right(&idx).unwrap(),
            }),
            _ => None,
        };
    }
}

#[wasm_bindgen]
impl GraphLayout {
    #[wasm_bindgen(constructor)]
    pub fn new(nodes: JsValue, edges: JsValue) -> GraphLayout {
        console_log::init_with_level(Level::Debug);
        let mut layout = GraphLayout {
            graph: StableGraph::<NodeData, ()>::new(),
            node_id_to_graph_index: BiMap::new(),
            graph_geo: GraphGeo::new(),
        };

        let nodes: Result<Vec<NodeData>, _> = JsValue::into_serde(&nodes);

        match nodes {
            Ok(nodes_data) => {
                for node_data in nodes_data {
                    layout.add_node_data(node_data)
                }
            }
            Err(_) => {
                // todo: logging
            }
        };

        let edges: Result<Vec<EdgeData>, _> = JsValue::into_serde(&edges);

        match edges {
            Ok(edges_data) => {
                for edge_data in edges_data {
                    layout.add_edge_data(edge_data)
                }
            }
            Err(_) => {
                // todo: logging
            }
        };

        return layout;
    }

    fn add_node_data(&mut self, node_data: NodeData) {
        let idx = self.graph.add_node(node_data);
        self.node_id_to_graph_index.insert(node_data.id, idx);
        let node_geo: NodeGeo = node_data.into();
        self.set_node_geo(idx, node_geo);
    }

    fn add_edge_data(&mut self, edge: EdgeData) {
        self.graph.add_edge(
            *self.node_id_to_graph_index.get_by_left(&edge.from).unwrap(),
            *self.node_id_to_graph_index.get_by_left(&edge.to).unwrap(),
            (),
        );
    }

    pub fn tree_facts(&self) {
        let tree = &self.graph_geo;
        info!(
            "nodes: {}, edges: {} ",
            tree.nodes.iter().count(),
            tree.edges.iter().count()
        );
    }

    pub fn inside_box(&self, size: f64, log_info: Option<bool>) -> Vec<usize> {
        let envelope = AABB::from_corners(
            Point::from(Coordinate {
                x: -size / 2.0,
                y: -size / 2.0,
            }),
            Point::from(Coordinate {
                x: size / 2.0,
                y: size / 2.0,
            }),
        );

        let in_envelope = self.graph_geo.rtree.locate_in_envelope(&envelope);

        let mut num_edges = 0;
        let mut num_nodes = 0;
        for geo in in_envelope {
            match geo {
                GraphGeoElement::Edge(_, _) => num_edges += 1,
                _ => num_nodes += 1,
            }
        }
        if log_info.unwrap_or_default() {
            info!(
                "Unit rect({}): {} nodes and {} edges in there",
                size, num_nodes, num_edges
            );
        }
        return vec![num_nodes, num_edges];
    }

    pub fn sum_of_charges(&self) -> f64 {
        self.graph
            .node_indices()
            .map(|n| self.graph[n])
            .into_iter()
            .combinations(2)
            .map(|c| Some(c[0].point()?.euclidean_distance(&c[1].point()?)))
            .filter_map(|d| d)
            .map(|d| 1.0 / d.powf(2.0))
            .sum()
    }

    pub fn count_edges_intersections(&self, log_info: Option<bool>) -> usize {
        let count = self
            .graph
            .edge_indices()
            .into_iter()
            .filter(|edge| self.count_edge_intersections(*edge) > 0)
            .count();

        if log_info.unwrap_or_default() {
            info!("All edges' intersections: {}", count);
        }

        return count;
    }

    pub fn count_graph_intersections(&self, log_info: Option<bool>) -> usize {
        let count = self
            .edges_geo()
            .iter()
            .combinations(2)
            .map(|c| if c[0].intersects(c[1]) { 1 } else { 0 })
            .sum();

        if log_info.unwrap_or_default() {
            info!("Graph intersections: {}", count);
        }

        return count;
    }

    pub fn nodes_data(&self) -> JsValue {
        let result: Vec<NodeData> = self
            .graph
            .node_indices()
            .map(|idx| self.node_data(idx))
            .into_iter()
            .flatten()
            .collect();
        JsValue::from_serde(&result).unwrap()
    }

    pub fn randomize_node_positions(&mut self, amount: f64) {
        let random_drift = || amount * (random() - 0.5);
        let node_indices: Vec<NodeIndex> = self.graph.node_indices().collect();

        for idx in node_indices.into_iter() {
            match self.graph_geo.nodes.clone().get_by_left(&idx) {
                Some(Node(_, pos)) => self.set_node_geo(
                    idx,
                    Coordinate {
                        x: pos.x + random_drift(),
                        y: pos.y + random_drift(),
                    },
                ),
                _ => self.set_node_geo(
                    idx,
                    Coordinate {
                        x: random_drift(),
                        y: random_drift(),
                    },
                ),
            }
        }
    }
}
