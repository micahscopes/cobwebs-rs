use super::{EdgeGeo, NodeGeo};
pub use super::{GraphGeo, GraphGeoElement};
use bimap::{BiMap, Overwritten};
use geo::Point;
use petgraph::graph::{EdgeIndex, NodeIndex};
use rstar::{RTree, RTreeObject, AABB};
use std::iter::Iterator;

pub use GraphGeoElement::Edge;
pub use GraphGeoElement::Node;

pub type Envelope = AABB<Point<f64>>;

impl PartialEq for GraphGeoElement {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Node(x, _), Node(y, _)) => x == y,
            (Edge(x, _), Edge(y, _)) => x == y,
            _ => false,
        }
    }
}

impl Eq for GraphGeoElement {}

use std::hash::{Hash, Hasher};
impl Hash for GraphGeoElement {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Node(x, _) => x.hash(state),
            Edge(x, _) => x.hash(state),
        }
    }
}

impl GraphGeo {
    pub fn new() -> Self {
        GraphGeo {
            rtree: RTree::<GraphGeoElement>::new(),
            nodes: BiMap::new(),
            edges: BiMap::new(),
        }
    }

    pub fn insert_node(
        &mut self,
        idx: NodeIndex,
        geo: NodeGeo,
    ) -> bimap::Overwritten<NodeIndex, GraphGeoElement> {
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
    ) -> bimap::Overwritten<EdgeIndex, GraphGeoElement> {
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

    pub fn remove_node(&mut self, idx: NodeIndex) -> Option<GraphGeoElement> {
        let removed = match self.nodes.get_by_left(&idx) {
            Some(node) => self.rtree.remove(node),
            _ => None,
        };
        return self.nodes.remove_by_left(&idx).map(|(_, x)| x);
    }

    pub fn remove_edge(&mut self, idx: EdgeIndex) -> Option<GraphGeoElement> {
        let removed = match self.edges.get_by_left(&idx) {
            Some(edge) => self.rtree.remove(edge),
            _ => None,
        };
        return self.edges.remove_by_left(&idx).map(|(_, x)| x);
    }

    pub fn locate_in_envelope(
        &self,
        envelope: &Envelope,
    ) -> impl Iterator<Item = &GraphGeoElement> {
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

impl RTreeObject for GraphGeoElement {
    type Envelope = Envelope;
    fn envelope(&self) -> Self::Envelope {
        match self {
            GraphGeoElement::Node(_, node) => {
                let envelope = Point::<f64>::from(*node).envelope();
                // info!("node_envelope");
                return envelope;
            }
            GraphGeoElement::Edge(_, edge) => {
                let envelope = edge.envelope();
                // info!("edge_envelope: {},{}...{},{}", envelope.lower().x(), envelope.lower().y(), envelope.upper().x(), envelope.upper().y() );
                return envelope;
            }
        }
    }
}
