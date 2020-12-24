use super::{EdgeGeo, NodeGeo};
use bimap::{BiMap, Overwritten};
use geo::Point;
use petgraph::graph::{EdgeIndex, NodeIndex};
use rstar::{RTree, RTreeObject, AABB};
use std::iter::Iterator;

#[derive(Clone, Copy, Debug)]
pub enum GraphGeoTreeObject {
    Node(NodeIndex, NodeGeo),
    Edge(EdgeIndex, EdgeGeo),
}

use GraphGeoTreeObject::Edge;
use GraphGeoTreeObject::Node;
pub type Envelope = AABB<Point<f64>>;
impl PartialEq for GraphGeoTreeObject {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Node(x, _), Node(y, _)) => x == y,
            (Edge(x, _), Edge(y, _)) => x == y,
            _ => false,
        }
    }
}

impl Eq for GraphGeoTreeObject {}

use std::hash::{Hash, Hasher};
impl Hash for GraphGeoTreeObject {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Node(x, _) => x.hash(state),
            Edge(x, _) => x.hash(state),
        }
    }
}

pub struct GraphGeoTree {
    pub rtree: RTree<GraphGeoTreeObject>,
    pub nodes: BiMap<NodeIndex, GraphGeoTreeObject>,
    pub edges: BiMap<EdgeIndex, GraphGeoTreeObject>,
}

impl GraphGeoTree {
    pub fn new() -> Self {
        GraphGeoTree {
            rtree: RTree::<GraphGeoTreeObject>::new(),
            nodes: BiMap::new(),
            edges: BiMap::new(),
        }
    }

    pub fn insert_node(
        &mut self,
        idx: NodeIndex,
        geo: NodeGeo,
    ) -> bimap::Overwritten<NodeIndex, GraphGeoTreeObject> {
        // info!("added node to tree: ({},{})", geo.x, geo.y);
        let fresh = Node(idx, geo);
        let result = self.nodes.insert(idx, fresh);
        match result {
            Overwritten::Left(_, stale)
            | Overwritten::Pair(_, stale)
            | Overwritten::Both((_, stale), _) => {
                self.rtree.remove(&stale);
            }
            _ => {}
        };
        self.rtree.insert(fresh);
        return result;
    }

    pub fn insert_edge(
        &mut self,
        idx: EdgeIndex,
        geo: EdgeGeo,
    ) -> bimap::Overwritten<EdgeIndex, GraphGeoTreeObject> {
        // info!(
        //     "added edge to tree: ({}, {}) to ({},{})",
        //     geo.line.start.x, geo.line.start.x, geo.line.end.x, geo.line.end.y
        // );
        let fresh = Edge(idx, geo);
        let result = self.edges.insert(idx, fresh);
        match result {
            Overwritten::Left(_, stale)
            | Overwritten::Pair(_, stale)
            | Overwritten::Both((_, stale), _) => {
                self.rtree.remove(&stale);
            }
            _ => {}
        };
        self.rtree.insert(fresh);
        return result;
    }

    pub fn remove_node(
        &mut self,
        idx: NodeIndex,
    ) -> Option<GraphGeoTreeObject> {
        let removed = match self.nodes.get_by_left(&idx) {
            Some(node) => self.rtree.remove(node),
            _ => None,
        };
        return self.nodes.remove_by_left(&idx).map(|(_, x)| x);
    }

    pub fn remove_edge(
        &mut self,
        idx: EdgeIndex,
    ) -> Option<GraphGeoTreeObject> {
        let removed = match self.edges.get_by_left(&idx) {
            Some(edge) => self.rtree.remove(edge),
            _ => None,
        };
        return self.edges.remove_by_left(&idx).map(|(_, x)| x);
    }

    pub fn locate_in_envelope(
        &self,
        envelope: &Envelope,
    ) -> impl Iterator<Item = &GraphGeoTreeObject> {
        return self.rtree.locate_in_envelope(envelope);
    }

    pub fn edges_in_envelope(
        &self,
        envelope: &Envelope,
    ) -> impl Iterator<Item = (&EdgeIndex, &EdgeGeo)> {
        return self.locate_in_envelope(envelope).filter_map(
            move |obj| match obj {
                Edge(idx, geo) => Some((idx, geo)),
                _ => None,
            },
        );
    }

    pub fn nodes_in_envelope(
        &self,
        envelope: &Envelope,
    ) -> impl Iterator<Item = (&NodeIndex, &NodeGeo)> {
        return self.locate_in_envelope(envelope).filter_map(
            move |obj| match obj {
                Node(idx, geo) => Some((idx, geo)),
                _ => None,
            },
        );
    }
}

impl RTreeObject for EdgeGeo {
    type Envelope = Envelope;
    fn envelope(&self) -> Self::Envelope {
        return self.line.envelope();
    }
}

impl RTreeObject for GraphGeoTreeObject {
    type Envelope = Envelope;
    fn envelope(&self) -> Self::Envelope {
        match self {
            GraphGeoTreeObject::Node(_, node) => {
                let envelope = Point::<f64>::from(*node).envelope();
                // info!("node_envelope");
                return envelope;
            }
            GraphGeoTreeObject::Edge(_, edge) => {
                let envelope = edge.envelope();
                // info!("edge_envelope: {},{}...{},{}", envelope.lower().x(), envelope.lower().y(), envelope.upper().x(), envelope.upper().y() );
                return envelope;
            }
        }
    }
}
