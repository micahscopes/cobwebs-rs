use bimap::BiMap;
use fixed::types::I32F32;
use geo::algorithm::intersects::Intersects;
use geo::{Coordinate, Line};
use rstar::RTree;
pub mod tree;
use log::info;
use num_traits::pow::Pow;
use petgraph::visit::EdgeRef;

type Fixed = I32F32;
type EdgeLine = Line<f64>;
pub type NodeGeo = Coordinate<f64>;

fn to_grid(x: f64, grid_power: isize) -> Fixed {
    let grid_factor = 2.0f64.pow(grid_power as f64);
    return Fixed::from_num((x * grid_factor).round() / grid_factor);
}

pub fn quantize(n: NodeGeo, grid_power: isize) -> (Fixed, Fixed) {
    return (to_grid(n.x, grid_power), to_grid(n.y, grid_power));
}

#[derive(Clone, Copy, Debug)]
pub struct EdgeGeo {
    line: EdgeLine,
}

impl EdgeGeo {
    pub fn to_grid(&self, grid_power: isize) -> EdgeGeo {
        let mut fixed_edge = self.clone();
        match fixed_edge.quantized_coordinates(grid_power) {
            ((x1, y1), (x2, y2)) => {
                fixed_edge.line.start.x = Fixed::to_num::<f64>(x1);
                fixed_edge.line.start.y = Fixed::to_num::<f64>(y1);
                fixed_edge.line.end.x = Fixed::to_num::<f64>(x2);
                fixed_edge.line.end.y = Fixed::to_num::<f64>(y2);
            }
        }
        return fixed_edge;
    }
    pub fn quantized_coordinates(
        &self,
        grid_power: isize,
    ) -> ((Fixed, Fixed), (Fixed, Fixed)) {
        return (
            quantize(self.line.start, grid_power),
            quantize(self.line.end, grid_power),
        );
    }
    pub fn new(a: NodeGeo, b: NodeGeo) -> EdgeGeo {
        EdgeGeo {
            line: EdgeLine::new(a, b),
        }
    }
}

impl Intersects for EdgeGeo {
    fn intersects(&self, other: &Self) -> bool {
        return self.line.intersects(&other.line);
    }
}

impl PartialEq for EdgeGeo {
    fn eq(&self, other: &Self) -> bool {
        return self.quantized_coordinates(0) == other.quantized_coordinates(0);
    }
}

#[derive(Clone, Copy, Debug)]
pub enum GraphGeoElement {
    Node(NodeIndex, NodeGeo),
    Edge(EdgeIndex, EdgeGeo),
}

impl Into<Option<EdgeIndex>> for &GraphGeoElement {
    fn into(self) -> Option<EdgeIndex> {
        match self {
            Edge(idx, _) => Some(*idx),
            _ => None,
        }
    }
}

impl Into<Option<NodeIndex>> for &GraphGeoElement {
    fn into(self) -> Option<NodeIndex> {
        match self {
            Node(idx, _) => Some(*idx),
            _ => None,
        }
    }
}

impl Into<Option<EdgeGeo>> for &GraphGeoElement {
    fn into(self) -> Option<EdgeGeo> {
        match self {
            Edge(_, geo) => Some(*geo),
            _ => None,
        }
    }
}

impl Into<Option<NodeGeo>> for &GraphGeoElement {
    fn into(self) -> Option<NodeGeo> {
        match self {
            Node(_, geo) => Some(*geo),
            _ => None,
        }
    }
}

pub use GraphGeoElement::Edge;
pub use GraphGeoElement::Node;

pub struct GraphGeo {
    pub rtree: RTree<GraphGeoElement>,
    pub nodes: BiMap<NodeIndex, GraphGeoElement>,
    pub edges: BiMap<EdgeIndex, GraphGeoElement>,
}

use crate::layout::{GraphLayout, NodeData};
use petgraph::graph::{EdgeIndex, NodeIndex};

impl GraphLayout {
    pub fn edge_geo(&self, idx: EdgeIndex) -> Option<EdgeGeo> {
        let endpoints_geo = |(a, b): (NodeIndex, NodeIndex)| {
            Some((self.node_geo(a), self.node_geo(b)))
        };

        let edge_geo = |(a, b): (Option<&NodeGeo>, Option<&NodeGeo>)| {
            Some(EdgeGeo::new(*a?, *b?))
        };

        return self
            .graph
            .edge_endpoints(idx)
            .and_then(endpoints_geo)
            .and_then(edge_geo);
    }

    pub fn edges_geo(&self) -> Vec<EdgeGeo> {
        return self
            .graph
            .edge_indices()
            .filter_map(|e| self.edge_geo(e))
            .collect();
    }

    fn update_graph_geo_tree_for_nodes(&mut self, nodes_idx: Vec<NodeIndex>) {
        for idx in nodes_idx.iter() {
            self.graph_geo.remove_node(*idx);
            match self.graph_geo.nodes.clone().get_by_left(&idx) {
                Some(Node(_, node_geo)) => {
                    self.graph_geo.insert_node(*idx, *node_geo);
                }
                _ => {}
            };
        }
        let associated_edges: Vec<EdgeIndex> = nodes_idx
            .iter()
            .flat_map(|idx| self.graph.edges(*idx))
            .map(|r| r.id())
            .collect();
        // info!(
        //     "preparing to update with {} edges",
        //     associated_edges.iter().count()
        // );
        for idx in associated_edges {
            self.graph_geo.remove_edge(idx);
            match self.edge_geo(idx) {
                Some(edge_geo) => {
                    // info!("found edge_geo! {}", edge_geo.line.start.x);
                    self.graph_geo.insert_edge(idx, edge_geo);
                }
                None => info!("failed to find that edge!"),
            };
        }
        return ();
    }

    fn update_graph_geo_tree_for_node(&mut self, idx: NodeIndex) {
        return self.update_graph_geo_tree_for_nodes(vec![idx]);
    }

    pub fn node_geo(&self, idx: NodeIndex) -> Option<&NodeGeo> {
        match self.graph_geo.nodes.get_by_left(&idx) {
            Some(Node(_, geo)) => Some(&geo),
            _ => None,
        }
    }

    pub fn set_node_geo(&mut self, idx: NodeIndex, position: NodeGeo) {
        // info!("position: {}, {}", position.x, position.y);
        // info!("set_node_geo before: {}", before.is_some());
        self.graph_geo.nodes.insert(idx, Node(idx, position));
        self.update_graph_geo_tree_for_node(idx);
        // info!("set_node_geo after: {}", after.unwrap().x);
    }
}
